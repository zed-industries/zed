//! This module defines where the text should be displayed in an [`Editor`][Editor].
//!
//! Not literally though - rendering, layout and all that jazz is a responsibility of [`EditorElement`][EditorElement].
//! Instead, [`DisplayMap`] decides where Inlays/Inlay hints are displayed, when
//! to apply a soft wrap, where to add fold indicators, whether there are any tabs in the buffer that
//! we display as spaces and where to display custom blocks (like diagnostics).
//! Seems like a lot? That's because it is. [`DisplayMap`] is conceptually made up
//! of several smaller structures that form a hierarchy (starting at the bottom):
//! - [`InlayMap`] that decides where the [`Inlay`]s should be displayed.
//! - [`FoldMap`] that decides where the fold indicators should be; it also tracks parts of a source file that are currently folded.
//! - [`TabMap`] that keeps track of hard tabs in a buffer.
//! - [`WrapMap`] that handles soft wrapping.
//! - [`BlockMap`] that tracks custom blocks such as diagnostics that should be displayed within buffer.
//! - [`DisplayMap`] that adds background highlights to the regions of text.
//!   Each one of those builds on top of preceding map.
//!
//! ## Structure of the display map layers
//!
//! Each layer in the map (and the multibuffer itself to some extent) has a few
//! structures that are used to implement the public API available to the layer
//! above:
//! - a `Transform` type - this represents a region of text that the layer in
//!   question is "managing", that it transforms into a more "processed" text
//!   for the layer above. For example, the inlay map has an `enum Transform`
//!   that has two variants:
//!     - `Isomorphic`, representing a region of text that has no inlay hints (i.e.
//!       is passed through the map transparently)
//!     - `Inlay`, representing a location where an inlay hint is to be inserted.
//! - a `TransformSummary` type, which is usually a struct with two fields:
//!   [`input: TextSummary`][`TextSummary`] and [`output: TextSummary`][`TextSummary`]. Here,
//!   `input` corresponds to "text in the layer below", and `output` corresponds to the text
//!   exposed to the layer above. So in the inlay map case, a `Transform::Isomorphic`'s summary is
//!   just `input = output = summary`, where `summary` is the [`TextSummary`] stored in that
//!   variant. Conversely, a `Transform::Inlay` always has an empty `input` summary, because it's
//!   not "replacing" any text that exists on disk. The `output` is the summary of the inlay text
//!   to be injected. - Various newtype wrappers for co-ordinate spaces (e.g. [`WrapRow`]
//!   represents a row index, after soft-wrapping (and all lower layers)).
//! - A `Snapshot` type (e.g. [`InlaySnapshot`]) that captures the state of a layer at a specific
//!   point in time.
//! - various APIs which drill through the layers below to work with the underlying text. Notably:
//!   - `fn text_summary_for_offset()` returns a [`TextSummary`] for the range in the co-ordinate
//!     space that the map in question is responsible for.
//!   - `fn <A>_point_to_<B>_point()` converts a point in co-ordinate space `A` into co-ordinate
//!     space `B`.
//!   - A [`RowInfo`] iterator (e.g. [`InlayBufferRows`]) and a [`Chunk`] iterator
//!     (e.g. [`InlayChunks`])
//!   - A `sync` function (e.g. [`InlayMap::sync`]) that takes a snapshot and list of [`Edit<T>`]s,
//!     and returns a new snapshot and a list of transformed [`Edit<S>`]s. Note that the generic
//!     parameter on `Edit` changes, since these methods take in edits in the co-ordinate space of
//!     the lower layer, and return edits in their own co-ordinate space. The term "edit" is
//!     slightly misleading, since an [`Edit<T>`] doesn't tell you what changed - rather it can be
//!     thought of as a "region to invalidate". In theory, it would be correct to always use a
//!     single edit that covers the entire range. However, this would lead to lots of unnecessary
//!     recalculation.
//!
//! See the docs for the [`inlay_map`] module for a more in-depth explanation of how a single layer
//! works.
//!
//! [Editor]: crate::Editor
//! [EditorElement]: crate::element::EditorElement
//! [`TextSummary`]: multi_buffer::MBTextSummary
//! [`WrapRow`]: wrap_map::WrapRow
//! [`InlayBufferRows`]: inlay_map::InlayBufferRows
//! [`InlayChunks`]: inlay_map::InlayChunks
//! [`Edit<T>`]: text::Edit
//! [`Edit<S>`]: text::Edit
//! [`Chunk`]: language::Chunk

#[macro_use]
mod dimensions;

mod block_map;
mod crease_map;
mod custom_highlights;
mod fold_map;
mod inlay_map;
mod invisibles;
mod row_ruler;
mod tab_map;
mod wrap_map;

pub use crate::display_map::{fold_map::FoldMap, inlay_map::InlayMap, tab_map::TabMap};
pub use block_map::{
    Block, BlockContext, BlockId, BlockMap, BlockPlacement, BlockPoint, BlockProperties, BlockRows,
    BlockStyle, CompanionView, CompanionViewMut, CustomBlockId, EditorMargins, RenderBlock,
    StickyHeaderExcerpt,
};
pub use crease_map::*;
pub use fold_map::{
    ChunkRenderer, ChunkRendererContext, ChunkRendererId, Fold, FoldId, FoldPlaceholder, FoldPoint,
};
pub use inlay_map::{InlayOffset, InlayPoint};
use invisibles::is_standalone_grapheme;
pub use invisibles::{is_invisible, replacement};
pub use wrap_map::{WrapPoint, WrapRow, WrapSnapshot};

use collections::{HashMap, HashSet, IndexSet};
use gpui::{
    App, Context, Entity, EntityId, Font, FontId, HighlightStyle, Hsla, LineLayout, Pixels,
    TextAlign, UnderlineStyle, WeakEntity, WindowTextSystem,
};
use language::{
    LanguageAwareStyling, Point, Subscription as BufferSubscription,
    language_settings::{AllLanguageSettings, LanguageSettings, ShowWhitespaceSetting},
};

use multi_buffer::{
    Anchor, AnchorRangeExt, MultiBuffer, MultiBufferOffset, MultiBufferOffsetUtf16,
    MultiBufferPoint, MultiBufferRow, MultiBufferSnapshot, RowInfo, ToOffset, ToPoint,
};
use project::project_settings::DiagnosticSeverity;
use project::{InlayId, lsp_store::LspFoldingRange, lsp_store::TokenType};
use serde::Deserialize;
use settings::Settings;
use smallvec::SmallVec;
use sum_tree::{Bias, TreeMap};
use text::{BufferId, LineIndent, Patch};
use theme::StatusColors;
use ui::{SharedString, px};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};
use util::debug_panic;
use ztracing::instrument;

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::{
    any::TypeId,
    fmt::Debug,
    iter,
    num::NonZeroU32,
    ops::{self, Add, Range, RangeInclusive, Sub},
    sync::{Arc, LazyLock},
};

use crate::{
    EditorStyle, MAX_LINE_LEN, RowExt,
    hover_links::InlayHighlight,
    inlays::Inlay,
    movement::TextLayoutDetails,
    scroll::{ScrollOffset, ScrollPixelOffset},
};
use block_map::{BlockPointCursor, BlockRow, BlockSnapshot};
use fold_map::{Chunk, FoldPointCursor, FoldSnapshot};
use inlay_map::{BufferOffsetToInlayPointCursor, InlaySnapshot, inlay_chunk_renderer};
use itertools::Either;
use row_ruler::RowRulerCache;
pub use row_ruler::{RenderPiece, RowRuler, RulerShaper, renderer_metrics_key};
pub(crate) use tab_map::TabPoint;
use tab_map::{TabPointCursor, TabSnapshot};
use wrap_map::{WrapMap, WrapPatch, WrapPointCursor};

fn is_grid_byte(byte: u8) -> bool {
    (byte >= 0x20 && byte != 0x7f) || byte == b'\t' || byte == b'\n'
}

fn all_grid_bytes(bytes: &[u8]) -> bool {
    let found_control = bytes.iter().fold(false, |found_control, byte| {
        found_control | !is_grid_byte(*byte)
    });
    !found_control
}

const MAX_TRACKED_CONTROL_CHARS: usize = 1_024;
const MAX_TRACKED_RULER_DIRT: usize = 64;

#[derive(Default)]
struct RulerDirt {
    all: bool,
    ranges: Vec<Range<Anchor>>,
}

fn same_diagnostics(
    old_buffer: &MultiBufferSnapshot,
    old_range: Range<MultiBufferOffset>,
    buffer: &MultiBufferSnapshot,
    range: Range<MultiBufferOffset>,
) -> bool {
    let key = |base: MultiBufferOffset| {
        move |entry: language::DiagnosticEntryRef<'_, MultiBufferOffset>| {
            (
                entry.range.start.0.wrapping_sub(base.0),
                entry.range.end.0.wrapping_sub(base.0),
                entry.diagnostic.severity,
                entry.diagnostic.underline,
                entry.diagnostic.is_unnecessary,
            )
        }
    };
    old_buffer
        .diagnostics_in_range(old_range.clone())
        .map(key(old_range.start))
        .eq(buffer
            .diagnostics_in_range(range.clone())
            .map(key(range.start)))
}

fn same_syntax_highlights(
    old_buffer: &MultiBufferSnapshot,
    old_range: Range<MultiBufferOffset>,
    buffer: &MultiBufferSnapshot,
    range: Range<MultiBufferOffset>,
) -> bool {
    let old_ranges = old_buffer.range_to_buffer_ranges(old_range);
    let new_ranges = buffer.range_to_buffer_ranges(range);
    if old_ranges.len() != new_ranges.len() {
        return false;
    }
    old_ranges.iter().zip(&new_ranges).all(
        |((old_buffer, old_range, _), (new_buffer, new_range, _))| {
            old_buffer.remote_id() == new_buffer.remote_id()
                && (old_buffer.syntax_update_count() == new_buffer.syntax_update_count()
                    || same_highlight_captures(
                        old_buffer,
                        old_range.start.0..old_range.end.0,
                        new_buffer,
                        new_range.start.0..new_range.end.0,
                    ))
        },
    )
}

fn same_highlight_captures(
    old_buffer: &language::BufferSnapshot,
    old_range: Range<usize>,
    buffer: &language::BufferSnapshot,
    range: Range<usize>,
) -> bool {
    fn highlights_query(grammar: &language::Grammar) -> Option<&language::Query> {
        grammar
            .highlights_config
            .as_ref()
            .map(|config| &config.query)
    }
    let mut old_captures = old_buffer.captures(old_range.clone(), highlights_query);
    let mut new_captures = buffer.captures(range.clone(), highlights_query);
    let old_maps = old_captures
        .grammars()
        .iter()
        .map(|grammar| grammar.highlight_map())
        .collect::<Vec<_>>();
    let new_maps = new_captures
        .grammars()
        .iter()
        .map(|grammar| grammar.highlight_map())
        .collect::<Vec<_>>();
    loop {
        let (old, new) = match (old_captures.peek(), new_captures.peek()) {
            (Some(old), Some(new)) => (old, new),
            (None, None) => return true,
            _ => return false,
        };
        let old_highlight = old_maps
            .get(old.grammar_index)
            .and_then(|map| map.get(language::CaptureId(old.index)));
        let new_highlight = new_maps
            .get(new.grammar_index)
            .and_then(|map| map.get(language::CaptureId(new.index)));
        if old_highlight != new_highlight
            || old.node.start_byte().wrapping_sub(old_range.start)
                != new.node.start_byte().wrapping_sub(range.start)
            || old.node.end_byte().wrapping_sub(old_range.start)
                != new.node.end_byte().wrapping_sub(range.start)
        {
            return false;
        }
        old_captures.advance();
        new_captures.advance();
    }
}

fn anchor_hull(ranges: &[Range<Anchor>], buffer: &MultiBufferSnapshot) -> Option<Range<Anchor>> {
    let start = ranges
        .iter()
        .map(|range| range.start)
        .min_by(|a, b| a.cmp(b, buffer))?;
    let end = ranges
        .iter()
        .map(|range| range.end)
        .max_by(|a, b| a.cmp(b, buffer))?;
    Some(start..end)
}

fn wrap_row_for_offset(block_snapshot: &BlockSnapshot, offset: MultiBufferOffset) -> u32 {
    let wrap_snapshot = &block_snapshot.wrap_snapshot;
    let tab_snapshot = &wrap_snapshot.tab_snapshot;
    let fold_snapshot = &tab_snapshot.fold_snapshot;
    let inlay_snapshot = &fold_snapshot.inlay_snapshot;
    let inlay_point = inlay_snapshot.to_point(inlay_snapshot.to_inlay_offset(offset));
    let fold_point = fold_snapshot.to_fold_point(inlay_point, Bias::Left);
    let tab_point = tab_snapshot.fold_point_to_tab_point(fold_point);
    wrap_snapshot.tab_point_to_wrap_point(tab_point).row().0
}

fn row_buffer_range(block_snapshot: &BlockSnapshot, wrap_row: u32) -> Range<MultiBufferOffset> {
    let wrap_snapshot = &block_snapshot.wrap_snapshot;
    let tab_snapshot = &wrap_snapshot.tab_snapshot;
    let fold_snapshot = &tab_snapshot.fold_snapshot;
    let inlay_snapshot = &fold_snapshot.inlay_snapshot;
    let to_buffer = |wrap_point: WrapPoint, bias: Bias| {
        let tab_point = wrap_snapshot.to_tab_point(wrap_point);
        let fold_point = tab_snapshot.tab_point_to_fold_point(tab_point, bias).0;
        let inlay_point = fold_point.to_inlay_point(fold_snapshot);
        inlay_snapshot.to_buffer_offset(inlay_snapshot.to_offset(inlay_point))
    };
    let wrap_row = WrapRow(wrap_row);
    to_buffer(WrapPoint::new(wrap_row, 0), Bias::Left)
        ..to_buffer(
            WrapPoint::new(wrap_row, wrap_snapshot.line_len(wrap_row)),
            Bias::Right,
        )
}

#[derive(Clone, Debug)]
enum ControlChars {
    Tracked(Vec<Anchor>),
    Saturated(usize),
}

impl ControlChars {
    fn scan(buffer: &MultiBufferSnapshot) -> Self {
        let count = count_control_bytes(buffer, MultiBufferOffset(0)..buffer.len());
        if count > MAX_TRACKED_CONTROL_CHARS {
            return Self::Saturated(count);
        }
        let mut anchors = Vec::with_capacity(count);
        collect_control_anchors(buffer, MultiBufferOffset(0)..buffer.len(), &mut anchors);
        Self::Tracked(anchors)
    }

    fn edited(
        &self,
        old_buffer: &MultiBufferSnapshot,
        buffer: &MultiBufferSnapshot,
        edits: &[text::Edit<MultiBufferOffset>],
    ) -> Self {
        match self {
            Self::Saturated(count) => {
                let removed = edits
                    .iter()
                    .map(|edit| count_control_bytes(old_buffer, edit.old.clone()))
                    .sum::<usize>();
                let added = edits
                    .iter()
                    .map(|edit| count_control_bytes(buffer, edit.new.clone()))
                    .sum::<usize>();
                let count = count.saturating_sub(removed) + added;
                if count <= MAX_TRACKED_CONTROL_CHARS / 2 {
                    Self::scan(buffer)
                } else {
                    Self::Saturated(count)
                }
            }
            Self::Tracked(anchors) => {
                let mut anchors = anchors
                    .iter()
                    .filter(|anchor| {
                        let offset = anchor.to_offset(buffer);
                        !edits
                            .iter()
                            .any(|edit| edit.new.start <= offset && offset <= edit.new.end)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                for edit in edits {
                    let rescan_end = MultiBufferOffset(edit.new.end.0 + 1).min(buffer.len());
                    collect_control_anchors(buffer, edit.new.start..rescan_end, &mut anchors);
                }
                if anchors.len() > MAX_TRACKED_CONTROL_CHARS {
                    return Self::Saturated(anchors.len());
                }
                anchors.sort_unstable_by(|a, b| a.cmp(b, buffer));
                anchors.dedup_by(|a, b| a.cmp(b, buffer).is_eq());
                Self::Tracked(anchors)
            }
        }
    }

    fn intersects(&self, buffer: &MultiBufferSnapshot, range: Range<MultiBufferOffset>) -> bool {
        match self {
            Self::Saturated(_) => true,
            Self::Tracked(anchors) => {
                let first =
                    anchors.partition_point(|anchor| anchor.to_offset(buffer) < range.start);
                anchors
                    .get(first)
                    .is_some_and(|anchor| anchor.to_offset(buffer) < range.end)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct DiagnosticState {
    underline: Option<UnderlineStyle>,
    severity: Option<lsp::DiagnosticSeverity>,
}

impl DiagnosticState {
    fn observe(
        &mut self,
        severity: Option<lsp::DiagnosticSeverity>,
        underline: bool,
        is_unnecessary: bool,
        snapshot: &DisplaySnapshot,
        editor_style: &EditorStyle,
    ) -> Option<HighlightStyle> {
        let highlight =
            snapshot.observe_diagnostic_chunk(severity, underline, is_unnecessary, editor_style);
        self.underline = highlight.as_ref().and_then(|highlight| highlight.underline);
        self.severity = self
            .underline
            .and_then(|_| severity)
            .filter(|severity| snapshot.diagnostic_severity_is_visible(*severity));
        highlight
    }
}

fn count_control_bytes(buffer: &MultiBufferSnapshot, range: Range<MultiBufferOffset>) -> usize {
    buffer
        .bytes_in_range(range)
        .map(|bytes| bytes.iter().filter(|byte| !is_grid_byte(**byte)).count())
        .sum()
}

fn collect_control_anchors(
    buffer: &MultiBufferSnapshot,
    range: Range<MultiBufferOffset>,
    anchors: &mut Vec<Anchor>,
) {
    let mut offset = range.start;
    for bytes in buffer.bytes_in_range(range) {
        for (ix, byte) in bytes.iter().enumerate() {
            if !is_grid_byte(*byte) {
                anchors.push(buffer.anchor_before(MultiBufferOffset(offset.0 + ix)));
            }
        }
        offset.0 += bytes.len();
    }
}

const BULLETS: &str = match std::str::from_utf8(&[b'*'; rope::Chunk::MASK_BITS]) {
    Ok(bullets) => bullets,
    Err(_) => panic!("BULLETS must be ASCII"),
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FoldStatus {
    Folded,
    Foldable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NavigationOverlayKey(TypeId);

impl NavigationOverlayKey {
    pub const fn unique<T: 'static>() -> Self {
        Self(TypeId::of::<T>())
    }
}

/// Keys for tagging text highlights.
///
/// Note the order is important as it determines the priority of the highlights, lower means higher priority
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HighlightKey {
    // Note we want semantic tokens > colorized brackets
    // to allow language server highlights to work over brackets.
    ColorizeBracket(usize),
    SemanticToken(u32),
    // below is sorted lexicographically, as there is no relevant ordering for these aside from coming after the above
    BufferSearchHighlights,
    ConsoleAnsiHighlight(usize),
    DebugStackFrameLine,
    DocumentHighlightRead,
    DocumentHighlightWrite,
    EditPredictionHighlight,
    Editor,
    HighlightOnYank,
    HighlightsTreeView(usize),
    HoverState,
    HoveredLinkState,
    InlineAssist,
    InputComposition,
    MatchingBracket,
    NavigationOverlay(NavigationOverlayKey),
    PendingInput,
    PickerPreview,
    ProjectSearchView,
    Rename,
    SearchWithinRange,
    SelectedTextHighlight,
    SyntaxTreeView(usize),
    VimExchange,
}

pub trait ToDisplayPoint {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint;
}

type TextHighlights = Arc<HashMap<HighlightKey, Arc<(HighlightStyle, Vec<Range<Anchor>>)>>>;
type SemanticTokensHighlights =
    Arc<HashMap<BufferId, (Arc<[SemanticTokenHighlight]>, Arc<HighlightStyleInterner>)>>;
type InlayHighlights = TreeMap<HighlightKey, TreeMap<InlayId, (HighlightStyle, InlayHighlight)>>;

#[derive(Debug)]
pub struct CompanionExcerptPatch {
    pub patch: Patch<MultiBufferPoint>,
    pub edited_range: Range<MultiBufferPoint>,
    pub source_excerpt_range: Range<MultiBufferPoint>,
    pub target_excerpt_range: Range<MultiBufferPoint>,
}

/// Decides how text in a [`MultiBuffer`] should be displayed in a buffer, handling inlay hints,
/// folding, hard tabs, soft wrapping, custom blocks (like diagnostics), and highlighting.
///
/// See the [module level documentation](self) for more information.
pub struct DisplayMap {
    entity_id: EntityId,
    /// The buffer that we are displaying.
    buffer: Entity<MultiBuffer>,
    buffer_subscription: BufferSubscription<MultiBufferOffset>,
    /// Decides where the [`Inlay`]s should be displayed.
    inlay_map: InlayMap,
    /// Decides where the fold indicators should be and tracks parts of a source file that are currently folded.
    fold_map: FoldMap,
    /// Keeps track of hard tabs in a buffer.
    tab_map: TabMap,
    /// Handles soft wrapping.
    wrap_map: Entity<WrapMap>,
    /// Tracks custom blocks such as diagnostics that should be displayed within buffer.
    block_map: BlockMap,
    /// Regions of text that should be highlighted.
    text_highlights: TextHighlights,
    /// Regions of inlays that should be highlighted.
    inlay_highlights: InlayHighlights,
    /// The semantic tokens from the language server.
    pub semantic_token_highlights: SemanticTokensHighlights,
    /// A container for explicitly foldable ranges, which supersede indentation based fold range suggestions.
    crease_map: CreaseMap,
    pub(crate) fold_placeholder: FoldPlaceholder,
    pub clip_at_line_ends: bool,
    pub(crate) masked: bool,
    pub(crate) diagnostics_max_severity: DiagnosticSeverity,
    pub(crate) companion: Option<(WeakEntity<DisplayMap>, Entity<Companion>)>,
    lsp_folding_crease_ids: HashMap<BufferId, Vec<CreaseId>>,
    row_rulers: Arc<RowRulerCache>,
    control_chars: Arc<ControlChars>,
    highlight_version: usize,
    ruler_dirt: RulerDirt,
    ruler_old_buffer: Option<MultiBufferSnapshot>,
}

pub(crate) struct Companion {
    rhs_display_map_id: EntityId,
    rhs_custom_block_to_balancing_block: RefCell<HashMap<CustomBlockId, CustomBlockId>>,
    lhs_custom_block_to_balancing_block: RefCell<HashMap<CustomBlockId, CustomBlockId>>,
}

impl Companion {
    pub(crate) fn new(rhs_display_map_id: EntityId) -> Self {
        Self {
            rhs_display_map_id,
            rhs_custom_block_to_balancing_block: Default::default(),
            lhs_custom_block_to_balancing_block: Default::default(),
        }
    }

    pub(crate) fn is_rhs(&self, display_map_id: EntityId) -> bool {
        self.rhs_display_map_id == display_map_id
    }

    pub(crate) fn custom_block_to_balancing_block(
        &self,
        display_map_id: EntityId,
    ) -> &RefCell<HashMap<CustomBlockId, CustomBlockId>> {
        if self.is_rhs(display_map_id) {
            &self.rhs_custom_block_to_balancing_block
        } else {
            &self.lhs_custom_block_to_balancing_block
        }
    }

    pub(crate) fn convert_rows_to_companion(
        &self,
        display_map_id: EntityId,
        companion_snapshot: &MultiBufferSnapshot,
        our_snapshot: &MultiBufferSnapshot,
        bounds: Range<MultiBufferPoint>,
    ) -> Vec<CompanionExcerptPatch> {
        if self.is_rhs(display_map_id) {
            crate::split::patches_for_rhs_range(companion_snapshot, our_snapshot, bounds)
        } else {
            crate::split::patches_for_lhs_range(companion_snapshot, our_snapshot, bounds)
        }
    }

    pub(crate) fn convert_point_from_companion(
        &self,
        display_map_id: EntityId,
        our_snapshot: &MultiBufferSnapshot,
        companion_snapshot: &MultiBufferSnapshot,
        point: MultiBufferPoint,
    ) -> Range<MultiBufferPoint> {
        let patches = if self.is_rhs(display_map_id) {
            crate::split::patches_for_lhs_range(our_snapshot, companion_snapshot, point..point)
        } else {
            crate::split::patches_for_rhs_range(our_snapshot, companion_snapshot, point..point)
        };

        let Some(excerpt) = patches.into_iter().next() else {
            if cfg!(any(test, debug_assertions)) {
                assert!(
                    our_snapshot.max_point() == Point::zero(),
                    "`patches_for_*_in_range` is only allowed to return an empty vec if the multibuffer is empty"
                );
            }
            return Point::zero()..our_snapshot.max_point();
        };
        excerpt.patch.edit_for_old_position(point).new
    }

    pub(crate) fn convert_point_to_companion(
        &self,
        display_map_id: EntityId,
        our_snapshot: &MultiBufferSnapshot,
        companion_snapshot: &MultiBufferSnapshot,
        point: MultiBufferPoint,
    ) -> Range<MultiBufferPoint> {
        let patches = if self.is_rhs(display_map_id) {
            crate::split::patches_for_rhs_range(companion_snapshot, our_snapshot, point..point)
        } else {
            crate::split::patches_for_lhs_range(companion_snapshot, our_snapshot, point..point)
        };

        let Some(excerpt) = patches.into_iter().next() else {
            return Point::zero()..companion_snapshot.max_point();
        };
        excerpt.patch.edit_for_old_position(point).new
    }
}

#[derive(Default, Debug)]
pub struct HighlightStyleInterner {
    styles: IndexSet<HighlightStyle>,
}

impl HighlightStyleInterner {
    pub(crate) fn intern(&mut self, style: HighlightStyle) -> HighlightStyleId {
        HighlightStyleId(self.styles.insert_full(style).0 as u32)
    }

    pub(crate) fn styles(&self) -> impl Iterator<Item = &HighlightStyle> {
        self.styles.iter()
    }
}

fn highlight_styles<'a>(
    text_highlights: &'a TextHighlights,
    inlay_highlights: &'a InlayHighlights,
    semantic_token_highlights: &'a SemanticTokensHighlights,
) -> impl Iterator<Item = HighlightStyle> + 'a {
    text_highlights
        .values()
        .map(|highlight| highlight.0)
        .chain(
            inlay_highlights
                .iter()
                .flat_map(|(_, highlights)| highlights.iter().map(|(_, (style, _))| *style)),
        )
        .chain(
            semantic_token_highlights
                .values()
                .flat_map(|(_, interner)| interner.styles().copied()),
        )
}

impl ops::Index<HighlightStyleId> for HighlightStyleInterner {
    type Output = HighlightStyle;

    fn index(&self, index: HighlightStyleId) -> &Self::Output {
        &self.styles[index.0 as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HighlightStyleId(u32);

/// A `SemanticToken`, but positioned to an offset in a buffer, and stylized.
#[derive(Debug, Clone)]
pub struct SemanticTokenHighlight {
    pub range: Range<Anchor>,
    pub style: HighlightStyleId,
    pub token_type: TokenType,
    pub token_modifiers: u32,
    pub server_id: lsp::LanguageServerId,
    pub precedence: u32,
}

impl DisplayMap {
    pub fn new(
        buffer: Entity<MultiBuffer>,
        font: Font,
        font_size: Pixels,
        wrap_width: Option<Pixels>,
        buffer_header_height: u32,
        excerpt_header_height: u32,
        fold_placeholder: FoldPlaceholder,
        diagnostics_max_severity: DiagnosticSeverity,
        cx: &mut Context<Self>,
    ) -> Self {
        let tab_size = Self::tab_size(&buffer, cx);
        // Important: obtain the snapshot BEFORE creating the subscription.
        // snapshot() may call sync() which publishes edits. If we subscribe first,
        // those edits would be captured but the InlayMap would already be at the
        // post-edit state, causing a desync.
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let buffer_subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let control_chars = Arc::new(ControlChars::scan(&buffer_snapshot));
        let crease_map = CreaseMap::new(&buffer_snapshot);
        let (inlay_map, snapshot) = InlayMap::new(buffer_snapshot);
        let (fold_map, snapshot) = FoldMap::new(snapshot);
        let (tab_map, snapshot) = TabMap::new(snapshot, tab_size);
        let (wrap_map, snapshot) = WrapMap::new(snapshot, font, font_size, wrap_width, cx);
        let block_map = BlockMap::new(snapshot, buffer_header_height, excerpt_header_height);

        cx.observe(&wrap_map, |_, _, cx| cx.notify()).detach();

        DisplayMap {
            entity_id: cx.entity_id(),
            buffer,
            buffer_subscription,
            fold_map,
            inlay_map,
            tab_map,
            wrap_map,
            block_map,
            crease_map,
            fold_placeholder,
            diagnostics_max_severity,
            text_highlights: Default::default(),
            inlay_highlights: Default::default(),
            semantic_token_highlights: Default::default(),
            clip_at_line_ends: false,
            masked: false,
            companion: None,
            lsp_folding_crease_ids: HashMap::default(),
            row_rulers: Arc::new(RowRulerCache::new(
                (0, 0, 0, diagnostics_max_severity, tab_size),
                false,
                None,
                |_| None,
            )),
            control_chars,
            highlight_version: 0,
            ruler_dirt: RulerDirt::default(),
            ruler_old_buffer: None,
        }
    }

    fn consume_buffer_edits(
        &mut self,
        buffer: &MultiBufferSnapshot,
    ) -> Vec<text::Edit<MultiBufferOffset>> {
        let edits = self.buffer_subscription.consume().into_inner();
        if self.ruler_old_buffer.is_none() {
            self.ruler_old_buffer = Some(self.inlay_map.buffer_snapshot().clone());
        }
        let dirty_edits = edits
            .iter()
            .map(|edit| buffer.anchor_before(edit.new.start)..buffer.anchor_after(edit.new.end))
            .collect::<Vec<_>>();
        self.mark_rulers_dirty(dirty_edits);
        let tracking_none =
            matches!(&*self.control_chars, ControlChars::Tracked(anchors) if anchors.is_empty());
        if !edits.is_empty()
            && (!tracking_none
                || edits
                    .iter()
                    .any(|edit| !buffer.bytes_in_range(edit.new.clone()).all(all_grid_bytes)))
        {
            self.control_chars = Arc::new(self.control_chars.edited(
                self.inlay_map.buffer_snapshot(),
                buffer,
                &edits,
            ));
        }
        edits
    }

    pub(crate) fn set_companion(
        &mut self,
        companion: Option<(Entity<DisplayMap>, Entity<Companion>)>,
        cx: &mut Context<Self>,
    ) {
        let this = cx.weak_entity();
        // Reverting to no companion, recompute the block map to clear spacers
        // and balancing blocks.
        let Some((companion_display_map, companion)) = companion else {
            let Some((_, companion)) = self.companion.take() else {
                return;
            };
            assert_eq!(self.entity_id, companion.read(cx).rhs_display_map_id);
            let (snapshot, _edits) = self.sync_through_wrap(cx);
            let edits = Patch::new(vec![text::Edit {
                old: WrapRow(0)
                    ..self.block_map.wrap_snapshot.borrow().max_point().row() + WrapRow(1),
                new: WrapRow(0)..snapshot.max_point().row() + WrapRow(1),
            }]);
            self.block_map.deferred_edits.set(edits);
            self.block_map.retain_blocks_raw(&mut |block| {
                if companion
                    .read(cx)
                    .lhs_custom_block_to_balancing_block
                    .borrow()
                    .values()
                    .any(|id| *id == block.id)
                {
                    return false;
                }
                true
            });
            return;
        };
        assert_eq!(self.entity_id, companion.read(cx).rhs_display_map_id);

        // Note, throwing away the wrap edits because we defer spacer computation to the first render.
        let snapshot = {
            let snapshot = self.buffer.read(cx).snapshot(cx);
            let edits = self.consume_buffer_edits(&snapshot);
            let tab_size = Self::tab_size(&self.buffer, cx);
            let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
            let (mut writer, snapshot, edits) = self.fold_map.write(snapshot, edits);
            let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
            let (_snapshot, _edits) = self
                .wrap_map
                .update(cx, |wrap_map, cx| wrap_map.sync(snapshot, edits, cx));

            let (snapshot, edits) = writer.unfold_intersecting([Anchor::Min..Anchor::Max], true);
            let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
            let (snapshot, _edits) = self
                .wrap_map
                .update(cx, |wrap_map, cx| wrap_map.sync(snapshot, edits, cx));

            self.block_map.retain_blocks_raw(&mut |block| {
                !matches!(block.placement, BlockPlacement::Replace(_))
            });
            snapshot
        };

        let (companion_wrap_snapshot, _companion_wrap_edits) =
            companion_display_map.update(cx, |dm, cx| dm.sync_through_wrap(cx));

        let edits = Patch::new(vec![text::Edit {
            old: WrapRow(0)..self.block_map.wrap_snapshot.borrow().max_point().row() + WrapRow(1),
            new: WrapRow(0)..snapshot.max_point().row() + WrapRow(1),
        }]);
        self.block_map.deferred_edits.set(edits);

        let all_blocks: Vec<_> = self.block_map.blocks_raw().map(Clone::clone).collect();

        companion_display_map.update(cx, |companion_display_map, cx| {
            // Sync folded buffers from RHS to LHS. Also clean up stale
            // entries: the block map doesn't remove buffers from
            // `folded_buffers` when they leave the multibuffer, so we
            // unfold any RHS buffers whose companion mapping is missing.
            let rhs_snapshot = self.buffer.read(cx).snapshot(cx);
            let mut buffers_to_unfold = Vec::new();
            for my_buffer in self.folded_buffers() {
                let their_buffer = rhs_snapshot
                    .diff_for_buffer_id(*my_buffer)
                    .map(|diff| diff.base_text().remote_id());

                let Some(their_buffer) = their_buffer else {
                    buffers_to_unfold.push(*my_buffer);
                    continue;
                };

                companion_display_map
                    .block_map
                    .folded_buffers
                    .insert(their_buffer);
            }
            for buffer_id in buffers_to_unfold {
                self.block_map.folded_buffers.remove(&buffer_id);
            }

            for block in all_blocks {
                let Some(their_block) = block_map::balancing_block(
                    &block.properties(),
                    snapshot.buffer(),
                    companion_wrap_snapshot.buffer(),
                    self.entity_id,
                    companion.read(cx),
                ) else {
                    continue;
                };
                let their_id = companion_display_map
                    .block_map
                    .insert_block_raw(their_block, companion_wrap_snapshot.buffer());
                companion.update(cx, |companion, _cx| {
                    companion
                        .custom_block_to_balancing_block(self.entity_id)
                        .borrow_mut()
                        .insert(block.id, their_id);
                });
            }
            let companion_edits = Patch::new(vec![text::Edit {
                old: WrapRow(0)
                    ..companion_display_map
                        .block_map
                        .wrap_snapshot
                        .borrow()
                        .max_point()
                        .row()
                        + WrapRow(1),
                new: WrapRow(0)..companion_wrap_snapshot.max_point().row() + WrapRow(1),
            }]);
            companion_display_map
                .block_map
                .deferred_edits
                .set(companion_edits);
            companion_display_map.companion = Some((this, companion.clone()));
        });

        self.companion = Some((companion_display_map.downgrade(), companion));
    }

    pub(crate) fn companion(&self) -> Option<&Entity<Companion>> {
        self.companion.as_ref().map(|(_, c)| c)
    }

    fn sync_through_wrap(&mut self, cx: &mut App) -> (WrapSnapshot, WrapPatch) {
        let tab_size = Self::tab_size(&self.buffer, cx);
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.consume_buffer_edits(&buffer_snapshot);

        let (snapshot, edits) = self.inlay_map.sync(buffer_snapshot, edits);
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        self.wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx))
    }

    fn with_synced_companion_mut<R>(
        display_map_id: EntityId,
        companion: &Option<(WeakEntity<DisplayMap>, Entity<Companion>)>,
        cx: &mut App,
        callback: impl FnOnce(Option<CompanionViewMut<'_>>, &mut App) -> R,
    ) -> R {
        let Some((companion_display_map, companion)) = companion else {
            return callback(None, cx);
        };
        let Some(companion_display_map) = companion_display_map.upgrade() else {
            return callback(None, cx);
        };
        companion_display_map.update(cx, |companion_display_map, cx| {
            let (companion_wrap_snapshot, companion_wrap_edits) =
                companion_display_map.sync_through_wrap(cx);
            companion_display_map
                .buffer
                .update(cx, |companion_multibuffer, cx| {
                    companion.update(cx, |companion, cx| {
                        let companion_view = CompanionViewMut::new(
                            display_map_id,
                            companion_display_map.entity_id,
                            &companion_wrap_snapshot,
                            &companion_wrap_edits,
                            companion_multibuffer,
                            companion,
                            &mut companion_display_map.block_map,
                        );
                        callback(Some(companion_view), cx)
                    })
                })
        })
    }

    #[instrument(skip_all)]
    pub fn snapshot(&mut self, cx: &mut Context<Self>) -> DisplaySnapshot {
        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);
        let companion_wrap_data = self.companion.as_ref().and_then(|(companion_dm, _)| {
            companion_dm
                .update(cx, |dm, cx| dm.sync_through_wrap(cx))
                .ok()
        });
        let companion_ref = self.companion.as_ref().map(|(_, c)| c.read(cx));
        let companion_view = companion_wrap_data.as_ref().zip(companion_ref).map(
            |((snapshot, edits), companion)| {
                CompanionView::new(self.entity_id, snapshot, edits, companion)
            },
        );

        let block_snapshot = self
            .block_map
            .read(
                self_wrap_snapshot.clone(),
                self_wrap_edits.clone(),
                companion_view,
            )
            .snapshot;

        if let Some((companion_dm, _)) = &self.companion {
            let _ = companion_dm.update(cx, |dm, _cx| {
                if let Some((companion_snapshot, companion_edits)) = companion_wrap_data {
                    let their_companion_ref = dm.companion.as_ref().map(|(_, c)| c.read(_cx));
                    dm.block_map.read(
                        companion_snapshot,
                        companion_edits,
                        their_companion_ref.map(|c| {
                            CompanionView::new(
                                dm.entity_id,
                                &self_wrap_snapshot,
                                &self_wrap_edits,
                                c,
                            )
                        }),
                    );
                }
            });
        }

        let companion_display_snapshot = self.companion.as_ref().and_then(|(companion_dm, _)| {
            companion_dm
                .update(cx, |dm, cx| Arc::new(dm.snapshot_simple(cx)))
                .ok()
        });

        DisplaySnapshot {
            display_map_id: self.entity_id,
            companion_display_snapshot,
            row_rulers: self.row_rulers_for(&block_snapshot),
            control_chars: self.control_chars.clone(),
            block_snapshot,
            diagnostics_max_severity: self.diagnostics_max_severity,
            crease_snapshot: self.crease_map.snapshot(),
            text_highlights: self.text_highlights.clone(),
            inlay_highlights: self.inlay_highlights.clone(),
            semantic_token_highlights: self.semantic_token_highlights.clone(),
            clip_at_line_ends: self.clip_at_line_ends,
            masked: self.masked,
            use_lsp_folding_ranges: !self.lsp_folding_crease_ids.is_empty(),
            fold_placeholder: self.fold_placeholder.clone(),
        }
    }

    fn snapshot_simple(&mut self, cx: &mut Context<Self>) -> DisplaySnapshot {
        let (wrap_snapshot, wrap_edits) = self.sync_through_wrap(cx);

        let block_snapshot = self
            .block_map
            .read(wrap_snapshot, wrap_edits, None)
            .snapshot;

        DisplaySnapshot {
            display_map_id: self.entity_id,
            companion_display_snapshot: None,
            row_rulers: self.row_rulers_for(&block_snapshot),
            control_chars: self.control_chars.clone(),
            block_snapshot,
            diagnostics_max_severity: self.diagnostics_max_severity,
            crease_snapshot: self.crease_map.snapshot(),
            text_highlights: self.text_highlights.clone(),
            inlay_highlights: self.inlay_highlights.clone(),
            semantic_token_highlights: self.semantic_token_highlights.clone(),
            clip_at_line_ends: self.clip_at_line_ends,
            masked: self.masked,
            use_lsp_folding_ranges: !self.lsp_folding_crease_ids.is_empty(),
            fold_placeholder: self.fold_placeholder.clone(),
        }
    }

    fn row_rulers_for(&mut self, block_snapshot: &BlockSnapshot) -> Arc<RowRulerCache> {
        let tab_snapshot = &block_snapshot.wrap_snapshot.tab_snapshot;
        let buffer = &tab_snapshot.fold_snapshot.inlay_snapshot.buffer;
        let version = (
            tab_snapshot.version,
            self.highlight_version,
            buffer.non_text_state_update_count(),
            self.diagnostics_max_severity,
            tab_snapshot.tab_size,
        );
        if self.row_rulers.matches(version, self.masked) {
            return self.row_rulers.clone();
        }
        let dirt = std::mem::take(&mut self.ruler_dirt);
        let old_buffer = self.ruler_old_buffer.take();
        let (previous_version, previous_masked) = self.row_rulers.version();
        let retain_any = !dirt.all
            && dirt.ranges.len() <= MAX_TRACKED_RULER_DIRT
            && previous_version.3 == version.3
            && previous_version.4 == version.4
            && previous_masked == self.masked;
        let dirty_offsets = dirt
            .ranges
            .iter()
            .map(|range| range.start.to_offset(buffer)..range.end.to_offset(buffer))
            .collect::<Vec<_>>();
        let diagnostics_changed = previous_version.2 != version.2;
        let retain = |ruler: &RowRuler| -> Option<u32> {
            if !retain_any {
                return None;
            }
            let row_range = ruler.row_range()?;
            let start = row_range.start.to_offset(buffer);
            let end = row_range.end.to_offset(buffer);
            if dirty_offsets.iter().any(|dirty| {
                if dirty.is_empty() {
                    start <= dirty.start && dirty.start <= end
                } else {
                    dirty.start < end && start < dirty.end
                }
            }) {
                return None;
            }
            let old_buffer = old_buffer.as_ref()?;
            let old_start = row_range.start.to_offset(old_buffer);
            let old_end = row_range.end.to_offset(old_buffer);
            if diagnostics_changed
                && !same_diagnostics(old_buffer, old_start..old_end, buffer, start..end)
            {
                return None;
            }
            if !same_syntax_highlights(old_buffer, old_start..old_end, buffer, start..end) {
                return None;
            }
            let wrap_row = wrap_row_for_offset(block_snapshot, start);
            (row_buffer_range(block_snapshot, wrap_row) == (start..end)).then_some(wrap_row)
        };
        self.row_rulers = Arc::new(RowRulerCache::new(
            version,
            self.masked,
            Some(&self.row_rulers),
            retain,
        ));
        self.row_rulers.clone()
    }

    fn mark_rulers_dirty(&mut self, ranges: impl IntoIterator<Item = Range<Anchor>>) {
        if self.ruler_dirt.all {
            return;
        }
        for range in ranges {
            if self.ruler_dirt.ranges.len() >= MAX_TRACKED_RULER_DIRT {
                self.mark_all_rulers_dirty();
                return;
            }
            self.ruler_dirt.ranges.push(range);
        }
    }

    fn mark_rulers_dirty_in<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        buffer: &MultiBufferSnapshot,
    ) {
        let ranges = ranges
            .into_iter()
            .map(|range| {
                buffer.anchor_before(range.start.to_offset(buffer))
                    ..buffer.anchor_after(range.end.to_offset(buffer))
            })
            .collect::<Vec<_>>();
        self.mark_rulers_dirty(ranges);
    }

    fn mark_all_rulers_dirty(&mut self) {
        self.ruler_dirt.all = true;
        self.ruler_dirt.ranges.clear();
    }

    pub fn crease_snapshot(&self) -> CreaseSnapshot {
        self.crease_map.snapshot()
    }

    #[instrument(skip_all)]
    pub fn set_state(&mut self, other: &DisplaySnapshot, cx: &mut Context<Self>) {
        self.fold(
            other
                .folds_in_range(MultiBufferOffset(0)..other.buffer_snapshot().len())
                .map(|fold| {
                    Crease::simple(
                        fold.range.to_offset(other.buffer_snapshot()),
                        fold.placeholder.clone(),
                    )
                })
                .collect(),
            cx,
        );
        for buffer_id in &other.block_snapshot.buffers_with_disabled_headers {
            self.disable_header_for_buffer(*buffer_id, cx);
        }
    }

    /// Creates folds for the given creases.
    #[instrument(skip_all)]
    pub fn fold<T: Clone + ToOffset>(&mut self, creases: Vec<Crease<T>>, cx: &mut Context<Self>) {
        if self.companion().is_some() {
            return;
        }

        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.consume_buffer_edits(&buffer_snapshot);
        let tab_size = Self::tab_size(&self.buffer, cx);
        self.mark_rulers_dirty_in(
            creases.iter().map(|crease| crease.range().clone()),
            &buffer_snapshot,
        );

        let (snapshot, edits) = self.inlay_map.sync(buffer_snapshot.clone(), edits);
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits, None);

        let inline = creases.iter().filter_map(|crease| {
            if let Crease::Inline {
                range, placeholder, ..
            } = crease
            {
                Some((range.clone(), placeholder.clone()))
            } else {
                None
            }
        });
        let (snapshot, edits) = fold_map.fold(inline);

        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));

        let blocks = creases
            .into_iter()
            .filter_map(|crease| {
                if let Crease::Block {
                    range,
                    block_height,
                    render_block,
                    block_style,
                    block_priority,
                    ..
                } = crease
                {
                    Some((
                        range,
                        render_block,
                        block_height,
                        block_style,
                        block_priority,
                    ))
                } else {
                    None
                }
            })
            .map(|(range, render, height, style, priority)| {
                let start = buffer_snapshot.anchor_before(range.start);
                let end = buffer_snapshot.anchor_after(range.end);
                BlockProperties {
                    placement: BlockPlacement::Replace(start..=end),
                    render,
                    height: Some(height),
                    style,
                    priority,
                }
            });

        self.block_map.write(snapshot, edits, None).insert(blocks);
    }

    /// Removes any folds with the given ranges.
    #[instrument(skip_all)]
    pub fn remove_folds_with_type<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        type_id: TypeId,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let ranges = ranges
            .into_iter()
            .map(|range| range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot))
            .collect::<Vec<_>>();
        self.mark_rulers_dirty_in(ranges.iter().cloned(), &snapshot);
        let edits = self.consume_buffer_edits(&snapshot);
        let tab_size = Self::tab_size(&self.buffer, cx);

        let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits, None);

        let (snapshot, edits) = fold_map.remove_folds(ranges, type_id);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (self_new_wrap_snapshot, self_new_wrap_edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));

        self.block_map
            .write(self_new_wrap_snapshot, self_new_wrap_edits, None);
    }

    /// Removes any folds whose ranges intersect any of the given ranges.
    #[instrument(skip_all)]
    pub fn unfold_intersecting<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        inclusive: bool,
        cx: &mut Context<Self>,
    ) -> WrapSnapshot {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let offset_ranges = ranges
            .into_iter()
            .map(|range| range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot))
            .collect::<Vec<_>>();
        self.mark_rulers_dirty_in(offset_ranges.iter().cloned(), &snapshot);
        let edits = self.consume_buffer_edits(&snapshot);
        let tab_size = Self::tab_size(&self.buffer, cx);

        let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits, None);

        let (snapshot, edits) =
            fold_map.unfold_intersecting(offset_ranges.iter().cloned(), inclusive);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (self_new_wrap_snapshot, self_new_wrap_edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));

        self.block_map
            .write(self_new_wrap_snapshot.clone(), self_new_wrap_edits, None)
            .remove_intersecting_replace_blocks(offset_ranges, inclusive);

        self_new_wrap_snapshot
    }

    #[instrument(skip_all)]
    pub fn disable_header_for_buffer(&mut self, buffer_id: BufferId, cx: &mut Context<Self>) {
        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);
        self.block_map
            .write(self_wrap_snapshot, self_wrap_edits, None)
            .disable_header_for_buffer(buffer_id);
    }

    #[instrument(skip_all)]
    pub fn fold_buffers(
        &mut self,
        buffer_ids: impl IntoIterator<Item = language::BufferId>,
        cx: &mut App,
    ) {
        let buffer_ids: Vec<_> = buffer_ids.into_iter().collect();

        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);

        Self::with_synced_companion_mut(
            self.entity_id,
            &self.companion,
            cx,
            |companion_view, cx| {
                self.block_map
                    .write(
                        self_wrap_snapshot.clone(),
                        self_wrap_edits.clone(),
                        companion_view,
                    )
                    .fold_buffers(buffer_ids.iter().copied(), self.buffer.read(cx), cx);
            },
        )
    }

    #[instrument(skip_all)]
    pub fn unfold_buffers(
        &mut self,
        buffer_ids: impl IntoIterator<Item = language::BufferId>,
        cx: &mut Context<Self>,
    ) {
        let buffer_ids: Vec<_> = buffer_ids.into_iter().collect();

        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);

        Self::with_synced_companion_mut(
            self.entity_id,
            &self.companion,
            cx,
            |companion_view, cx| {
                self.block_map
                    .write(
                        self_wrap_snapshot.clone(),
                        self_wrap_edits.clone(),
                        companion_view,
                    )
                    .unfold_buffers(buffer_ids.iter().copied(), self.buffer.read(cx), cx);
            },
        )
    }

    #[instrument(skip_all)]
    pub(crate) fn is_buffer_folded(&self, buffer_id: language::BufferId) -> bool {
        self.block_map.folded_buffers.contains(&buffer_id)
    }

    #[instrument(skip_all)]
    pub(crate) fn folded_buffers(&self) -> &HashSet<BufferId> {
        &self.block_map.folded_buffers
    }

    #[instrument(skip_all)]
    pub fn insert_creases(
        &mut self,
        creases: impl IntoIterator<Item = Crease<Anchor>>,
        cx: &mut Context<Self>,
    ) -> Vec<CreaseId> {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        self.crease_map.insert(creases, &snapshot)
    }

    #[instrument(skip_all)]
    pub fn remove_creases(
        &mut self,
        crease_ids: impl IntoIterator<Item = CreaseId>,
        cx: &mut Context<Self>,
    ) -> Vec<(CreaseId, Range<Anchor>)> {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        self.crease_map.remove(crease_ids, &snapshot)
    }

    /// Replaces the LSP folding-range creases for a single buffer.
    /// Converts the supplied buffer-anchor ranges into multi-buffer creases
    /// by mapping them through the appropriate excerpts.
    pub(super) fn set_lsp_folding_ranges(
        &mut self,
        buffer_id: BufferId,
        ranges: Vec<LspFoldingRange>,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);

        let old_ids = self
            .lsp_folding_crease_ids
            .remove(&buffer_id)
            .unwrap_or_default();
        if !old_ids.is_empty() {
            self.crease_map.remove(old_ids, &snapshot);
        }

        if ranges.is_empty() {
            return;
        }

        let base_placeholder = self.fold_placeholder.clone();
        let creases = ranges.into_iter().filter_map(|folding_range| {
            let mb_range =
                snapshot.buffer_anchor_range_to_anchor_range(folding_range.range.clone())?;
            let placeholder = if let Some(collapsed_text) = folding_range.collapsed_text {
                FoldPlaceholder {
                    render: Arc::new({
                        let collapsed_text = collapsed_text.clone();
                        move |fold_id, _fold_range, cx: &mut gpui::App| {
                            use gpui::{Element as _, ParentElement as _};
                            FoldPlaceholder::fold_element(fold_id, cx)
                                .child(collapsed_text.clone())
                                .into_any()
                        }
                    }),
                    constrain_width: false,
                    merge_adjacent: base_placeholder.merge_adjacent,
                    type_tag: base_placeholder.type_tag,
                    collapsed_text: Some(collapsed_text),
                }
            } else {
                base_placeholder.clone()
            };
            Some(Crease::simple(mb_range, placeholder))
        });

        let new_ids = self.crease_map.insert(creases, &snapshot);
        if !new_ids.is_empty() {
            self.lsp_folding_crease_ids.insert(buffer_id, new_ids);
        }
    }

    /// Removes all LSP folding-range creases for a single buffer.
    pub(super) fn clear_lsp_folding_ranges(&mut self, buffer_id: BufferId, cx: &mut Context<Self>) {
        if let Some(old_ids) = self.lsp_folding_crease_ids.remove(&buffer_id) {
            let snapshot = self.buffer.read(cx).snapshot(cx);
            self.crease_map.remove(old_ids, &snapshot);
        }
    }

    /// Returns `true` when at least one buffer has LSP folding-range creases.
    pub(super) fn has_lsp_folding_ranges(&self) -> bool {
        !self.lsp_folding_crease_ids.is_empty()
    }

    #[instrument(skip_all)]
    pub fn insert_blocks(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
        cx: &mut Context<Self>,
    ) -> Vec<CustomBlockId> {
        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);
        Self::with_synced_companion_mut(
            self.entity_id,
            &self.companion,
            cx,
            |companion_view, _cx| {
                self.block_map
                    .write(
                        self_wrap_snapshot.clone(),
                        self_wrap_edits.clone(),
                        companion_view,
                    )
                    .insert(blocks)
            },
        )
    }

    #[instrument(skip_all)]
    pub fn resize_blocks(&mut self, heights: HashMap<CustomBlockId, u32>, cx: &mut Context<Self>) {
        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);

        Self::with_synced_companion_mut(
            self.entity_id,
            &self.companion,
            cx,
            |companion_view, _cx| {
                self.block_map
                    .write(
                        self_wrap_snapshot.clone(),
                        self_wrap_edits.clone(),
                        companion_view,
                    )
                    .resize(heights);
            },
        )
    }

    #[instrument(skip_all)]
    pub fn replace_blocks(&mut self, renderers: HashMap<CustomBlockId, RenderBlock>) {
        self.block_map.replace_blocks(renderers);
    }

    #[instrument(skip_all)]
    pub fn remove_blocks(&mut self, ids: HashSet<CustomBlockId>, cx: &mut Context<Self>) {
        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);

        Self::with_synced_companion_mut(
            self.entity_id,
            &self.companion,
            cx,
            |companion_view, _cx| {
                self.block_map
                    .write(
                        self_wrap_snapshot.clone(),
                        self_wrap_edits.clone(),
                        companion_view,
                    )
                    .remove(ids);
            },
        )
    }

    #[instrument(skip_all)]
    pub fn row_for_block(
        &mut self,
        block_id: CustomBlockId,
        cx: &mut Context<Self>,
    ) -> Option<DisplayRow> {
        let (self_wrap_snapshot, self_wrap_edits) = self.sync_through_wrap(cx);

        let companion_wrap_data = self.companion.as_ref().and_then(|(companion_dm, _)| {
            companion_dm
                .update(cx, |dm, cx| dm.sync_through_wrap(cx))
                .ok()
        });

        let companion_ref = self.companion.as_ref().map(|(_, c)| c.read(cx));
        let companion_view = companion_wrap_data.as_ref().zip(companion_ref).map(
            |((snapshot, edits), companion)| {
                CompanionView::new(self.entity_id, snapshot, edits, companion)
            },
        );

        let block_map = self.block_map.read(
            self_wrap_snapshot.clone(),
            self_wrap_edits.clone(),
            companion_view,
        );
        let block_row = block_map.row_for_block(block_id)?;

        if let Some((companion_dm, _)) = &self.companion {
            let _ = companion_dm.update(cx, |dm, cx| {
                if let Some((companion_snapshot, companion_edits)) = companion_wrap_data {
                    let their_companion_ref = dm.companion.as_ref().map(|(_, c)| c.read(cx));
                    dm.block_map.read(
                        companion_snapshot,
                        companion_edits,
                        their_companion_ref.map(|c| {
                            CompanionView::new(
                                dm.entity_id,
                                &self_wrap_snapshot,
                                &self_wrap_edits,
                                c,
                            )
                        }),
                    );
                }
            });
        }

        Some(DisplayRow(block_row.0))
    }

    #[instrument(skip_all)]
    pub fn highlight_text(
        &mut self,
        key: HighlightKey,
        mut ranges: Vec<Range<Anchor>>,
        style: HighlightStyle,
        merge: bool,
        cx: &App,
    ) {
        self.highlight_version += 1;
        let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let previous_hull = self
            .text_highlights
            .get(&key)
            .filter(|previous| !merge || previous.0 != style)
            .and_then(|previous| anchor_hull(&previous.1, &multi_buffer_snapshot));
        let dirty = previous_hull
            .into_iter()
            .chain(anchor_hull(&ranges, &multi_buffer_snapshot))
            .collect::<Vec<_>>();
        self.mark_rulers_dirty(dirty);
        match Arc::make_mut(&mut self.text_highlights).entry(key) {
            Entry::Occupied(mut slot) => match Arc::get_mut(slot.get_mut()) {
                Some((_, previous_ranges)) if merge => {
                    previous_ranges.extend(ranges);
                    previous_ranges.sort_by(|a, b| a.start.cmp(&b.start, &multi_buffer_snapshot));
                }
                Some((previous_style, previous_ranges)) => {
                    *previous_style = style;
                    *previous_ranges = ranges;
                    previous_ranges.sort_by(|a, b| a.start.cmp(&b.start, &multi_buffer_snapshot));
                }
                None if merge => {
                    ranges.extend(slot.get().1.iter().cloned());
                    ranges.sort_by(|a, b| a.start.cmp(&b.start, &multi_buffer_snapshot));
                    slot.insert(Arc::new((style, ranges)));
                }
                None => {
                    ranges.sort_by(|a, b| a.start.cmp(&b.start, &multi_buffer_snapshot));
                    slot.insert(Arc::new((style, ranges)));
                }
            },
            Entry::Vacant(slot) => {
                ranges.sort_by(|a, b| a.start.cmp(&b.start, &multi_buffer_snapshot));
                slot.insert(Arc::new((style, ranges)));
            }
        }
    }

    #[instrument(skip_all)]
    pub(crate) fn highlight_inlays(
        &mut self,
        key: HighlightKey,
        highlights: Vec<InlayHighlight>,
        style: HighlightStyle,
    ) {
        self.highlight_version += 1;
        let dirty = highlights
            .iter()
            .map(|highlight| highlight.inlay_position..highlight.inlay_position)
            .collect::<Vec<_>>();
        self.mark_rulers_dirty(dirty);
        for highlight in highlights {
            let update = self.inlay_highlights.update(&key, |highlights| {
                highlights.insert(highlight.inlay, (style, highlight.clone()))
            });
            if update.is_none() {
                self.inlay_highlights.insert(
                    key,
                    TreeMap::from_ordered_entries([(highlight.inlay, (style, highlight))]),
                );
            }
        }
    }

    #[instrument(skip_all)]
    pub fn text_highlights(&self, key: HighlightKey) -> Option<(HighlightStyle, &[Range<Anchor>])> {
        let highlights = self.text_highlights.get(&key)?;
        Some((highlights.0, &highlights.1))
    }

    pub fn all_text_highlights(
        &self,
    ) -> impl Iterator<Item = (&HighlightKey, &Arc<(HighlightStyle, Vec<Range<Anchor>>)>)> {
        self.text_highlights.iter()
    }

    pub fn all_semantic_token_highlights(
        &self,
    ) -> impl Iterator<
        Item = (
            &BufferId,
            &(Arc<[SemanticTokenHighlight]>, Arc<HighlightStyleInterner>),
        ),
    > {
        self.semantic_token_highlights.iter()
    }

    pub(crate) fn highlight_styles(&self) -> impl Iterator<Item = HighlightStyle> + '_ {
        highlight_styles(
            &self.text_highlights,
            &self.inlay_highlights,
            &self.semantic_token_highlights,
        )
    }

    pub fn clear_highlights(&mut self, key: HighlightKey) -> bool {
        let removed_text = self
            .text_highlights
            .contains_key(&key)
            .then(|| Arc::make_mut(&mut self.text_highlights).remove(&key))
            .flatten();
        let removed_inlays = self.inlay_highlights.remove(&key);
        let cleared = removed_text.is_some() || removed_inlays.is_some();
        if cleared {
            self.highlight_version += 1;
            self.mark_removed_highlights_dirty(removed_text.as_deref(), removed_inlays.as_ref());
        }
        cleared
    }

    pub fn clear_highlights_with(&mut self, f: &mut dyn FnMut(&HighlightKey) -> bool) -> bool {
        let mut cleared = false;
        let mut removed_text = Vec::new();
        let mut removed_inlays = Vec::new();
        Arc::make_mut(&mut self.text_highlights).retain(|k, highlights| {
            let b = !f(k);
            cleared |= b;
            if !b {
                removed_text.push(highlights.clone());
            }
            b
        });
        self.inlay_highlights.retain(|k, highlights| {
            let b = !f(k);
            cleared |= b;
            if !b {
                removed_inlays.push(highlights.clone());
            }
            b
        });
        if !removed_text.is_empty() || !removed_inlays.is_empty() {
            self.highlight_version += 1;
            for highlights in &removed_text {
                self.mark_removed_highlights_dirty(Some(highlights), None);
            }
            for highlights in &removed_inlays {
                self.mark_removed_highlights_dirty(None, Some(highlights));
            }
        }
        cleared
    }

    fn mark_removed_highlights_dirty(
        &mut self,
        text: Option<&(HighlightStyle, Vec<Range<Anchor>>)>,
        inlays: Option<&TreeMap<InlayId, (HighlightStyle, InlayHighlight)>>,
    ) {
        let dirty = text
            .into_iter()
            .flat_map(|(_, ranges)| ranges.iter().cloned())
            .chain(inlays.into_iter().flat_map(|inlays| {
                inlays
                    .iter()
                    .map(|(_, (_, highlight))| highlight.inlay_position..highlight.inlay_position)
            }))
            .collect::<Vec<_>>();
        self.mark_rulers_dirty(dirty);
    }

    pub fn set_font(&self, font: Font, font_size: Pixels, cx: &mut Context<Self>) -> bool {
        self.wrap_map
            .update(cx, |map, cx| map.set_font_with_size(font, font_size, cx))
    }

    pub fn set_wrap_width(&self, width: Option<Pixels>, cx: &mut Context<Self>) -> bool {
        self.wrap_map
            .update(cx, |map, cx| map.set_wrap_width(width, cx))
    }

    #[instrument(skip_all)]
    pub fn update_fold_widths(
        &mut self,
        widths: impl IntoIterator<Item = (ChunkRendererId, Pixels)>,
        renderer_metrics_key: u64,
        cx: &mut Context<Self>,
    ) -> bool {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.consume_buffer_edits(&snapshot);
        let tab_size = Self::tab_size(&self.buffer, cx);

        let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits, None);

        let widths = widths.into_iter().collect::<Vec<_>>();
        let ruler_widths_changed = self
            .row_rulers
            .update_renderer_widths(widths.iter().copied(), renderer_metrics_key);
        let (snapshot, edits) = fold_map.update_fold_widths(widths);
        if !edits.is_empty() {
            self.mark_all_rulers_dirty();
        }
        let widths_changed = ruler_widths_changed || !edits.is_empty();
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (self_new_wrap_snapshot, self_new_wrap_edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));

        self.block_map
            .read(self_new_wrap_snapshot, self_new_wrap_edits, None);

        widths_changed
    }

    pub(crate) fn current_inlays(&self) -> impl Iterator<Item = &Inlay> + Default {
        self.inlay_map.current_inlays()
    }

    #[instrument(skip_all)]
    pub(crate) fn splice_inlays(
        &mut self,
        to_remove: &[InlayId],
        to_insert: Vec<Inlay>,
        cx: &mut Context<Self>,
    ) {
        if to_remove.is_empty() && to_insert.is_empty() {
            return;
        }
        self.row_rulers.remove_renderer_widths(to_remove);
        let dirty = self
            .inlay_map
            .current_inlays()
            .filter(|inlay| to_remove.contains(&inlay.id))
            .chain(&to_insert)
            .map(|inlay| inlay.position..inlay.position)
            .collect::<Vec<_>>();
        self.mark_rulers_dirty(dirty);
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.consume_buffer_edits(&buffer_snapshot);
        let tab_size = Self::tab_size(&self.buffer, cx);

        let companion_wrap_data = self.companion.as_ref().and_then(|(companion_dm, _)| {
            companion_dm
                .update(cx, |dm, cx| dm.sync_through_wrap(cx))
                .ok()
        });

        let (snapshot, edits) = self.inlay_map.sync(buffer_snapshot, edits);
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));

        {
            let companion_ref = self.companion.as_ref().map(|(_, c)| c.read(cx));
            let companion_view = companion_wrap_data.as_ref().zip(companion_ref).map(
                |((snapshot, edits), companion)| {
                    CompanionView::new(self.entity_id, snapshot, edits, companion)
                },
            );
            self.block_map.read(snapshot, edits, companion_view);
        }

        let (snapshot, edits) = self.inlay_map.splice(to_remove, to_insert);
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (self_new_wrap_snapshot, self_new_wrap_edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));

        let (self_wrap_snapshot, self_wrap_edits) =
            (self_new_wrap_snapshot.clone(), self_new_wrap_edits.clone());

        {
            let companion_ref = self.companion.as_ref().map(|(_, c)| c.read(cx));
            let companion_view = companion_wrap_data.as_ref().zip(companion_ref).map(
                |((snapshot, edits), companion)| {
                    CompanionView::new(self.entity_id, snapshot, edits, companion)
                },
            );
            self.block_map
                .read(self_new_wrap_snapshot, self_new_wrap_edits, companion_view);
        }

        if let Some((companion_dm, _)) = &self.companion {
            let _ = companion_dm.update(cx, |dm, cx| {
                if let Some((companion_snapshot, companion_edits)) = companion_wrap_data {
                    let their_companion_ref = dm.companion.as_ref().map(|(_, c)| c.read(cx));
                    dm.block_map.read(
                        companion_snapshot,
                        companion_edits,
                        their_companion_ref.map(|c| {
                            CompanionView::new(
                                dm.entity_id,
                                &self_wrap_snapshot,
                                &self_wrap_edits,
                                c,
                            )
                        }),
                    );
                }
            });
        }
    }

    #[instrument(skip_all)]
    fn tab_size(buffer: &Entity<MultiBuffer>, cx: &App) -> NonZeroU32 {
        if let Some(buffer) = buffer.read(cx).as_singleton().map(|buffer| buffer.read(cx)) {
            LanguageSettings::for_buffer(buffer, cx).tab_size
        } else {
            AllLanguageSettings::get_global(cx).defaults.tab_size
        }
    }

    pub fn is_rewrapping(&self, cx: &gpui::App) -> bool {
        self.wrap_map.read(cx).is_rewrapping()
    }

    pub fn invalidate_semantic_highlights(&mut self, buffer_id: BufferId) {
        self.highlight_version += 1;
        self.mark_all_rulers_dirty();
        Arc::make_mut(&mut self.semantic_token_highlights).remove(&buffer_id);
    }

    pub(crate) fn clear_semantic_highlights(&mut self) {
        self.highlight_version += 1;
        self.mark_all_rulers_dirty();
        match Arc::get_mut(&mut self.semantic_token_highlights) {
            Some(highlights) => highlights.clear(),
            None => self.semantic_token_highlights = Arc::new(Default::default()),
        }
    }

    pub(crate) fn set_semantic_highlights(
        &mut self,
        buffer_id: BufferId,
        highlights: Arc<[SemanticTokenHighlight]>,
        interner: Arc<HighlightStyleInterner>,
    ) {
        self.highlight_version += 1;
        self.mark_all_rulers_dirty();
        Arc::make_mut(&mut self.semantic_token_highlights)
            .insert(buffer_id, (highlights, interner));
    }
}

#[derive(Debug, Default)]
pub(crate) struct Highlights<'a> {
    pub text_highlights: Option<&'a TextHighlights>,
    pub inlay_highlights: Option<&'a InlayHighlights>,
    pub semantic_token_highlights: Option<&'a SemanticTokensHighlights>,
    pub styles: HighlightStyles,
}

#[derive(Clone, Copy, Debug)]
pub struct EditPredictionStyles {
    pub insertion: HighlightStyle,
    pub whitespace: HighlightStyle,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct HighlightStyles {
    pub inlay_hint: Option<HighlightStyle>,
    pub edit_prediction: Option<EditPredictionStyles>,
}

#[derive(Clone)]
pub enum ChunkReplacement {
    Renderer(ChunkRenderer),
    Str(SharedString),
}

pub struct HighlightedChunk<'a> {
    pub text: &'a str,
    pub style: Option<HighlightStyle>,
    pub(crate) diagnostic_underline_severity: Option<lsp::DiagnosticSeverity>,
    pub is_tab: bool,
    pub is_inlay: bool,
    pub replacement: Option<ChunkReplacement>,
}

impl<'a> HighlightedChunk<'a> {
    #[instrument(skip_all)]
    fn highlight_invisibles(
        self,
        editor_style: &'a EditorStyle,
    ) -> impl Iterator<Item = Self> + 'a {
        let mut text = self.text;
        let style = self.style;
        let diagnostic_underline_severity = self.diagnostic_underline_severity;
        let is_tab = self.is_tab;
        let renderer = self.replacement;
        let is_inlay = self.is_inlay;
        iter::from_fn(move || {
            if text.is_empty() {
                return None;
            }
            for (offset, ch) in text.char_indices() {
                if !is_invisible(ch) {
                    continue;
                }
                let ch_end = offset + ch.len_utf8();
                if !is_standalone_grapheme(text, offset, ch_end) {
                    continue;
                }
                if offset > 0 {
                    let (prefix, suffix) = text.split_at(offset);
                    text = suffix;
                    return Some(HighlightedChunk {
                        text: prefix,
                        style,
                        diagnostic_underline_severity,
                        is_tab,
                        is_inlay,
                        replacement: renderer.clone(),
                    });
                }
                let (invisible_text, suffix) = text.split_at(ch_end);
                text = suffix;
                let invisible_highlight = HighlightStyle {
                    background_color: Some(editor_style.status.hint_background),
                    underline: Some(UnderlineStyle {
                        color: Some(editor_style.status.hint),
                        thickness: px(1.),
                        wavy: false,
                    }),
                    ..Default::default()
                };
                let invisible_style = if let Some(style) = style {
                    style.highlight(invisible_highlight)
                } else {
                    invisible_highlight
                };
                return Some(HighlightedChunk {
                    text: invisible_text,
                    style: Some(invisible_style),
                    diagnostic_underline_severity: None,
                    is_tab: false,
                    is_inlay,
                    replacement: match replacement(ch) {
                        Some(replacement) => {
                            Some(ChunkReplacement::Str(SharedString::from(replacement)))
                        }
                        None => renderer.clone(),
                    },
                });
            }
            let remainder = text;
            text = "";
            Some(HighlightedChunk {
                text: remainder,
                style,
                diagnostic_underline_severity,
                is_tab,
                is_inlay,
                replacement: renderer.clone(),
            })
        })
    }
}

fn mask_chunks<'a>(chunks: impl Iterator<Item = Chunk<'a>>) -> impl Iterator<Item = Chunk<'a>> {
    chunks.flat_map(|chunk| {
        let text = chunk.text;
        text.split_inclusive('\n').flat_map(move |segment| {
            let (content, has_newline) = match segment.strip_suffix('\n') {
                Some(content) => (content, true),
                None => (segment, false),
            };
            let content_chunks = std::iter::from_fn({
                let chunk = chunk.clone();
                let mut content = content;
                move || {
                    if content.is_empty() {
                        return None;
                    }
                    let (piece_len, bullet_count) = match content.char_indices().nth(BULLETS.len())
                    {
                        Some((ix, _)) => (ix, BULLETS.len()),
                        None => (content.len(), content.chars().count()),
                    };
                    content = &content[piece_len..];
                    Some(Chunk {
                        text: &BULLETS[..bullet_count],
                        tabs: 0,
                        chars: 1u128.unbounded_shl(bullet_count as u32).wrapping_sub(1),
                        newlines: 0,
                        ..chunk.clone()
                    })
                }
            });
            let newline_chunk = has_newline.then(|| Chunk {
                text: "\n",
                tabs: 0,
                chars: 1,
                newlines: 1,
                ..chunk.clone()
            });
            content_chunks.chain(newline_chunk)
        })
    })
}

static GRID_EXACT_FONTS: LazyLock<parking_lot::Mutex<HashMap<(FontId, u32, u32), bool>>> =
    LazyLock::new(parking_lot::Mutex::default);

fn font_is_grid_exact(
    text_system: &WindowTextSystem,
    font: &Font,
    font_size: Pixels,
    cell_width: Pixels,
) -> bool {
    let font_id = text_system.resolve_font(font);
    let key = (
        font_id,
        f32::from(font_size).to_bits(),
        f32::from(cell_width).to_bits(),
    );
    if let Some(exact) = GRID_EXACT_FONTS.lock().get(&key) {
        return *exact;
    }
    let exact = text_system.ascii_shaping_preserves_advances(font)
        && (0x20u8..=0x7E)
            .all(|byte| text_system.layout_width(font_id, font_size, byte as char) == cell_width);
    GRID_EXACT_FONTS.lock().insert(key, exact);
    exact
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridCell {
    pub width: Pixels,
    pub monospace: bool,
}

impl GridCell {
    pub(crate) const FIT_TOLERANCE: ScrollPixelOffset = 0.5;

    pub fn measure(
        text_system: &WindowTextSystem,
        style: &EditorStyle,
        highlight_styles: impl Iterator<Item = HighlightStyle>,
        font_size: Pixels,
    ) -> Self {
        let base = style.text.font();
        let font_id = text_system.resolve_font(&base);
        let width = text_system.em_layout_width(font_id, font_size);
        let mut variants = vec![(base.weight, base.style)];
        let styles = style
            .syntax
            .highlights()
            .copied()
            .chain([
                style.inlay_hints_style,
                style.edit_prediction_styles.insertion,
                style.edit_prediction_styles.whitespace,
            ])
            .chain(highlight_styles);
        for highlight in styles {
            let variant = (
                highlight.font_weight.unwrap_or(base.weight),
                highlight.font_style.unwrap_or(base.style),
            );
            if !variants.contains(&variant) {
                variants.push(variant);
            }
        }
        let monospace = width > Pixels::ZERO
            && variants.into_iter().all(|(weight, style)| {
                let font = Font {
                    weight,
                    style,
                    ..base.clone()
                };
                font_is_grid_exact(text_system, &font, font_size, width)
            });
        Self { width, monospace }
    }

    pub fn fits(&self, columns: &Range<u32>, shaped_width: Pixels) -> bool {
        let grid_width = ScrollPixelOffset::from(self.width) * columns.len() as ScrollPixelOffset;
        (ScrollPixelOffset::from(shaped_width) - grid_width).abs() <= Self::FIT_TOLERANCE
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HorizontalViewport {
    pub scroll_columns: ScrollOffset,
    pub visible_columns: ScrollOffset,
    pub text_align: TextAlign,
    pub content_width: Pixels,
}

impl HorizontalViewport {
    pub fn aligned(&self, line_width: ScrollPixelOffset, cell: GridCell) -> Self {
        let content_width = ScrollPixelOffset::from(self.content_width);
        let alignment_offset = match self.text_align {
            TextAlign::Left => 0.,
            TextAlign::Center => (content_width - line_width) / 2.,
            TextAlign::Right => content_width - line_width,
        };
        if alignment_offset == 0. || cell.width <= Pixels::ZERO {
            return *self;
        }
        Self {
            scroll_columns: self.scroll_columns
                - alignment_offset / ScrollPixelOffset::from(cell.width),
            ..*self
        }
    }

    pub fn first_column(&self) -> usize {
        self.scroll_columns.max(0.).floor() as usize
    }

    pub fn column_count(&self) -> usize {
        (self.visible_columns.max(0.).ceil() as usize).max(1)
    }

    pub fn shaping_window(&self, row_len: u32) -> Range<u32> {
        let visible = self.column_count();
        let leading = visible / 2;
        let total = leading + visible * 2;
        let row_len = row_len as usize;
        let start = (self.first_column().saturating_sub(leading) / visible * visible)
            .min(row_len.saturating_sub(total));
        let end = (start + total).min(row_len);
        column_from_usize(start)..column_from_usize(end)
    }
}

#[derive(Clone)]
pub struct RuledRow {
    snapshot: Arc<DisplaySnapshot>,
    row: DisplayRow,
    ruler: Arc<RowRuler>,
    shaper: RulerShaper,
}

impl Debug for RuledRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuledRow")
            .field("row", &self.row)
            .field("ruler", &self.ruler)
            .finish()
    }
}

impl RuledRow {
    pub fn columns_for_viewport(
        &self,
        viewport: &HorizontalViewport,
        cell: GridCell,
    ) -> Range<u32> {
        let viewport = viewport.aligned(self.ruler.width(), cell);
        let cell_width = ScrollPixelOffset::from(cell.width);
        let visible_width = viewport.visible_columns.max(1.) * cell_width;
        let left = viewport.scroll_columns.max(0.) * cell_width;
        self.ruler
            .columns_for_x_range(left - visible_width / 2.0..left + visible_width * 1.5)
    }

    pub fn render_pieces(&self, columns: Range<u32>) -> impl Iterator<Item = RenderPiece> + '_ {
        self.ruler.render_pieces(columns)
    }

    pub fn x_range_for_columns(&self, columns: Range<u32>) -> Range<ScrollPixelOffset> {
        self.ruler.x_range_for_columns(columns)
    }

    pub fn is_rtl(&self) -> bool {
        self.ruler.is_rtl()
    }

    fn x_for_column(&self, column: u32) -> ScrollPixelOffset {
        self.ruler
            .x_for_column(column, &self.snapshot, self.row, &self.shaper)
    }

    fn column_for_x(&self, x: ScrollPixelOffset) -> u32 {
        self.ruler
            .column_for_x(x, &self.snapshot, self.row, &self.shaper)
    }
}

#[derive(Clone, Debug)]
pub struct WindowedRowGeometry {
    row_len: u32,
    cell: GridCell,
    window: Range<u32>,
    ruled: Option<RuledRow>,
}

impl WindowedRowGeometry {
    pub fn new(row_len: u32, cell: GridCell, window: Range<u32>) -> Self {
        Self {
            row_len,
            cell,
            window,
            ruled: None,
        }
    }

    pub fn ruled(ruled: RuledRow, row_len: u32, cell: GridCell, window: Range<u32>) -> Self {
        Self {
            row_len,
            cell,
            window,
            ruled: Some(ruled),
        }
    }

    pub fn is_ruled(&self) -> bool {
        self.ruled.is_some()
    }

    pub fn row_len(&self) -> u32 {
        self.row_len
    }

    pub fn window(&self) -> &Range<u32> {
        &self.window
    }

    pub fn cell(&self) -> GridCell {
        self.cell
    }

    pub fn start_x(&self) -> ScrollPixelOffset {
        match &self.ruled {
            Some(ruled) => ruled.x_range_for_columns(self.window.clone()).start,
            None => self.column_x(self.window.start),
        }
    }

    pub fn end_x(&self) -> ScrollPixelOffset {
        match &self.ruled {
            Some(ruled) => ruled.x_range_for_columns(self.window.clone()).end,
            None => self.column_x(self.window.end),
        }
    }

    pub fn width(&self, shaped_width: Pixels) -> ScrollPixelOffset {
        let row_width = self.column_x(self.row_len);
        if self.ruled.is_some() {
            return row_width;
        }
        row_width.max(self.start_x() + ScrollPixelOffset::from(shaped_width))
    }

    pub fn column_x(&self, column: u32) -> ScrollPixelOffset {
        if let Some(ruled) = &self.ruled {
            return ruled.x_for_column(column.min(self.row_len));
        }
        ScrollPixelOffset::from(self.cell.width) * column as ScrollPixelOffset
    }

    pub fn column_for_x(&self, x: ScrollPixelOffset) -> u32 {
        if let Some(ruled) = &self.ruled {
            return ruled.column_for_x(x).min(self.row_len);
        }
        if self.cell.width <= Pixels::ZERO {
            return 0;
        }
        let column = (x / ScrollPixelOffset::from(self.cell.width))
            .round()
            .max(0.);
        column_from_usize(column as usize).min(self.row_len)
    }

    pub fn reaches_left_edge(&self) -> bool {
        if self.is_rtl() {
            self.window.end >= self.row_len
        } else {
            self.window.start == 0
        }
    }

    pub fn reaches_right_edge(&self) -> bool {
        if self.is_rtl() {
            self.window.start == 0
        } else {
            self.window.end >= self.row_len
        }
    }

    pub fn column_left_of_window_for_x(&self, x: ScrollPixelOffset) -> u32 {
        if self.is_rtl() {
            self.column_for_x(x).max(self.window.end)
        } else {
            self.column_for_x(x).min(self.window.start)
        }
    }

    pub fn column_right_of_window_for_x(&self, x: ScrollPixelOffset) -> u32 {
        if self.is_rtl() {
            self.column_for_x(x).min(self.window.start)
        } else {
            self.column_for_x(x).max(self.window.end)
        }
    }

    fn is_rtl(&self) -> bool {
        self.ruled.as_ref().is_some_and(RuledRow::is_rtl)
    }
}

fn column_from_usize(column: usize) -> u32 {
    u32::try_from(column).unwrap_or(u32::MAX)
}

pub enum RowLayout {
    Shaped(Arc<LineLayout>),
    Windowed {
        geometry: WindowedRowGeometry,
        shaped: Arc<LineLayout>,
    },
}

impl RowLayout {
    pub fn width(&self) -> ScrollPixelOffset {
        match self {
            Self::Shaped(layout) => ScrollPixelOffset::from(layout.width),
            Self::Windowed { geometry, shaped } => geometry.width(shaped.width),
        }
    }

    pub fn x_for_index(&self, index: usize) -> ScrollPixelOffset {
        match self {
            Self::Shaped(layout) => ScrollPixelOffset::from(layout.x_for_index(index)),
            Self::Windowed { geometry, shaped } => {
                let window = geometry.window();
                let column = column_from_usize(index);
                if !window.is_empty() && column >= window.start && column <= window.end {
                    geometry.start_x()
                        + ScrollPixelOffset::from(
                            shaped.x_for_index((column - window.start) as usize),
                        )
                } else {
                    geometry.column_x(column)
                }
            }
        }
    }

    pub fn closest_index_for_x(&self, x: ScrollPixelOffset) -> usize {
        match self {
            Self::Shaped(layout) => layout.closest_index_for_x(Pixels::from(x)),
            Self::Windowed { geometry, shaped } => {
                let start_x = geometry.start_x();
                let window = geometry.window();
                if x < start_x {
                    geometry.column_left_of_window_for_x(x) as usize
                } else if x <= start_x + ScrollPixelOffset::from(shaped.width) {
                    window.start as usize + shaped.closest_index_for_x(Pixels::from(x - start_x))
                } else {
                    geometry.column_right_of_window_for_x(x) as usize
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct DisplaySnapshot {
    pub display_map_id: EntityId,
    pub companion_display_snapshot: Option<Arc<DisplaySnapshot>>,
    pub crease_snapshot: CreaseSnapshot,
    block_snapshot: BlockSnapshot,
    row_rulers: Arc<RowRulerCache>,
    control_chars: Arc<ControlChars>,
    text_highlights: TextHighlights,
    inlay_highlights: InlayHighlights,
    semantic_token_highlights: SemanticTokensHighlights,
    clip_at_line_ends: bool,
    masked: bool,
    diagnostics_max_severity: DiagnosticSeverity,
    pub(crate) fold_placeholder: FoldPlaceholder,
    /// When true, LSP folding ranges are used via the crease map and the
    /// indent-based fallback in `crease_for_buffer_row` is skipped.
    pub(crate) use_lsp_folding_ranges: bool,
}

impl DisplaySnapshot {
    pub fn companion_snapshot(&self) -> Option<&DisplaySnapshot> {
        self.companion_display_snapshot.as_deref()
    }

    fn diagnostic_severity_is_visible(&self, severity: lsp::DiagnosticSeverity) -> bool {
        self.diagnostics_max_severity
            .into_lsp()
            .is_some_and(|max_severity| severity <= max_severity)
    }

    pub(crate) fn diagnostic_underline_style(
        &self,
        severity: lsp::DiagnosticSeverity,
        underline: bool,
        is_unnecessary: bool,
        editor_style: &EditorStyle,
    ) -> Option<UnderlineStyle> {
        (underline
            && editor_style.show_underlines
            && self.diagnostic_severity_is_visible(severity)
            && !(is_unnecessary && severity > lsp::DiagnosticSeverity::WARNING))
            .then(|| UnderlineStyle {
                color: Some(diagnostic_style(severity, &editor_style.status)),
                thickness: 1.0.into(),
                wavy: true,
            })
    }

    pub fn wrap_snapshot(&self) -> &WrapSnapshot {
        &self.block_snapshot.wrap_snapshot
    }

    pub(crate) fn highlight_styles(&self) -> impl Iterator<Item = HighlightStyle> + '_ {
        highlight_styles(
            &self.text_highlights,
            &self.inlay_highlights,
            &self.semantic_token_highlights,
        )
    }

    fn observe_diagnostic_chunk(
        &self,
        severity: Option<lsp::DiagnosticSeverity>,
        underline: bool,
        is_unnecessary: bool,
        editor_style: &EditorStyle,
    ) -> Option<HighlightStyle> {
        let severity = severity.filter(|severity| self.diagnostic_severity_is_visible(*severity));
        severity.map(|severity| HighlightStyle {
            fade_out: is_unnecessary.then_some(editor_style.unnecessary_code_fade),
            underline: self.diagnostic_underline_style(
                severity,
                underline,
                is_unnecessary,
                editor_style,
            ),
            ..Default::default()
        })
    }

    pub fn has_soft_wraps(&self) -> bool {
        self.wrap_snapshot().has_soft_wraps()
    }

    pub fn tab_snapshot(&self) -> &TabSnapshot {
        &self.block_snapshot.wrap_snapshot.tab_snapshot
    }

    pub fn fold_snapshot(&self) -> &FoldSnapshot {
        &self.block_snapshot.wrap_snapshot.tab_snapshot.fold_snapshot
    }

    #[inline(always)]
    pub fn has_collapsed_content(&self) -> bool {
        self.fold_snapshot().has_folds() || self.block_snapshot.has_replacement_blocks()
    }

    pub fn inlay_snapshot(&self) -> &InlaySnapshot {
        &self
            .block_snapshot
            .wrap_snapshot
            .tab_snapshot
            .fold_snapshot
            .inlay_snapshot
    }

    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        &self
            .block_snapshot
            .wrap_snapshot
            .tab_snapshot
            .fold_snapshot
            .inlay_snapshot
            .buffer
    }

    #[cfg(test)]
    pub fn fold_count(&self) -> usize {
        self.fold_snapshot().fold_count()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer_snapshot().len() == MultiBufferOffset(0)
    }

    /// Returns whether tree-sitter syntax highlighting should be used.
    /// Returns `false` if any buffer with semantic token highlights has the "full" mode setting,
    /// meaning LSP semantic tokens should replace tree-sitter highlighting.
    pub fn use_tree_sitter_for_syntax(&self, position: DisplayRow, cx: &App) -> bool {
        let position = DisplayPoint::new(position, 0);
        let Some((buffer_snapshot, ..)) = self.point_to_buffer_point(position.to_point(self))
        else {
            return false;
        };
        let settings = LanguageSettings::for_buffer_snapshot(&buffer_snapshot, None, cx);
        settings.semantic_tokens.use_tree_sitter()
    }

    pub fn shows_trailing_whitespace(&self, position: DisplayRow, cx: &App) -> bool {
        let position = DisplayPoint::new(position, 0);
        let Some((buffer_snapshot, ..)) = self.point_to_buffer_point(position.to_point(self))
        else {
            return false;
        };
        let settings = LanguageSettings::for_buffer_snapshot(&buffer_snapshot, None, cx);
        settings.show_whitespaces == ShowWhitespaceSetting::Trailing
    }

    pub fn row_infos(&self, start_row: DisplayRow) -> impl Iterator<Item = RowInfo> + '_ {
        self.block_snapshot.row_infos(BlockRow(start_row.0))
    }

    pub fn widest_line_number(&self) -> u32 {
        self.buffer_snapshot().widest_line_number()
    }

    #[instrument(skip_all)]
    pub fn prev_line_boundary(&self, mut point: MultiBufferPoint) -> (Point, DisplayPoint) {
        loop {
            let mut inlay_point = self.inlay_snapshot().to_inlay_point(point);
            let mut fold_point = self.fold_snapshot().to_fold_point(inlay_point, Bias::Left);
            fold_point.0.column = 0;
            inlay_point = fold_point.to_inlay_point(self.fold_snapshot());
            point = self.inlay_snapshot().to_buffer_point(inlay_point);

            let mut display_point = self.point_to_display_point(point, Bias::Left);
            *display_point.column_mut() = 0;
            let next_point = self.display_point_to_point(display_point, Bias::Left);
            if next_point == point {
                return (point, display_point);
            }
            point = next_point;
        }
    }

    #[instrument(skip_all)]
    pub fn next_line_boundary(
        &self,
        mut point: MultiBufferPoint,
    ) -> (MultiBufferPoint, DisplayPoint) {
        let original_point = point;
        loop {
            let mut inlay_point = self.inlay_snapshot().to_inlay_point(point);
            let mut fold_point = self.fold_snapshot().to_fold_point(inlay_point, Bias::Right);
            fold_point.0.column = self.fold_snapshot().line_len(fold_point.row());
            inlay_point = fold_point.to_inlay_point(self.fold_snapshot());
            point = self.inlay_snapshot().to_buffer_point(inlay_point);

            let mut display_point = self.point_to_display_point(point, Bias::Right);
            *display_point.column_mut() = self.line_len(display_point.row());
            let next_point = self.display_point_to_point(display_point, Bias::Right);
            if next_point == point || original_point == point || original_point == next_point {
                return (point, display_point);
            }
            point = next_point;
        }
    }

    // used by line_mode selections and tries to match vim behavior
    pub fn expand_to_line(&self, range: Range<Point>) -> Range<Point> {
        let new_start = MultiBufferPoint::new(range.start.row, 0);
        let new_end = if range.end.column > 0 {
            MultiBufferPoint::new(
                range.end.row,
                self.buffer_snapshot()
                    .line_len(MultiBufferRow(range.end.row)),
            )
        } else {
            range.end
        };

        new_start..new_end
    }

    #[instrument(skip_all)]
    pub fn point_to_display_point(&self, point: MultiBufferPoint, bias: Bias) -> DisplayPoint {
        let inlay_point = self.inlay_snapshot().to_inlay_point(point);
        let fold_point = self.fold_snapshot().to_fold_point(inlay_point, bias);
        let tab_point = self.tab_snapshot().fold_point_to_tab_point(fold_point);
        let wrap_point = self.wrap_snapshot().tab_point_to_wrap_point(tab_point);
        let block_point = self.block_snapshot.to_block_point(wrap_point);
        DisplayPoint(block_point)
    }

    /// Converts a buffer offset range into one or more `DisplayPoint` ranges
    /// that cover only actual buffer text, excluding any inlay hint text that
    /// falls within the range.
    pub fn isomorphic_display_point_ranges_for_buffer_range(
        &self,
        range: Range<MultiBufferOffset>,
    ) -> SmallVec<[Range<DisplayPoint>; 1]> {
        self.display_point_converter().map(range)
    }

    /// Converts a non-empty buffer range into one contiguous display range.
    /// Inlays at either boundary are excluded, while inlays between selected
    /// buffer characters are included.
    pub fn contiguous_display_point_range_for_buffer_range(
        &self,
        range: Range<MultiBufferOffset>,
    ) -> Option<Range<DisplayPoint>> {
        if range.is_empty() {
            return None;
        }

        let buffer = self.buffer_snapshot();
        let first_character_end =
            buffer.clip_offset((range.start + 1usize).min(range.end), Bias::Right);
        let last_character_start = buffer.clip_offset(
            range.end.saturating_sub_usize(1).max(range.start),
            Bias::Left,
        );

        let mut converter = self.display_point_converter();
        let first_ranges = converter.map(range.start..first_character_end);
        let start = first_ranges.first()?.start;
        if first_character_end == range.end {
            return Some(start..first_ranges.last()?.end);
        }

        let last_ranges = converter.map(last_character_start..range.end);
        Some(start..last_ranges.last()?.end)
    }

    /// Returns a converter that maps buffer offset ranges to `DisplayPoint`
    /// ranges (as in [`Self::isomorphic_display_point_ranges_for_buffer_range`])
    /// while reusing cursor state across calls. Use this when converting many
    /// ranges in a single pass; the inputs must be supplied with non-decreasing
    /// offsets so the underlying cursors only advance forward.
    pub fn display_point_converter(&self) -> DisplayPointConverter<'_> {
        DisplayPointConverter {
            inlay_cursor: self.inlay_snapshot().buffer_offset_to_inlay_point_cursor(),
            fold_point_cursor: self.fold_snapshot().fold_point_cursor(),
            tab_point_cursor: self.tab_snapshot().tab_point_cursor(),
            wrap_point_cursor: self.wrap_snapshot().wrap_point_cursor(),
            block_point_cursor: self.block_snapshot.block_point_cursor(),
            prev_end: None,
        }
    }

    pub fn display_point_to_point(&self, point: DisplayPoint, bias: Bias) -> Point {
        self.inlay_snapshot()
            .to_buffer_point(self.display_point_to_inlay_point(point, bias))
    }

    pub fn display_point_to_inlay_offset(&self, point: DisplayPoint, bias: Bias) -> InlayOffset {
        self.inlay_snapshot()
            .to_offset(self.display_point_to_inlay_point(point, bias))
    }

    pub fn anchor_to_inlay_offset(&self, anchor: Anchor) -> InlayOffset {
        self.inlay_snapshot()
            .to_inlay_offset(anchor.to_offset(self.buffer_snapshot()))
    }

    pub fn display_point_to_anchor(&self, point: DisplayPoint, bias: Bias) -> Anchor {
        self.buffer_snapshot()
            .anchor_at(point.to_offset(self, bias), bias)
    }

    #[instrument(skip_all)]
    fn display_point_to_inlay_point(&self, point: DisplayPoint, bias: Bias) -> InlayPoint {
        let block_point = point.0;
        let wrap_point = self.block_snapshot.to_wrap_point(block_point, bias);
        let tab_point = self.wrap_snapshot().to_tab_point(wrap_point);
        let fold_point = self
            .tab_snapshot()
            .tab_point_to_fold_point(tab_point, bias)
            .0;
        fold_point.to_inlay_point(self.fold_snapshot())
    }

    #[instrument(skip_all)]
    pub fn display_point_to_fold_point(&self, point: DisplayPoint, bias: Bias) -> FoldPoint {
        let block_point = point.0;
        let wrap_point = self.block_snapshot.to_wrap_point(block_point, bias);
        let tab_point = self.wrap_snapshot().to_tab_point(wrap_point);
        self.tab_snapshot()
            .tab_point_to_fold_point(tab_point, bias)
            .0
    }

    #[instrument(skip_all)]
    pub fn fold_point_to_display_point(&self, fold_point: FoldPoint) -> DisplayPoint {
        let tab_point = self.tab_snapshot().fold_point_to_tab_point(fold_point);
        let wrap_point = self.wrap_snapshot().tab_point_to_wrap_point(tab_point);
        let block_point = self.block_snapshot.to_block_point(wrap_point);
        DisplayPoint(block_point)
    }

    pub fn max_point(&self) -> DisplayPoint {
        DisplayPoint(self.block_snapshot.max_point())
    }

    /// Returns text chunks starting at the given display row until the end of the file
    #[instrument(skip_all)]
    pub fn text_chunks(&self, display_row: DisplayRow) -> impl Iterator<Item = &str> {
        let chunks = self.block_snapshot.chunks(
            BlockRow(display_row.0)..BlockRow(self.max_point().row().next_row().0),
            LanguageAwareStyling {
                tree_sitter: false,
                diagnostics: false,
            },
            Highlights::default(),
        );
        self.mask_chunks_if_needed(chunks).map(|h| h.text)
    }

    fn text_chunks_from(&self, point: DisplayPoint) -> impl Iterator<Item = &str> {
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: false,
        };
        if self.is_long_unwrapped_row(point.row()) {
            let start = self.block_snapshot.to_wrap_point(point.0, Bias::Left);
            let wrap_snapshot = self.wrap_snapshot();
            let end = if start.row() < wrap_snapshot.max_point().row() {
                WrapPoint::new(start.row() + WrapRow(1), 0)
            } else {
                WrapPoint::new(start.row(), wrap_snapshot.line_len(start.row()))
            };
            let chunks =
                self.wrap_snapshot()
                    .chunks(start..end, language_aware, Highlights::default());
            Either::Left(self.mask_chunks_if_needed(chunks).map(|chunk| chunk.text))
        } else {
            let chunks = self.block_snapshot.chunks(
                BlockRow(point.row().0)..BlockRow(self.max_point().row().next_row().0),
                language_aware,
                Highlights::default(),
            );
            let mut column = 0;
            Either::Right(
                self.mask_chunks_if_needed(chunks)
                    .map(|chunk| chunk.text)
                    .filter_map(move |chunk| {
                        let chunk_start = column;
                        column += chunk.len() as u32;
                        let skip = point.column().saturating_sub(chunk_start) as usize;
                        (skip < chunk.len()).then(|| &chunk[skip..])
                    }),
            )
        }
    }

    /// Returns text chunks starting at the end of the given display row in reverse until the start of the file
    #[instrument(skip_all)]
    pub fn reverse_text_chunks(&self, display_row: DisplayRow) -> impl Iterator<Item = &str> {
        (0..=display_row.0).rev().flat_map(move |row| {
            let chunks = self.block_snapshot.chunks(
                BlockRow(row)..BlockRow(row + 1),
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                Highlights::default(),
            );
            self.mask_chunks_if_needed(chunks)
                .map(|h| h.text)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
        })
    }

    #[instrument(skip_all)]
    pub fn chunks(
        &self,
        display_rows: Range<DisplayRow>,
        language_aware: LanguageAwareStyling,
        highlight_styles: HighlightStyles,
    ) -> impl Iterator<Item = Chunk<'_>> {
        let chunks = self.block_snapshot.chunks(
            BlockRow(display_rows.start.0)..BlockRow(display_rows.end.0),
            language_aware,
            Highlights {
                text_highlights: Some(&self.text_highlights),
                inlay_highlights: Some(&self.inlay_highlights),
                semantic_token_highlights: Some(&self.semantic_token_highlights),
                styles: highlight_styles,
            },
        );
        self.mask_chunks_if_needed(chunks)
    }

    fn mask_chunks_if_needed<'a>(
        &self,
        chunks: impl Iterator<Item = Chunk<'a>>,
    ) -> impl Iterator<Item = Chunk<'a>> {
        if self.masked {
            Either::Left(mask_chunks(chunks))
        } else {
            Either::Right(chunks)
        }
    }

    #[instrument(skip_all)]
    pub fn highlighted_chunks<'a>(
        &'a self,
        display_rows: Range<DisplayRow>,
        language_aware: LanguageAwareStyling,
        editor_style: &'a EditorStyle,
    ) -> impl Iterator<Item = HighlightedChunk<'a>> {
        let chunks = self.chunks(
            display_rows,
            language_aware,
            HighlightStyles {
                inlay_hint: Some(editor_style.inlay_hints_style),
                edit_prediction: Some(editor_style.edit_prediction_styles),
            },
        );
        self.map_to_highlighted_chunks(chunks, editor_style)
    }

    pub(crate) fn highlighted_chunks_in_range<'a>(
        &'a self,
        range: Range<DisplayPoint>,
        language_aware: LanguageAwareStyling,
        editor_style: &'a EditorStyle,
    ) -> impl Iterator<Item = HighlightedChunk<'a>> {
        debug_assert!(
            range.start.column() <= self.line_len(range.start.row())
                && range.end.column() <= self.line_len(range.end.row()),
            "column sub-ranges must stay within their rows: {range:?}"
        );
        debug_assert!(
            !self.has_soft_wraps() || (range.start.column() == 0 && range.end.column() == 0),
            "column sub-ranges are unsupported with soft wraps: wrap chunks emit synthetic \
             soft-wrap indentation whole, never clipped by column"
        );
        debug_assert!(
            (range.start.row().0..=range.end.row().0)
                .all(|row| !self.is_block_line(DisplayRow(row))),
            "block rows are unsupported: this path reads the wrap snapshot, bypassing the block map"
        );
        let start = self.block_snapshot.to_wrap_point(range.start.0, Bias::Left);
        let end = self.block_snapshot.to_wrap_point(range.end.0, Bias::Right);
        let chunks = self.wrap_snapshot().chunks(
            start..end,
            language_aware,
            Highlights {
                text_highlights: Some(&self.text_highlights),
                inlay_highlights: Some(&self.inlay_highlights),
                semantic_token_highlights: Some(&self.semantic_token_highlights),
                styles: HighlightStyles {
                    inlay_hint: Some(editor_style.inlay_hints_style),
                    edit_prediction: Some(editor_style.edit_prediction_styles),
                },
            },
        );
        let chunks = self.mask_chunks_if_needed(chunks);
        let seed = if range.start.column() > 0 {
            self.diagnostic_state_before(range.start, language_aware, editor_style)
        } else {
            DiagnosticState::default()
        };
        self.map_to_highlighted_chunks_from(chunks, editor_style, seed)
    }

    fn diagnostic_state_before(
        &self,
        point: DisplayPoint,
        language_aware: LanguageAwareStyling,
        editor_style: &EditorStyle,
    ) -> DiagnosticState {
        let buffer = self.buffer_snapshot();
        let inlay_offset = self.display_point_to_inlay_offset(point, Bias::Left);
        let end = self.inlay_snapshot().to_buffer_offset(inlay_offset);
        let mut state = DiagnosticState::default();
        let Some(start) = end.0.checked_sub(1) else {
            return state;
        };
        let start = buffer.clip_offset(MultiBufferOffset(start), Bias::Left);
        if self.fold_snapshot().intersects_fold(start) {
            return state;
        }
        for chunk in buffer.chunks(start..end, language_aware) {
            state.observe(
                chunk.diagnostic_severity,
                chunk.underline,
                chunk.is_unnecessary,
                self,
                editor_style,
            );
        }
        state
    }

    fn map_to_highlighted_chunks<'a>(
        &'a self,
        chunks: impl Iterator<Item = Chunk<'a>> + 'a,
        editor_style: &'a EditorStyle,
    ) -> impl Iterator<Item = HighlightedChunk<'a>> + 'a {
        self.map_to_highlighted_chunks_from(chunks, editor_style, DiagnosticState::default())
    }

    fn map_to_highlighted_chunks_from<'a>(
        &'a self,
        chunks: impl Iterator<Item = Chunk<'a>> + 'a,
        editor_style: &'a EditorStyle,
        mut diagnostic_state: DiagnosticState,
    ) -> impl Iterator<Item = HighlightedChunk<'a>> + 'a {
        chunks.flat_map({
            // track the current underline style so that we can apply it to
            // inlay hints within the diagnostic's span
            move |chunk| {
                let syntax_highlight_style = chunk
                    .syntax_highlight_id
                    .and_then(|id| editor_style.syntax.get(id).cloned());

                let chunk_highlight = chunk.highlight_style.map(|chunk_highlight| {
                    HighlightStyle {
                        // For color inlays, blend the color with the editor background
                        // if the color has transparency (alpha < 1.0)
                        color: chunk_highlight.color.map(|color| {
                            if chunk.is_inlay && !color.is_opaque() {
                                editor_style.background.blend(color)
                            } else {
                                color
                            }
                        }),
                        underline: chunk_highlight
                            .underline
                            .filter(|_| editor_style.show_underlines),
                        ..chunk_highlight
                    }
                });

                let (diagnostic_highlight, diagnostic_severity) = if chunk.is_inlay {
                    (
                        diagnostic_state.underline.map(|underline| HighlightStyle {
                            underline: Some(underline),
                            ..Default::default()
                        }),
                        diagnostic_state.severity,
                    )
                } else {
                    let highlight = diagnostic_state.observe(
                        chunk.diagnostic_severity,
                        chunk.underline,
                        chunk.is_unnecessary,
                        self,
                        editor_style,
                    );
                    (highlight, diagnostic_state.severity)
                };

                let style = [
                    syntax_highlight_style,
                    chunk_highlight,
                    diagnostic_highlight,
                ]
                .into_iter()
                .flatten()
                .reduce(|acc, highlight| acc.highlight(highlight));

                HighlightedChunk {
                    text: chunk.text,
                    style,
                    diagnostic_underline_severity: diagnostic_severity,
                    is_tab: chunk.is_tab,
                    is_inlay: chunk.is_inlay,
                    replacement: chunk.renderer.map(ChunkReplacement::Renderer),
                }
                .highlight_invisibles(editor_style)
            }
        })
    }

    /// Returns combined highlight styles (tree-sitter syntax + semantic tokens)
    /// for a byte range within the specified buffer.
    /// Returned ranges are 0-based relative to `buffer_range.start`.
    pub(super) fn combined_highlights(
        &self,
        multibuffer_range: Range<MultiBufferOffset>,
        syntax_theme: &theme::SyntaxTheme,
    ) -> Vec<(Range<usize>, HighlightStyle)> {
        let multibuffer = self.buffer_snapshot();

        let chunks = custom_highlights::CustomHighlightsChunks::new(
            multibuffer_range,
            LanguageAwareStyling {
                tree_sitter: true,
                diagnostics: true,
            },
            None,
            Some(&self.semantic_token_highlights),
            multibuffer,
        );

        let mut highlights = Vec::new();
        let mut offset = 0usize;
        for chunk in chunks {
            let chunk_len = chunk.text.len();
            if chunk_len == 0 {
                continue;
            }

            let syntax_style = chunk
                .syntax_highlight_id
                .and_then(|id| syntax_theme.get(id).cloned());

            let overlay_style = chunk.highlight_style;

            let combined = match (syntax_style, overlay_style) {
                (Some(syntax), Some(overlay)) => Some(syntax.highlight(overlay)),
                (some @ Some(_), None) | (None, some @ Some(_)) => some,
                (None, None) => None,
            };

            if let Some(style) = combined {
                highlights.push((offset..offset + chunk_len, style));
            }
            offset += chunk_len;
        }
        highlights
    }

    pub fn is_windowed_row(&self, display_row: DisplayRow, cell: GridCell) -> bool {
        self.is_long_unwrapped_row(display_row) && self.row_has_exact_grid(display_row, cell)
    }

    pub fn is_long_unwrapped_row(&self, display_row: DisplayRow) -> bool {
        self.long_unwrapped_row_len(display_row).is_some()
    }

    pub fn long_unwrapped_row_len(&self, display_row: DisplayRow) -> Option<u32> {
        if self.masked
            || self.has_soft_wraps()
            || display_row > self.max_point().row()
            || self.is_block_line(display_row)
        {
            return None;
        }
        let row_len = self.line_len(display_row);
        (row_len as usize > MAX_LINE_LEN).then_some(row_len)
    }

    pub fn grid_window(
        &self,
        display_row: DisplayRow,
        row_len: u32,
        viewport: &HorizontalViewport,
        cell: GridCell,
        shaper: &RulerShaper,
    ) -> Option<(Range<u32>, Arc<LineLayout>)> {
        if !self.row_has_exact_grid(display_row, cell) {
            return None;
        }
        let row_width = ScrollPixelOffset::from(cell.width) * row_len as ScrollPixelOffset;
        let window = viewport.aligned(row_width, cell).shaping_window(row_len);
        let shaped = shaper.layout_columns(self, display_row, window.clone());
        if !cell.fits(&window, shaped.width) {
            debug_panic!(
                "grid-exact fonts must shape {} columns to {} px, got {:?}",
                window.len(),
                f64::from(cell.width) * window.len() as f64,
                shaped.width
            );
            return None;
        }
        Some((window, shaped))
    }

    fn wrap_row(&self, display_row: DisplayRow) -> u32 {
        self.block_snapshot
            .to_wrap_point(DisplayPoint::new(display_row, 0).0, Bias::Left)
            .row()
            .0
    }

    pub fn ruled_row(&self, display_row: DisplayRow, shaper: RulerShaper) -> RuledRow {
        let wrap_row = self.wrap_row(display_row);
        let ruler = self
            .row_rulers
            .get_or_build(wrap_row, &shaper, |previous, renderer_widths| {
                RowRuler::new(self, display_row, &shaper, previous, renderer_widths)
            });
        RuledRow {
            snapshot: Arc::new(self.clone()),
            row: display_row,
            ruler,
            shaper,
        }
    }

    pub fn row_has_exact_grid(&self, display_row: DisplayRow, cell: GridCell) -> bool {
        if !cell.monospace {
            return false;
        }
        let fold_snapshot = self.fold_snapshot();
        let fold_row = self
            .display_point_to_fold_point(DisplayPoint::new(display_row, 0), Bias::Left)
            .row();
        let fold_range =
            FoldPoint::new(fold_row, 0)..FoldPoint::new(fold_row, fold_snapshot.line_len(fold_row));
        let summary = fold_snapshot.text_summary_for_range(fold_range.clone());
        if summary.chars != summary.len.0 {
            return false;
        }
        let inlay_range = fold_range.start.to_inlay_point(fold_snapshot)
            ..fold_range.end.to_inlay_point(fold_snapshot);
        let inlay_snapshot = self.inlay_snapshot();
        let buffer_range = inlay_snapshot.to_buffer_point(inlay_range.start)
            ..inlay_snapshot.to_buffer_point(inlay_range.end);
        let buffer = self.buffer_snapshot();
        let buffer_offsets =
            buffer.point_to_offset(buffer_range.start)..buffer.point_to_offset(buffer_range.end);
        !self.control_chars.intersects(buffer, buffer_offsets)
            && fold_snapshot.folds_in_range(buffer_range).next().is_none()
            && !inlay_snapshot.has_inlays_matching(inlay_range, |inlay, text_range| {
                inlay_chunk_renderer(inlay).is_some()
                    || !inlay
                        .text()
                        .chunks_in_range(text_range)
                        .all(|chunk| all_grid_bytes(chunk.as_bytes()))
            })
    }

    #[instrument(skip_all)]
    pub fn layout_row(&self, display_row: DisplayRow, details: &TextLayoutDetails) -> RowLayout {
        let cell = details.grid_cell();
        if let Some(row_len) = self.long_unwrapped_row_len(display_row) {
            let shaper = details.ruler_shaper(self, display_row);
            let viewport = details.horizontal_viewport(self);
            if let Some((window, shaped)) =
                self.grid_window(display_row, row_len, &viewport, cell, &shaper)
            {
                return RowLayout::Windowed {
                    geometry: WindowedRowGeometry::new(row_len, cell, window),
                    shaped,
                };
            }
            let ruled = self.ruled_row(display_row, shaper);
            return RowLayout::Windowed {
                geometry: WindowedRowGeometry::ruled(ruled, row_len, cell, 0..0),
                shaped: Arc::new(LineLayout::default()),
            };
        }

        let chunks = self.highlighted_chunks(
            display_row..display_row.next_row(),
            details.language_aware(self, display_row),
            &details.editor_style,
        );
        RowLayout::Shaped(details.shape_row_text(chunks))
    }

    pub fn x_for_display_point(
        &self,
        display_point: DisplayPoint,
        text_layout_details: &TextLayoutDetails,
    ) -> ScrollPixelOffset {
        let line = self.layout_row(display_point.row(), text_layout_details);
        line.x_for_index(display_point.column() as usize)
    }

    pub fn display_column_for_x(
        &self,
        display_row: DisplayRow,
        x: ScrollPixelOffset,
        details: &TextLayoutDetails,
    ) -> u32 {
        let layout_line = self.layout_row(display_row, details);
        layout_line.closest_index_for_x(x) as u32
    }

    #[instrument(skip_all)]
    pub fn grapheme_at(&self, mut point: DisplayPoint) -> Option<SharedString> {
        point = DisplayPoint(self.block_snapshot.clip_point(point.0, Bias::Left));
        let mut chars = self.text_chunks_from(point).flat_map(str::chars);
        let mut grapheme = String::from(chars.next()?);
        let mut cursor = GraphemeCursor::new(0, usize::MAX, true);
        loop {
            match cursor.next_boundary(&grapheme, 0) {
                Ok(Some(boundary)) => {
                    grapheme.truncate(boundary);
                    break;
                }
                Err(GraphemeIncomplete::NextChunk) => match chars.next() {
                    Some(char) => grapheme.push(char),
                    None => break,
                },
                Ok(None) | Err(_) => break,
            }
        }
        let grapheme = SharedString::from(grapheme);
        if let Some(invisible) = grapheme.chars().next().filter(|&c| is_invisible(c)) {
            Some(replacement(invisible).map_or(grapheme, SharedString::from))
        } else if grapheme == "\n" {
            Some(" ".into())
        } else {
            Some(grapheme)
        }
    }

    pub fn buffer_chars_at(
        &self,
        mut offset: MultiBufferOffset,
    ) -> impl Iterator<Item = (char, MultiBufferOffset)> + '_ {
        self.buffer_snapshot().chars_at(offset).map(move |ch| {
            let ret = (ch, offset);
            offset += ch.len_utf8();
            ret
        })
    }

    pub fn reverse_buffer_chars_at(
        &self,
        mut offset: MultiBufferOffset,
    ) -> impl Iterator<Item = (char, MultiBufferOffset)> + '_ {
        self.buffer_snapshot()
            .reversed_chars_at(offset)
            .map(move |ch| {
                offset -= ch.len_utf8();
                (ch, offset)
            })
    }

    pub fn clip_point(&self, point: DisplayPoint, bias: Bias) -> DisplayPoint {
        let mut clipped = self.block_snapshot.clip_point(point.0, bias);
        if self.clip_at_line_ends {
            clipped = self.clip_at_line_end(DisplayPoint(clipped)).0
        }
        DisplayPoint(clipped)
    }

    pub fn clip_ignoring_line_ends(&self, point: DisplayPoint, bias: Bias) -> DisplayPoint {
        DisplayPoint(self.block_snapshot.clip_point(point.0, bias))
    }

    pub fn inlay_bias_at(&self, point: DisplayPoint) -> Option<Bias> {
        let wrap_point = self.block_snapshot.to_wrap_point(point.0, Bias::Left);
        let tab_point = self.block_snapshot.to_tab_point(wrap_point);
        let (fold_point, _, _) = self
            .block_snapshot
            .tab_snapshot
            .tab_point_to_fold_point(tab_point, Bias::Left);
        let inlay_point =
            fold_point.to_inlay_point(&self.block_snapshot.tab_snapshot.fold_snapshot);
        self.block_snapshot
            .tab_snapshot
            .fold_snapshot
            .inlay_bias_at_point(inlay_point)
    }

    pub fn clip_at_line_end(&self, display_point: DisplayPoint) -> DisplayPoint {
        let mut point = self.display_point_to_point(display_point, Bias::Left);

        if point.column != self.buffer_snapshot().line_len(MultiBufferRow(point.row)) {
            return display_point;
        }
        point.column = point.column.saturating_sub(1);
        point = self.buffer_snapshot().clip_point(point, Bias::Left);
        self.point_to_display_point(point, Bias::Left)
    }

    pub fn folds_in_range<T>(&self, range: Range<T>) -> impl Iterator<Item = &Fold>
    where
        T: ToOffset,
    {
        self.fold_snapshot().folds_in_range(range)
    }

    pub fn blocks_in_range(
        &self,
        rows: Range<DisplayRow>,
    ) -> impl Iterator<Item = (DisplayRow, &Block)> {
        self.block_snapshot
            .blocks_in_range(BlockRow(rows.start.0)..BlockRow(rows.end.0))
            .map(|(row, block)| (DisplayRow(row.0), block))
    }

    pub fn sticky_header_excerpt(&self, row: f64) -> Option<StickyHeaderExcerpt<'_>> {
        self.block_snapshot.sticky_header_excerpt(row)
    }

    pub fn block_for_id(&self, id: BlockId) -> Option<Block> {
        self.block_snapshot.block_for_id(id)
    }

    pub fn intersects_fold<T: ToOffset>(&self, offset: T) -> bool {
        self.fold_snapshot().intersects_fold(offset)
    }

    pub fn is_line_folded(&self, buffer_row: MultiBufferRow) -> bool {
        self.block_snapshot.is_line_replaced(buffer_row)
            || self.fold_snapshot().is_line_folded(buffer_row)
    }

    pub fn is_block_line(&self, display_row: DisplayRow) -> bool {
        self.block_snapshot.is_block_line(BlockRow(display_row.0))
    }

    pub fn is_folded_buffer_header(&self, display_row: DisplayRow) -> bool {
        self.block_snapshot
            .is_folded_buffer_header(BlockRow(display_row.0))
    }

    pub fn soft_wrap_indent(&self, display_row: DisplayRow) -> Option<u32> {
        let wrap_row = self
            .block_snapshot
            .to_wrap_point(BlockPoint::new(BlockRow(display_row.0), 0), Bias::Left)
            .row();
        self.wrap_snapshot().soft_wrap_indent(wrap_row)
    }

    pub fn text(&self) -> String {
        self.text_chunks(DisplayRow(0)).collect()
    }

    pub fn line(&self, display_row: DisplayRow) -> String {
        let mut result = String::new();
        for chunk in self.text_chunks(display_row) {
            if let Some(ix) = chunk.find('\n') {
                result.push_str(&chunk[0..ix]);
                break;
            } else {
                result.push_str(chunk);
            }
        }
        result
    }

    pub fn line_indent_for_buffer_row(&self, buffer_row: MultiBufferRow) -> LineIndent {
        self.buffer_snapshot().line_indent_for_row(buffer_row)
    }

    pub fn line_len(&self, row: DisplayRow) -> u32 {
        self.block_snapshot.line_len(BlockRow(row.0))
    }

    pub fn longest_row(&self) -> DisplayRow {
        DisplayRow(self.block_snapshot.longest_row().0)
    }

    pub fn longest_row_in_range(&self, range: Range<DisplayRow>) -> DisplayRow {
        let block_range = BlockRow(range.start.0)..BlockRow(range.end.0);
        let longest_row = self.block_snapshot.longest_row_in_range(block_range);
        DisplayRow(longest_row.0)
    }

    pub fn starts_indent(&self, buffer_row: MultiBufferRow) -> bool {
        let max_row = self.buffer_snapshot().max_row();
        if buffer_row >= max_row {
            return false;
        }

        let line_indent = self.line_indent_for_buffer_row(buffer_row);
        if line_indent.is_line_blank() {
            return false;
        }

        (buffer_row.0 + 1..=max_row.0)
            .find_map(|next_row| {
                let next_line_indent = self.line_indent_for_buffer_row(MultiBufferRow(next_row));
                if next_line_indent.raw_len() > line_indent.raw_len() {
                    Some(true)
                } else if !next_line_indent.is_line_blank() {
                    Some(false)
                } else {
                    None
                }
            })
            .unwrap_or(false)
    }

    /// Returns the indent length of `row` if it starts with a closing bracket.
    fn closing_bracket_indent_len(&self, row: u32) -> Option<u32> {
        let snapshot = self.buffer_snapshot();
        let indent_len = self
            .line_indent_for_buffer_row(MultiBufferRow(row))
            .raw_len();
        let content_start = Point::new(row, indent_len);
        let line_text: String = snapshot
            .chars_at(content_start)
            .take_while(|ch| *ch != '\n')
            .collect();

        let scope = snapshot.language_scope_at(Point::new(row, 0))?;
        if scope
            .brackets()
            .any(|(pair, _)| line_text.starts_with(&pair.end))
        {
            return Some(indent_len);
        }

        None
    }

    #[instrument(skip_all)]
    pub fn crease_for_buffer_row(&self, buffer_row: MultiBufferRow) -> Option<Crease<Point>> {
        let start =
            MultiBufferPoint::new(buffer_row.0, self.buffer_snapshot().line_len(buffer_row));
        if let Some(crease) = self
            .crease_snapshot
            .query_row(buffer_row, self.buffer_snapshot())
        {
            match crease {
                Crease::Inline {
                    range,
                    placeholder,
                    render_toggle,
                    render_trailer,
                    metadata,
                } => Some(Crease::Inline {
                    range: range.to_point(self.buffer_snapshot()),
                    placeholder: placeholder.clone(),
                    render_toggle: render_toggle.clone(),
                    render_trailer: render_trailer.clone(),
                    metadata: metadata.clone(),
                }),
                Crease::Block {
                    range,
                    block_height,
                    block_style,
                    render_block,
                    block_priority,
                    render_toggle,
                } => Some(Crease::Block {
                    range: range.to_point(self.buffer_snapshot()),
                    block_height: *block_height,
                    block_style: *block_style,
                    render_block: render_block.clone(),
                    block_priority: *block_priority,
                    render_toggle: render_toggle.clone(),
                }),
            }
        } else if !self.use_lsp_folding_ranges
            && self.starts_indent(MultiBufferRow(start.row))
            && !self.is_line_folded(MultiBufferRow(start.row))
        {
            let start_line_indent = self.line_indent_for_buffer_row(buffer_row);
            let snapshot = self.buffer_snapshot();
            let max_point = snapshot.max_point();
            let mut closing_row = None;

            // End byte of the smallest syntactic node enclosing `buffer_row`.
            // Used to tell standalone top-level comments (which terminate the
            // fold) apart from unindented content inside a multi-line string
            // or block comment belonging to the folded node (which does not).
            let foldable_node_end = {
                let row_start = Point::new(buffer_row.0, 0);
                let row_end = Point::new(buffer_row.0, snapshot.line_len(buffer_row));
                snapshot
                    .syntax_ancestor(row_start..row_end)
                    .map(|(_, range)| range.end)
            };

            for row in (buffer_row.0 + 1)..=max_point.row {
                let line_indent = self.line_indent_for_buffer_row(MultiBufferRow(row));
                if !line_indent.is_line_blank()
                    && line_indent.raw_len() <= start_line_indent.raw_len()
                {
                    let in_string_or_comment_scope = snapshot
                        .language_scope_at(Point::new(row, 0))
                        .is_some_and(|scope| {
                            matches!(
                                scope.override_name(),
                                Some("string") | Some("comment") | Some("comment.inclusive")
                            )
                        });
                    if in_string_or_comment_scope
                        && let Some(end) = foldable_node_end
                        && Point::new(row, 0).to_offset(snapshot) < end
                    {
                        continue;
                    }

                    closing_row = Some(row);
                    break;
                }
            }

            let last_non_blank_row = |from_row: u32| -> Point {
                let mut row = from_row;
                while row > start.row && self.buffer_snapshot().is_line_blank(MultiBufferRow(row)) {
                    row -= 1;
                }
                Point::new(row, self.buffer_snapshot().line_len(MultiBufferRow(row)))
            };

            let end = if let Some(row) = closing_row {
                if let Some(indent_len) = self.closing_bracket_indent_len(row) {
                    // Include newline and whitespace before closing delimiter,
                    // so it appears on the same display line as the fold placeholder
                    Point::new(row, indent_len)
                } else {
                    last_non_blank_row(row - 1)
                }
            } else {
                last_non_blank_row(max_point.row)
            };

            Some(Crease::Inline {
                range: start..end,
                placeholder: self.fold_placeholder.clone(),
                render_toggle: None,
                render_trailer: None,
                metadata: None,
            })
        } else {
            None
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    #[instrument(skip_all)]
    pub fn text_highlight_ranges(
        &self,
        key: HighlightKey,
    ) -> Option<Arc<(HighlightStyle, Vec<Range<Anchor>>)>> {
        self.text_highlights.get(&key).cloned()
    }

    #[cfg(any(test, feature = "test-support"))]
    #[instrument(skip_all)]
    pub fn all_text_highlight_ranges(
        &self,
        f: &dyn Fn(&HighlightKey) -> bool,
    ) -> Vec<(gpui::Hsla, Range<Point>)> {
        use itertools::Itertools;

        self.text_highlights
            .iter()
            .filter(|(key, _)| f(key))
            .map(|(_, value)| value.clone())
            .flat_map(|ranges| {
                ranges
                    .1
                    .iter()
                    .flat_map(|range| {
                        Some((ranges.0.color?, range.to_point(self.buffer_snapshot())))
                    })
                    .collect::<Vec<_>>()
            })
            .sorted_by_key(|(_, range)| range.start)
            .collect()
    }

    #[allow(unused)]
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn inlay_highlights(
        &self,
        key: HighlightKey,
    ) -> Option<&TreeMap<InlayId, (HighlightStyle, InlayHighlight)>> {
        self.inlay_highlights.get(&key)
    }

    pub fn buffer_header_height(&self) -> u32 {
        self.block_snapshot.buffer_header_height
    }

    pub fn excerpt_header_height(&self) -> u32 {
        self.block_snapshot.excerpt_header_height
    }

    /// Given a `DisplayPoint`, returns another `DisplayPoint` corresponding to
    /// the start of the buffer row that is a given number of buffer rows away
    /// from the provided point.
    ///
    /// This moves by buffer rows instead of display rows, a distinction that is
    /// important when soft wrapping is enabled.
    #[instrument(skip_all)]
    pub fn start_of_relative_buffer_row(&self, point: DisplayPoint, times: isize) -> DisplayPoint {
        let start = self.display_point_to_fold_point(point, Bias::Left);
        let target = start.row() as isize + times;
        let new_row = (target.max(0) as u32).min(self.fold_snapshot().max_point().row());

        self.clip_point(
            self.fold_point_to_display_point(
                self.fold_snapshot()
                    .clip_point(FoldPoint::new(new_row, 0), Bias::Right),
            ),
            Bias::Right,
        )
    }

    pub(crate) fn fully_replaced_tab_rows(&self, row: u32) -> Option<RangeInclusive<u32>> {
        if !self.block_snapshot.has_replacement_blocks()
            || row > self.tab_snapshot().max_point().row()
        {
            return None;
        }
        let wraps = self.wrap_snapshot();
        let wrap_point = wraps.tab_point_to_wrap_point(TabPoint::new(row, 0));
        let input_range = self
            .block_snapshot
            .replacement_block_input_range(wrap_point.row())?;
        let start = wraps.to_tab_point(WrapPoint::new(input_range.start, 0));
        let start_row = start.row().checked_add(u32::from(start.column() > 0))?;
        let end_row = if input_range.end > wraps.max_point().row() {
            self.tab_snapshot().max_point().row()
        } else {
            wraps
                .to_tab_point(WrapPoint::new(input_range.end, 0))
                .row()
                .checked_sub(1)?
        };
        let rows = start_row..=end_row;
        rows.contains(&row).then_some(rows)
    }
}

fn diagnostic_style(severity: lsp::DiagnosticSeverity, colors: &StatusColors) -> Hsla {
    match severity {
        lsp::DiagnosticSeverity::ERROR => colors.error,
        lsp::DiagnosticSeverity::WARNING => colors.warning,
        lsp::DiagnosticSeverity::INFORMATION => colors.info,
        lsp::DiagnosticSeverity::HINT => colors.hint,
        _ => colors.ignored,
    }
}

impl std::ops::Deref for DisplaySnapshot {
    type Target = BlockSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.block_snapshot
    }
}

/// A zero-indexed point in a text buffer consisting of a row and column adjusted for inserted blocks.
#[derive(Copy, Clone, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DisplayPoint(BlockPoint);

impl Debug for DisplayPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "DisplayPoint({}, {})",
            self.row().0,
            self.column()
        ))
    }
}

impl Add for DisplayPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        DisplayPoint(BlockPoint(self.0.0 + other.0.0))
    }
}

impl Sub for DisplayPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        DisplayPoint(BlockPoint(self.0.0 - other.0.0))
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, Ord, PartialOrd, PartialEq, Deserialize, Hash)]
#[serde(transparent)]
pub struct DisplayRow(pub u32);

impl DisplayRow {
    pub(crate) fn as_display_point(&self) -> DisplayPoint {
        DisplayPoint::new(*self, 0)
    }
}

impl Add<DisplayRow> for DisplayRow {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        DisplayRow(self.0 + other.0)
    }
}

impl Add<u32> for DisplayRow {
    type Output = Self;

    fn add(self, other: u32) -> Self::Output {
        DisplayRow(self.0 + other)
    }
}

impl Sub<DisplayRow> for DisplayRow {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        DisplayRow(self.0 - other.0)
    }
}

impl Sub<u32> for DisplayRow {
    type Output = Self;

    fn sub(self, other: u32) -> Self::Output {
        DisplayRow(self.0 - other)
    }
}

impl DisplayPoint {
    pub fn new(row: DisplayRow, column: u32) -> Self {
        Self(BlockPoint(Point::new(row.0, column)))
    }

    pub fn zero() -> Self {
        Self::new(DisplayRow(0), 0)
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn row(self) -> DisplayRow {
        DisplayRow(self.0.row)
    }

    pub fn column(self) -> u32 {
        self.0.column
    }

    pub fn row_mut(&mut self) -> &mut u32 {
        &mut self.0.row
    }

    pub fn column_mut(&mut self) -> &mut u32 {
        &mut self.0.column
    }

    pub fn to_point(self, map: &DisplaySnapshot) -> Point {
        map.display_point_to_point(self, Bias::Left)
    }

    pub fn to_offset(self, map: &DisplaySnapshot, bias: Bias) -> MultiBufferOffset {
        let wrap_point = map.block_snapshot.to_wrap_point(self.0, bias);
        let tab_point = map.wrap_snapshot().to_tab_point(wrap_point);
        let fold_point = map
            .tab_snapshot()
            .tab_point_to_fold_point(tab_point, bias)
            .0;
        let inlay_point = fold_point.to_inlay_point(map.fold_snapshot());
        map.inlay_snapshot()
            .to_buffer_offset(map.inlay_snapshot().to_offset(inlay_point))
    }
}

impl ToDisplayPoint for MultiBufferOffset {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        map.point_to_display_point(self.to_point(map.buffer_snapshot()), Bias::Left)
    }
}

impl ToDisplayPoint for MultiBufferOffsetUtf16 {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        self.to_offset(map.buffer_snapshot()).to_display_point(map)
    }
}

impl ToDisplayPoint for Point {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        map.point_to_display_point(*self, Bias::Left)
    }
}

impl ToDisplayPoint for Anchor {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        self.to_point(map.buffer_snapshot()).to_display_point(map)
    }
}

/// Maps buffer offset ranges to `DisplayPoint` ranges covering only buffer text
/// (excluding inlay text), reusing cursor state across calls.
///
/// Created via [`DisplaySnapshot::display_point_converter`]. Each layer
/// (inlay -> fold -> tab -> wrap -> block) is backed by a forward-only cursor,
/// so it is most efficient when ranges are supplied with non-decreasing
/// offsets. If a range starts before the previous one ended, the cursors are
/// transparently reset so the result stays correct (at the cost of an extra
/// seek), which keeps the converter robust to overlapping inputs such as the
/// base and buffer word diffs of an inline modified hunk.
pub struct DisplayPointConverter<'a> {
    inlay_cursor: BufferOffsetToInlayPointCursor<'a>,
    fold_point_cursor: FoldPointCursor<'a>,
    tab_point_cursor: TabPointCursor<'a>,
    wrap_point_cursor: WrapPointCursor<'a>,
    block_point_cursor: BlockPointCursor<'a>,
    prev_end: Option<MultiBufferOffset>,
}

impl DisplayPointConverter<'_> {
    pub fn map(&mut self, range: Range<MultiBufferOffset>) -> SmallVec<[Range<DisplayPoint>; 1]> {
        if self.prev_end.is_some_and(|prev_end| range.start < prev_end) {
            // The input went backward relative to where the cursors are
            // positioned; reset them so they can seek freely.
            self.inlay_cursor.reset();
            self.fold_point_cursor.reset();
            self.tab_point_cursor.reset();
            self.wrap_point_cursor.reset();
            self.block_point_cursor.reset();
        }
        self.prev_end = Some(range.end);

        let inlay_ranges = self.inlay_cursor.map(range);
        inlay_ranges
            .into_iter()
            .map(|inlay_range| {
                let start = self.inlay_point_to_display_point(inlay_range.start);
                let end = self.inlay_point_to_display_point(inlay_range.end);
                start..end
            })
            .collect()
    }

    fn inlay_point_to_display_point(&mut self, inlay_point: InlayPoint) -> DisplayPoint {
        let fold_point = self.fold_point_cursor.map(inlay_point, Bias::Left);
        let tab_point = self.tab_point_cursor.map(fold_point);
        let wrap_point = self.wrap_point_cursor.map(tab_point);
        DisplayPoint(self.block_point_cursor.map(wrap_point))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        movement,
        test::{marked_display_snapshot, test_font},
    };
    use Bias::*;
    use block_map::BlockPlacement;
    use gpui::{
        App, AppContext as _, BorrowAppContext, Element, Hsla, Rgba, div, font, observe, px,
    };
    use language::{
        Buffer, Diagnostic, DiagnosticEntry, DiagnosticSet, Language, LanguageConfig,
        LanguageMatcher,
    };
    use lsp::LanguageServerId;

    use futures::stream::StreamExt;
    use multi_buffer::PathKey;
    use rand::{Rng, prelude::*};
    use settings::{SettingsContent, SettingsStore};
    use std::{env, num::NonZeroU32, sync::Arc};
    use text::PointUtf16;
    use theme::{LoadThemes, SyntaxTheme};
    use unindent::Unindent as _;
    use util::test::{marked_text_ranges, sample_text};

    #[gpui::test(iterations = 100)]
    async fn test_random_display_map(cx: &mut gpui::TestAppContext, mut rng: StdRng) {
        cx.background_executor.set_block_on_ticks(0..=50);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut tab_size = rng.random_range(1..=4);
        let buffer_start_excerpt_header_height = rng.random_range(1..=5);
        let excerpt_header_height = rng.random_range(1..=5);
        let font_size = px(14.0);
        let max_wrap_width = 300.0;
        let mut wrap_width = if rng.random_bool(0.1) {
            None
        } else {
            Some(px(rng.random_range(0.0..=max_wrap_width)))
        };

        log::info!("tab size: {}", tab_size);
        log::info!("wrap width: {:?}", wrap_width);

        cx.update(|cx| {
            init_test(cx, &|s| {
                s.project.all_languages.defaults.tab_size = NonZeroU32::new(tab_size)
            });
        });

        let buffer = cx.update(|cx| {
            if rng.random() {
                let len = rng.random_range(0..10);
                let text = util::RandomCharIter::new(&mut rng)
                    .take(len)
                    .collect::<String>();
                MultiBuffer::build_simple(&text, cx)
            } else {
                MultiBuffer::build_random(&mut rng, cx)
            }
        });

        let font = test_font();
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font,
                font_size,
                wrap_width,
                buffer_start_excerpt_header_height,
                excerpt_header_height,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        let mut notifications = observe(&map, cx);
        let mut fold_count = 0;
        let mut blocks = Vec::new();

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        log::info!("buffer text: {:?}", snapshot.buffer_snapshot().text());
        log::info!("fold text: {:?}", snapshot.fold_snapshot().text());
        log::info!("tab text: {:?}", snapshot.tab_snapshot().text());
        log::info!("wrap text: {:?}", snapshot.wrap_snapshot().text());
        log::info!("block text: {:?}", snapshot.block_snapshot.text());
        log::info!("display text: {:?}", snapshot.text());

        for _i in 0..operations {
            match rng.random_range(0..100) {
                0..=19 => {
                    wrap_width = if rng.random_bool(0.2) {
                        None
                    } else {
                        Some(px(rng.random_range(0.0..=max_wrap_width)))
                    };
                    log::info!("setting wrap width to {:?}", wrap_width);
                    map.update(cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                20..=29 => {
                    let mut tab_sizes = vec![1, 2, 3, 4];
                    tab_sizes.remove((tab_size - 1) as usize);
                    tab_size = *tab_sizes.choose(&mut rng).unwrap();
                    log::info!("setting tab size to {:?}", tab_size);
                    cx.update(|cx| {
                        cx.update_global::<SettingsStore, _>(|store, cx| {
                            store.update_user_settings(cx, |s| {
                                s.project.all_languages.defaults.tab_size =
                                    NonZeroU32::new(tab_size);
                            });
                        });
                    });
                }
                30..=44 => {
                    map.update(cx, |map, cx| {
                        if rng.random() || blocks.is_empty() {
                            let snapshot = map.snapshot(cx);
                            let buffer = snapshot.buffer_snapshot();
                            let block_properties = (0..rng.random_range(1..=1))
                                .map(|_| {
                                    let position = buffer.anchor_after(buffer.clip_offset(
                                        rng.random_range(MultiBufferOffset(0)..=buffer.len()),
                                        Bias::Left,
                                    ));

                                    let placement = if rng.random() {
                                        BlockPlacement::Above(position)
                                    } else {
                                        BlockPlacement::Below(position)
                                    };
                                    let height = rng.random_range(1..5);
                                    log::info!(
                                        "inserting block {:?} with height {}",
                                        placement.as_ref().map(|p| p.to_point(&buffer)),
                                        height
                                    );
                                    let priority = rng.random_range(1..100);
                                    BlockProperties {
                                        placement,
                                        style: BlockStyle::Fixed,
                                        height: Some(height),
                                        render: Arc::new(|_| div().into_any()),
                                        priority,
                                    }
                                })
                                .collect::<Vec<_>>();
                            blocks.extend(map.insert_blocks(block_properties, cx));
                        } else {
                            blocks.shuffle(&mut rng);
                            let remove_count = rng.random_range(1..=4.min(blocks.len()));
                            let block_ids_to_remove = (0..remove_count)
                                .map(|_| blocks.remove(rng.random_range(0..blocks.len())))
                                .collect();
                            log::info!("removing block ids {:?}", block_ids_to_remove);
                            map.remove_blocks(block_ids_to_remove, cx);
                        }
                    });
                }
                45..=79 => {
                    let mut ranges = Vec::new();
                    for _ in 0..rng.random_range(1..=3) {
                        buffer.read_with(cx, |buffer, cx| {
                            let buffer = buffer.read(cx);
                            let end = buffer.clip_offset(
                                rng.random_range(MultiBufferOffset(0)..=buffer.len()),
                                Right,
                            );
                            let start = buffer
                                .clip_offset(rng.random_range(MultiBufferOffset(0)..=end), Left);
                            ranges.push(start..end);
                        });
                    }

                    if rng.random() && fold_count > 0 {
                        log::info!("unfolding ranges: {:?}", ranges);
                        map.update(cx, |map, cx| {
                            map.unfold_intersecting(ranges, true, cx);
                        });
                    } else {
                        log::info!("folding ranges: {:?}", ranges);
                        map.update(cx, |map, cx| {
                            map.fold(
                                ranges
                                    .into_iter()
                                    .map(|range| Crease::simple(range, FoldPlaceholder::test()))
                                    .collect(),
                                cx,
                            );
                        });
                    }
                }
                _ => {
                    buffer.update(cx, |buffer, cx| buffer.randomly_mutate(&mut rng, 5, cx));
                }
            }

            if map.read_with(cx, |map, cx| map.is_rewrapping(cx)) {
                notifications.next().await.unwrap();
            }

            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            fold_count = snapshot.fold_count();
            log::info!("buffer text: {:?}", snapshot.buffer_snapshot().text());
            log::info!("fold text: {:?}", snapshot.fold_snapshot().text());
            log::info!("tab text: {:?}", snapshot.tab_snapshot().text());
            log::info!("wrap text: {:?}", snapshot.wrap_snapshot().text());
            log::info!("block text: {:?}", snapshot.block_snapshot.text());
            log::info!("display text: {:?}", snapshot.text());

            let mut masked_snapshot = snapshot.clone();
            masked_snapshot.masked = true;
            let text = snapshot.text();
            let masked_text = masked_snapshot.text();
            assert_eq!(
                masked_text.split('\n').count(),
                text.split('\n').count(),
                "masking must preserve display row structure"
            );
            for (masked_line, line) in masked_text.split('\n').zip(text.split('\n')) {
                assert_eq!(
                    masked_line,
                    "*".repeat(line.chars().count()),
                    "masking must emit one bullet per char of {line:?}"
                );
            }
            let masked_chunks = masked_snapshot.chunks(
                DisplayRow(0)..masked_snapshot.max_point().row().next_row(),
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                HighlightStyles::default(),
            );
            for chunk in masked_chunks {
                let mut expected_chars = 0u128;
                let mut expected_newlines = 0u128;
                for (ix, c) in chunk.text.char_indices() {
                    expected_chars |= 1 << ix;
                    if c == '\n' {
                        expected_newlines |= 1 << ix;
                    }
                }
                assert_eq!(
                    chunk.tabs, 0,
                    "masked chunk {:?} must have no tabs",
                    chunk.text
                );
                assert_eq!(
                    chunk.chars, expected_chars,
                    "masked chunk {:?} has an inconsistent chars bitmask",
                    chunk.text
                );
                assert_eq!(
                    chunk.newlines, expected_newlines,
                    "masked chunk {:?} has an inconsistent newlines bitmask",
                    chunk.text
                );
            }

            // Line boundaries
            let buffer = snapshot.buffer_snapshot();
            for _ in 0..5 {
                let row = rng.random_range(0..=buffer.max_point().row);
                let column = rng.random_range(0..=buffer.line_len(MultiBufferRow(row)));
                let point = buffer.clip_point(Point::new(row, column), Left);

                let (prev_buffer_bound, prev_display_bound) = snapshot.prev_line_boundary(point);
                let (next_buffer_bound, next_display_bound) = snapshot.next_line_boundary(point);

                assert!(prev_buffer_bound <= point);
                assert!(next_buffer_bound >= point);
                assert_eq!(prev_buffer_bound.column, 0);
                assert_eq!(prev_display_bound.column(), 0);
                if next_buffer_bound < buffer.max_point() {
                    assert_eq!(buffer.chars_at(next_buffer_bound).next(), Some('\n'));
                }

                assert_eq!(
                    prev_display_bound,
                    prev_buffer_bound.to_display_point(&snapshot),
                    "row boundary before {:?}. reported buffer row boundary: {:?}",
                    point,
                    prev_buffer_bound
                );
                assert_eq!(
                    next_display_bound,
                    next_buffer_bound.to_display_point(&snapshot),
                    "display row boundary after {:?}. reported buffer row boundary: {:?}",
                    point,
                    next_buffer_bound
                );
                assert_eq!(
                    prev_buffer_bound,
                    prev_display_bound.to_point(&snapshot),
                    "row boundary before {:?}. reported display row boundary: {:?}",
                    point,
                    prev_display_bound
                );
                assert_eq!(
                    next_buffer_bound,
                    next_display_bound.to_point(&snapshot),
                    "row boundary after {:?}. reported display row boundary: {:?}",
                    point,
                    next_display_bound
                );
            }

            // Movement
            let min_point = snapshot.clip_point(DisplayPoint::new(DisplayRow(0), 0), Left);
            let max_point = snapshot.clip_point(snapshot.max_point(), Right);
            for _ in 0..5 {
                let row = rng.random_range(0..=snapshot.max_point().row().0);
                let column = rng.random_range(0..=snapshot.line_len(DisplayRow(row)));
                let point = snapshot.clip_point(DisplayPoint::new(DisplayRow(row), column), Left);

                log::info!("Moving from point {:?}", point);

                let moved_right = movement::right(&snapshot, point);
                log::info!("Right {:?}", moved_right);
                if point < max_point {
                    assert!(moved_right > point);
                    if point.column() == snapshot.line_len(point.row())
                        || snapshot.soft_wrap_indent(point.row()).is_some()
                            && point.column() == snapshot.line_len(point.row()) - 1
                    {
                        assert!(moved_right.row() > point.row());
                    }
                } else {
                    assert_eq!(moved_right, point);
                }

                let moved_left = movement::left(&snapshot, point);
                log::info!("Left {:?}", moved_left);
                if point > min_point {
                    assert!(moved_left < point);
                    if point.column() == 0 {
                        assert!(moved_left.row() < point.row());
                    }
                } else {
                    assert_eq!(moved_left, point);
                }
            }
        }
    }

    #[gpui::test(retries = 5)]
    async fn test_soft_wraps(cx: &mut gpui::TestAppContext) {
        cx.background_executor
            .set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.update(|cx| {
            init_test(cx, &|_| {});
        });

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        let editor = cx.editor.clone();
        let window = cx.window;

        _ = cx.update_window(window, |_, window, cx| {
            let text_layout_details =
                editor.update(cx, |editor, cx| editor.text_layout_details(window, cx));

            let font_size = px(12.0);
            let wrap_width = Some(px(96.));

            let text = "one two three four five\nsix seven eight";
            let buffer = MultiBuffer::build_simple(text, cx);
            let map = cx.new(|cx| {
                DisplayMap::new(
                    buffer.clone(),
                    font("Helvetica"),
                    font_size,
                    wrap_width,
                    1,
                    1,
                    FoldPlaceholder::test(),
                    DiagnosticSeverity::Warning,
                    cx,
                )
            });

            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                snapshot.text_chunks(DisplayRow(0)).collect::<String>(),
                "one two \nthree four \nfive\nsix seven \neight"
            );
            assert_eq!(
                snapshot.clip_point(DisplayPoint::new(DisplayRow(0), 8), Bias::Left),
                DisplayPoint::new(DisplayRow(0), 7)
            );
            assert_eq!(
                snapshot.clip_point(DisplayPoint::new(DisplayRow(0), 8), Bias::Right),
                DisplayPoint::new(DisplayRow(1), 0)
            );
            assert_eq!(
                movement::right(&snapshot, DisplayPoint::new(DisplayRow(0), 7)),
                DisplayPoint::new(DisplayRow(1), 0)
            );
            assert_eq!(
                movement::left(&snapshot, DisplayPoint::new(DisplayRow(1), 0)),
                DisplayPoint::new(DisplayRow(0), 7)
            );

            let x = snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(1), 10), &text_layout_details);
            assert_eq!(
                movement::up(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(1), 10),
                    language::SelectionGoal::None,
                    false,
                    &text_layout_details,
                ),
                (
                    DisplayPoint::new(DisplayRow(0), 7),
                    language::SelectionGoal::HorizontalPosition(x)
                )
            );
            assert_eq!(
                movement::down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(0), 7),
                    language::SelectionGoal::HorizontalPosition(x),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(1), 10),
                    language::SelectionGoal::HorizontalPosition(x)
                )
            );
            assert_eq!(
                movement::down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(1), 10),
                    language::SelectionGoal::HorizontalPosition(x),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(2), 4),
                    language::SelectionGoal::HorizontalPosition(x)
                )
            );

            let ix = MultiBufferOffset(snapshot.buffer_snapshot().text().find("seven").unwrap());
            buffer.update(cx, |buffer, cx| {
                buffer.edit([(ix..ix, "and ")], None, cx);
            });

            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                snapshot.text_chunks(DisplayRow(1)).collect::<String>(),
                "three four \nfive\nsix and \nseven eight"
            );

            // Re-wrap on font size changes
            map.update(cx, |map, cx| {
                map.set_font(font("Helvetica"), font_size + Pixels::from(3.), cx)
            });

            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                snapshot.text_chunks(DisplayRow(1)).collect::<String>(),
                "three \nfour five\nsix and \nseven \neight"
            )
        });
    }

    #[gpui::test]
    fn test_text_chunks(cx: &mut gpui::App) {
        init_test(cx, &|_| {});

        let text = sample_text(6, 6, 'a');
        let buffer = MultiBuffer::build_simple(&text, cx);

        let font_size = px(14.0);
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![
                    (
                        MultiBufferPoint::new(1, 0)..MultiBufferPoint::new(1, 0),
                        "\t",
                    ),
                    (
                        MultiBufferPoint::new(1, 1)..MultiBufferPoint::new(1, 1),
                        "\t",
                    ),
                    (
                        MultiBufferPoint::new(2, 1)..MultiBufferPoint::new(2, 1),
                        "\t",
                    ),
                ],
                None,
                cx,
            )
        });

        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx))
                .text_chunks(DisplayRow(1))
                .collect::<String>()
                .lines()
                .next(),
            Some("    b   bbbbb")
        );
        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx))
                .text_chunks(DisplayRow(2))
                .collect::<String>()
                .lines()
                .next(),
            Some("c   ccccc")
        );
    }

    #[gpui::test]
    fn test_inlays_with_newlines_after_blocks(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let buffer = cx.new(|cx| Buffer::local("a", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));

        let font_size = px(14.0);
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        map.update(cx, |map, cx| {
            map.insert_blocks(
                [BlockProperties {
                    placement: BlockPlacement::Above(
                        buffer_snapshot.anchor_before(Point::new(0, 0)),
                    ),
                    height: Some(2),
                    style: BlockStyle::Sticky,
                    render: Arc::new(|_| div().into_any()),
                    priority: 0,
                }],
                cx,
            );
        });
        map.update(cx, |m, cx| assert_eq!(m.snapshot(cx).text(), "\n\na"));

        map.update(cx, |map, cx| {
            map.splice_inlays(
                &[],
                vec![Inlay::edit_prediction(
                    0,
                    buffer_snapshot.anchor_after(MultiBufferOffset(0)),
                    "\n",
                )],
                cx,
            );
        });
        map.update(cx, |m, cx| assert_eq!(m.snapshot(cx).text(), "\n\n\na"));

        // Regression test: updating the display map does not crash when a
        // block is immediately followed by a multi-line inlay.
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(MultiBufferOffset(1)..MultiBufferOffset(1), "b")],
                None,
                cx,
            );
        });
        map.update(cx, |m, cx| assert_eq!(m.snapshot(cx).text(), "\n\n\nab"));
    }

    #[gpui::test]
    async fn test_chunks(cx: &mut gpui::TestAppContext) {
        let text = r#"
            fn outer() {}

            mod module {
                fn inner() {}
            }"#
        .unindent();

        let theme =
            SyntaxTheme::new_test(vec![("mod.body", Hsla::red()), ("fn.name", Hsla::blue())]);
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Test".into(),
                    matcher: (LanguageMatcher {
                        path_suffixes: vec![".test".to_string()],
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_highlights_query(
                r#"
                (mod_item name: (identifier) body: _ @mod.body)
                (function_item name: (identifier) @fn.name)
                "#,
            )
            .unwrap(),
        );
        language.set_theme(&theme);

        cx.update(|cx| {
            init_test(cx, &|s| {
                s.project.all_languages.defaults.tab_size = Some(2.try_into().unwrap())
            })
        });

        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.condition(&buffer, |buf, _| !buf.is_parsing()).await;
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let font_size = px(14.0);

        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Helvetica"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(0)..DisplayRow(5), &map, &theme, cx)),
            vec![
                ("fn ".to_string(), None),
                ("outer".to_string(), Some(Hsla::blue())),
                ("() {}\n\nmod module ".to_string(), None),
                ("{\n    fn ".to_string(), Some(Hsla::red())),
                ("inner".to_string(), Some(Hsla::blue())),
                ("() {}\n}".to_string(), Some(Hsla::red())),
            ]
        );
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(3)..DisplayRow(5), &map, &theme, cx)),
            vec![
                ("    fn ".to_string(), Some(Hsla::red())),
                ("inner".to_string(), Some(Hsla::blue())),
                ("() {}\n}".to_string(), Some(Hsla::red())),
            ]
        );

        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    MultiBufferPoint::new(0, 6)..MultiBufferPoint::new(3, 2),
                    FoldPlaceholder::test(),
                )],
                cx,
            )
        });
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(0)..DisplayRow(2), &map, &theme, cx)),
            vec![
                ("fn ".to_string(), None),
                ("out".to_string(), Some(Hsla::blue())),
                ("⋯".to_string(), None),
                ("  fn ".to_string(), Some(Hsla::red())),
                ("inner".to_string(), Some(Hsla::blue())),
                ("() {}\n}".to_string(), Some(Hsla::red())),
            ]
        );
    }

    #[gpui::test]
    async fn test_highlighted_chunks_in_range_masks_redacted_text(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let long_len = MAX_LINE_LEN * 2;
        let buffer = cx.new(|cx| Buffer::local(format!("{}\nsecret", "x".repeat(long_len)), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Helvetica"),
                px(14.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        let snapshot = cx.update(|cx| {
            map.update(cx, |map, cx| {
                map.masked = true;
                map.snapshot(cx)
            })
        });
        let style = EditorStyle::default();
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: false,
        };
        let chunks_text = |range: Range<DisplayPoint>| {
            snapshot
                .highlighted_chunks_in_range(range, language_aware, &style)
                .map(|chunk| chunk.text)
                .collect::<String>()
        };

        let window_start = long_len as u32 - 200;
        assert_eq!(
            chunks_text(
                DisplayPoint::new(DisplayRow(0), window_start)
                    ..DisplayPoint::new(DisplayRow(0), window_start + 10)
            ),
            "*".repeat(10)
        );
        assert_eq!(
            chunks_text(
                DisplayPoint::new(DisplayRow(0), long_len as u32 - 2)
                    ..DisplayPoint::new(DisplayRow(1), 3)
            ),
            "**\n***"
        );
    }

    #[gpui::test]
    async fn test_highlighted_chunks_in_range_keeps_exact_endpoints(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            init_test(cx, &|settings| {
                settings.project.all_languages.defaults.tab_size = NonZeroU32::new(128);
            })
        });
        let style = EditorStyle::default();
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: false,
        };

        let text = format!("{}\t{}", "x".repeat(79), "y".repeat(3_000));
        let snapshot = build_snapshot(&text, cx);
        assert_eq!(snapshot.line_len(DisplayRow(0)), 79 + 49 + 3_000);
        let chunks_text = |snapshot: &DisplaySnapshot, columns: Range<u32>| {
            snapshot
                .highlighted_chunks_in_range(
                    DisplayPoint::new(DisplayRow(0), columns.start)
                        ..DisplayPoint::new(DisplayRow(0), columns.end),
                    language_aware,
                    &style,
                )
                .map(|chunk| chunk.text)
                .collect::<String>()
        };
        assert_eq!(
            chunks_text(&snapshot, 100..350),
            format!("{}{}", " ".repeat(28), "y".repeat(222))
        );
        assert_eq!(
            chunks_text(&snapshot, 50..100),
            format!("{}{}", "x".repeat(29), " ".repeat(21))
        );
        assert_eq!(chunks_text(&snapshot, 90..110), " ".repeat(20));

        let text = format!("{}Z", "x".repeat(2_047));
        let mut snapshot = build_snapshot(&text, cx);
        snapshot.clip_at_line_ends = true;
        assert_eq!(
            chunks_text(&snapshot, 1_798..2_048),
            format!("{}Z", "x".repeat(249))
        );
    }

    #[gpui::test]
    async fn test_is_windowed_row_requires_exact_grid(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));
        let monospace = GridCell {
            width: px(10.),
            monospace: true,
        };
        let proportional = GridCell {
            width: px(10.),
            monospace: false,
        };

        let long_len = MAX_LINE_LEN * 2;
        let text = format!(
            "{}\n{}\t{}\n{}\nshort\n{}\n{}\n{}",
            "x".repeat(long_len),
            "x".repeat(30),
            "y".repeat(long_len),
            "é".repeat(long_len),
            "f".repeat(long_len),
            "r".repeat(long_len),
            "c".repeat(long_len)
        );
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                test_font(),
                px(14.),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        map.update(cx, |map, cx| {
            let fold_row_start = text.find("fff").unwrap();
            map.fold(
                vec![Crease::simple(
                    buffer_snapshot.offset_to_point(MultiBufferOffset(fold_row_start + 10))
                        ..buffer_snapshot.offset_to_point(MultiBufferOffset(fold_row_start + 20)),
                    FoldPlaceholder::test(),
                )],
                cx,
            );
            let repl_row_start = text.find("rrr").unwrap();
            map.splice_inlays(
                &[],
                vec![
                    Inlay::mock_hint(
                        0,
                        buffer_snapshot.anchor_after(MultiBufferOffset(5)),
                        "hint",
                    ),
                    Inlay::repl_result(
                        1,
                        buffer_snapshot.anchor_after(MultiBufferOffset(repl_row_start + 5)),
                        "result",
                    ),
                    Inlay::mock_hint(
                        2,
                        buffer_snapshot
                            .anchor_after(MultiBufferOffset(text.find("ccc").unwrap() + 5)),
                        "\u{2}",
                    ),
                ],
                cx,
            );
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));

        assert!(snapshot.is_windowed_row(DisplayRow(0), monospace));
        assert!(!snapshot.is_windowed_row(DisplayRow(0), proportional));
        assert!(snapshot.is_windowed_row(DisplayRow(1), monospace));
        assert!(!snapshot.is_windowed_row(DisplayRow(2), monospace));
        assert!(!snapshot.is_windowed_row(DisplayRow(3), monospace));
        assert!(!snapshot.is_windowed_row(DisplayRow(4), monospace));
        assert!(!snapshot.is_windowed_row(DisplayRow(5), monospace));

        assert!(snapshot.is_long_unwrapped_row(DisplayRow(6)));
        assert!(!snapshot.is_windowed_row(DisplayRow(6), monospace));

        let mut masked = snapshot;
        masked.masked = true;
        assert!(!masked.is_long_unwrapped_row(DisplayRow(0)));
        assert!(!masked.is_long_unwrapped_row(DisplayRow(2)));
    }

    #[gpui::test]
    async fn test_grapheme_at_returns_whole_graphemes(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let long_cluster = format!("a{}", "\u{301}".repeat(64));
        let text = format!(
            "{long_cluster}{}\ne\u{301}🇺🇸🇺🇸x \u{1}",
            "漢".repeat(MAX_LINE_LEN * 2)
        );
        let snapshot = build_snapshot(&text, cx);
        let grapheme_at = |row: u32, column: usize| {
            snapshot.grapheme_at(DisplayPoint::new(DisplayRow(row), column as u32))
        };

        assert_eq!(grapheme_at(0, 0), Some(long_cluster.as_str().into()));
        assert_eq!(grapheme_at(0, long_cluster.len()), Some("漢".into()));
        assert_eq!(
            grapheme_at(0, long_cluster.len() + 3 * 1_000),
            Some("漢".into())
        );
        assert_eq!(grapheme_at(0, text.find('\n').unwrap()), Some(" ".into()));
        assert_eq!(grapheme_at(1, 0), Some("e\u{301}".into()));
        assert_eq!(grapheme_at(1, 3), Some("🇺🇸".into()));
        assert_eq!(grapheme_at(1, 11), Some("🇺🇸".into()));
        assert_eq!(grapheme_at(1, 19), Some("x".into()));
        assert_eq!(grapheme_at(1, 20), Some(" ".into()));
        assert_eq!(grapheme_at(1, 21), Some("␁".into()));
        assert_eq!(grapheme_at(1, 22), None);
    }

    #[gpui::test]
    async fn test_control_characters_disable_the_exact_grid(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));
        let monospace = GridCell {
            width: px(10.),
            monospace: true,
        };

        let text = format!("{}\u{1}{}", "x".repeat(1_000), "x".repeat(3_000));
        let snapshot = build_snapshot(&text, cx);
        assert!(snapshot.is_long_unwrapped_row(DisplayRow(0)));
        assert!(!snapshot.is_windowed_row(DisplayRow(0), monospace));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        cx.set_state(&format!("ˇ{}\nshort", "x".repeat(MAX_LINE_LEN * 2)));
        let is_grid = |cx: &mut crate::test::editor_test_context::EditorTestContext| {
            cx.update_editor(|editor, window, cx| {
                editor
                    .snapshot(window, cx)
                    .is_windowed_row(DisplayRow(0), monospace)
            })
        };
        assert!(is_grid(&mut cx));

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(1, 0)..Point::new(1, 0), "\u{7f}")], cx);
        });
        assert!(is_grid(&mut cx));

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(0, 5)..Point::new(0, 5), "\u{7f}")], cx);
        });
        assert!(!is_grid(&mut cx));

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(0, 5)..Point::new(0, 6), "")], cx);
        });
        assert!(is_grid(&mut cx));

        let many_controls = "\u{1}".repeat(MAX_TRACKED_CONTROL_CHARS + 1);
        cx.update_editor(|editor, _, cx| {
            editor.edit(
                [(Point::new(1, 0)..Point::new(1, 0), many_controls.as_str())],
                cx,
            );
        });
        assert!(!is_grid(&mut cx));
        cx.update_editor(|editor, _, cx| {
            editor.edit(
                [(
                    Point::new(1, 0)..Point::new(1, (MAX_TRACKED_CONTROL_CHARS / 2 + 1) as u32),
                    "",
                )],
                cx,
            );
        });
        assert!(!is_grid(&mut cx));
        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(1, 0)..Point::new(1, 1), "")], cx);
        });
        assert!(is_grid(&mut cx));
    }

    #[gpui::test]
    async fn test_ruler_uses_measured_inlay_renderer_widths(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        cx.set_state(&format!("ˇ{}", "漢".repeat(MAX_LINE_LEN)));
        let inlay_id = InlayId::ReplResult(7);
        cx.update_editor(|editor, _, cx| {
            let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            editor.display_map.update(cx, |map, cx| {
                map.splice_inlays(
                    &[],
                    vec![Inlay::repl_result(
                        7,
                        buffer_snapshot.anchor_after(MultiBufferOffset(3)),
                        "x",
                    )],
                    cx,
                );
            });
        });
        let ruler_width = |cx: &mut crate::test::editor_test_context::EditorTestContext| {
            cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx).display_snapshot;
                let details = editor.text_layout_details(window, cx);
                let cell = details.grid_cell();
                assert!(!snapshot.is_windowed_row(DisplayRow(0), cell));
                let layout = snapshot.layout_row(DisplayRow(0), &details);
                (
                    layout.width(),
                    ScrollPixelOffset::from(cell.width),
                    details.ruler_shaper(&snapshot, DisplayRow(0)),
                )
            })
        };
        let update_widths = |cx: &mut crate::test::editor_test_context::EditorTestContext,
                             width: Pixels,
                             key: u64| {
            cx.update_editor(|editor, _, cx| {
                editor.display_map.update(cx, |map, cx| {
                    map.update_fold_widths([(ChunkRendererId::Inlay(inlay_id), width)], key, cx)
                })
            })
        };

        let (guessed_width, cell_width, shaper) = ruler_width(&mut cx);
        let key = renderer_metrics_key(&shaper.style.text.font(), shaper.font_size);
        assert!(update_widths(&mut cx, px(200.), key));
        let (measured_width, ..) = ruler_width(&mut cx);
        assert!(((measured_width - guessed_width) - (200. - cell_width)).abs() < 0.01);
        assert!(!update_widths(&mut cx, px(200.), key));

        let zoomed_key = renderer_metrics_key(&shaper.style.text.font(), shaper.font_size * 2.);
        assert!(update_widths(&mut cx, px(400.), zoomed_key));
        let (stale_width, ..) = ruler_width(&mut cx);
        assert!((stale_width - guessed_width).abs() < 0.01);
        assert!(update_widths(&mut cx, px(200.), key));
        let (remeasured_width, ..) = ruler_width(&mut cx);
        assert!((remeasured_width - measured_width).abs() < 0.01);
    }

    #[gpui::test]
    async fn test_rulers_of_inlay_rows_are_never_retained_for_each_other(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        cx.set_state("ˇ\ncd");
        let inlay_text = format!("p\n{}\n{}\nr", "漢".repeat(2_000), "🙂".repeat(2_000));
        cx.update_editor(|editor, _, cx| {
            let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            editor.display_map.update(cx, |map, cx| {
                map.splice_inlays(
                    &[],
                    vec![Inlay::mock_hint(
                        0,
                        buffer_snapshot.anchor_after(MultiBufferOffset(0)),
                        inlay_text.as_str(),
                    )],
                    cx,
                );
            });
        });
        let rulers_for = |cx: &mut crate::test::editor_test_context::EditorTestContext| {
            cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx).display_snapshot;
                let details = editor.text_layout_details(window, cx);
                [DisplayRow(1), DisplayRow(2)].map(|row| {
                    let ruler = snapshot
                        .ruled_row(row, details.ruler_shaper(&snapshot, row))
                        .ruler;
                    assert_eq!(ruler.len(), snapshot.line_len(row));
                    ruler
                })
            })
        };
        let [first, second] = rulers_for(&mut cx);
        assert_eq!(first.len(), 6_000);
        assert_eq!(second.len(), 8_000);

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(1, 0)..Point::new(1, 0), "x")], cx);
        });
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx).display_snapshot;
            for (wrap_row, ruler) in snapshot.row_rulers.cached_rulers() {
                assert_eq!(
                    ruler.len(),
                    snapshot.line_len(DisplayRow(wrap_row)),
                    "cached ruler for row {wrap_row} has a foreign length"
                );
            }
        });
        let [first_again, second_again] = rulers_for(&mut cx);
        assert_eq!(first_again.len(), 6_000);
        assert_eq!(second_again.len(), 8_000);
        assert!(!Arc::ptr_eq(&first, &first_again));
        assert!(!Arc::ptr_eq(&second, &second_again));
    }

    #[gpui::test]
    async fn test_windowed_rows_inside_multiline_inlays(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        cx.set_state("aˇb");
        let inlay_text = format!("p\n{}\nr", "q".repeat(2_048));
        cx.update_editor(|editor, _, cx| {
            let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            editor.display_map.update(cx, |map, cx| {
                map.splice_inlays(
                    &[],
                    vec![Inlay::mock_hint(
                        0,
                        buffer_snapshot.anchor_after(MultiBufferOffset(1)),
                        inlay_text.as_str(),
                    )],
                    cx,
                );
            });
        });
        let (snapshot, details) = cx.update_editor(|editor, window, cx| {
            editor.set_visible_column_count(100.);
            editor.set_scroll_position(gpui::point(200., 0.), window, cx);
            (
                editor.snapshot(window, cx).display_snapshot,
                editor.text_layout_details(window, cx),
            )
        });
        assert_eq!(snapshot.text(), format!("ap\n{}\nrb", "q".repeat(2_048)));
        let cell = details.grid_cell();
        assert!(cell.monospace);
        assert!(!snapshot.is_windowed_row(DisplayRow(0), cell));
        assert!(snapshot.is_windowed_row(DisplayRow(1), cell));
        assert!(!snapshot.is_windowed_row(DisplayRow(2), cell));

        let cell_width = ScrollPixelOffset::from(cell.width);
        let layout = snapshot.layout_row(DisplayRow(1), &details);
        assert!(matches!(layout, RowLayout::Windowed { .. }));
        assert_eq!(layout.x_for_index(50), cell_width * 50.);
        assert!((layout.x_for_index(250) - cell_width * 250.).abs() < 0.01);
        assert_eq!(layout.closest_index_for_x(cell_width * 250.), 250);
        assert_eq!(layout.closest_index_for_x(cell_width * 2_000.), 2_000);
        assert_eq!(layout.width(), cell_width * 2_048.);

        let style = EditorStyle::default();
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: false,
        };
        let chunks_text = snapshot
            .highlighted_chunks_in_range(
                DisplayPoint::new(DisplayRow(1), 100)..DisplayPoint::new(DisplayRow(1), 350),
                language_aware,
                &style,
            )
            .map(|chunk| chunk.text)
            .collect::<String>();
        assert_eq!(chunks_text, "q".repeat(250));
    }

    #[gpui::test]
    async fn test_shaping_window_selection(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let cell = GridCell {
            width: px(10.),
            monospace: true,
        };
        let viewport = |scroll_columns: f64| HorizontalViewport {
            scroll_columns,
            visible_columns: 100.,
            text_align: TextAlign::Left,
            content_width: px(600.),
        };

        let centered = HorizontalViewport {
            text_align: TextAlign::Center,
            ..viewport(0.)
        };
        assert_eq!(centered.aligned(40_000., cell).scroll_columns, 1_970.);
        assert_eq!(
            centered.aligned(40_000., cell).shaping_window(4_000),
            1_900..2_150
        );
        let right_aligned = HorizontalViewport {
            text_align: TextAlign::Right,
            ..viewport(0.)
        };
        assert_eq!(right_aligned.aligned(40_000., cell).scroll_columns, 3_940.);
        assert_eq!(viewport(0.).aligned(40_000., cell), viewport(0.));

        assert_eq!(viewport(0.).shaping_window(10_000), 0..250);
        assert_eq!(viewport(120.).shaping_window(10_000), 0..250);
        assert_eq!(viewport(170.).shaping_window(10_000), 100..350);
        assert_eq!(viewport(9_950.).shaping_window(10_000), 9_750..10_000);
        assert_eq!(viewport(20_000.).shaping_window(10_000), 9_750..10_000);
        assert_eq!(viewport(170.).shaping_window(300), 50..300);

        let window = viewport(170.).shaping_window(10_000);
        assert!(cell.fits(&window, px(2_500.)));
        assert!(cell.fits(&window, px(2_500.4)));
        assert!(!cell.fits(&window, px(2_501.)));
        assert!(!cell.fits(&window, px(2_490.)));
    }

    #[gpui::test]
    async fn test_windowed_row_layout_positions(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let cell = GridCell {
            width: px(10.),
            monospace: true,
        };
        let geometry = WindowedRowGeometry::new(10_000, cell, 2_000..2_300);
        let glyphs = (0..300)
            .map(|glyph_index| gpui::ShapedGlyph {
                id: gpui::GlyphId(1),
                position: gpui::point(px(10. * glyph_index as f32), px(0.)),
                index: glyph_index,
                is_emoji: false,
            })
            .collect::<Vec<_>>();
        let shaped = Arc::new(LineLayout {
            width: px(3_000.),
            len: 300,
            runs: vec![gpui::ShapedRun {
                font_id: gpui::FontId(0),
                glyphs,
            }],
            ..LineLayout::default()
        });
        let layout = RowLayout::Windowed { geometry, shaped };

        assert_eq!(layout.x_for_index(0), 0.);
        assert_eq!(layout.x_for_index(1_000), 10_000.);
        assert_eq!(layout.x_for_index(2_000), 20_000.);
        assert_eq!(layout.x_for_index(2_150), 21_500.);
        assert_eq!(layout.x_for_index(2_300), 23_000.);
        assert_eq!(layout.x_for_index(8_000), 80_000.);
        assert_eq!(layout.x_for_index(10_000), 100_000.);
        assert_eq!(layout.width(), 100_000.);

        assert_eq!(layout.closest_index_for_x(-5.), 0);
        assert_eq!(layout.closest_index_for_x(10_004.), 1_000);
        assert_eq!(layout.closest_index_for_x(10_006.), 1_001);
        assert_eq!(layout.closest_index_for_x(20_000.), 2_000);
        assert_eq!(layout.closest_index_for_x(21_504.), 2_150);
        assert_eq!(layout.closest_index_for_x(43_000.), 4_300);
        assert_eq!(layout.closest_index_for_x(1_000_000.), 10_000);
    }

    #[gpui::test]
    async fn test_vertical_movement_on_long_rows_is_scroll_invariant(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        let ascii_len = MAX_LINE_LEN * 3;
        let wide_len = MAX_LINE_LEN * 2;
        cx.set_state(&format!(
            "ˇ{}\nx\n{}\nx",
            "a".repeat(ascii_len),
            "🙂".repeat(wide_len)
        ));

        for (row, column) in [
            (0, ascii_len as u32 - 100),
            (0, 1_500),
            (2, ('🙂'.len_utf8() * (wide_len - 100)) as u32),
            (2, '🙂'.len_utf8() as u32 * 700),
        ] {
            let start = DisplayPoint::new(DisplayRow(row), column);
            let (below, goal) = cx.update_editor(|editor, window, cx| {
                editor.set_visible_column_count(100.);
                editor.set_scroll_position(gpui::point(column as f64 - 40., 0.), window, cx);
                let snapshot = editor.snapshot(window, cx);
                let details = editor.text_layout_details(window, cx);
                movement::down(
                    &snapshot,
                    start,
                    language::SelectionGoal::None,
                    false,
                    &details,
                )
            });
            assert_eq!(below, DisplayPoint::new(DisplayRow(row + 1), 1));

            for scroll_columns in [0., 20., 700., 3_000., 5_000.] {
                let (back, _) = cx.update_editor(|editor, window, cx| {
                    editor.set_scroll_position(gpui::point(scroll_columns, 0.), window, cx);
                    let snapshot = editor.snapshot(window, cx);
                    let details = editor.text_layout_details(window, cx);
                    movement::up(&snapshot, below, goal, false, &details)
                });
                assert_eq!(
                    back, start,
                    "moving up from {below:?} with the viewport at column {scroll_columns}"
                );
            }
        }
    }

    fn build_snapshot(text: &str, cx: &mut gpui::TestAppContext) -> DisplaySnapshot {
        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                test_font(),
                px(14.),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        map.update(cx, |map, cx| map.snapshot(cx))
    }

    #[gpui::test]
    async fn test_masked_text_chunks_preserve_newlines(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let buffer = cx.new(|cx| Buffer::local("secret\nwörds\nhere", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Helvetica"),
                px(14.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        let snapshot = cx.update(|cx| {
            map.update(cx, |map, cx| {
                map.masked = true;
                map.snapshot(cx)
            })
        });

        assert_eq!(
            snapshot.text_chunks(DisplayRow(0)).collect::<String>(),
            "******\n*****\n****"
        );
        assert_eq!(
            snapshot
                .reverse_text_chunks(DisplayRow(2))
                .collect::<String>(),
            "****\n*****\n******"
        );
        assert_eq!(
            snapshot
                .chunks(
                    DisplayRow(0)..DisplayRow(3),
                    LanguageAwareStyling {
                        tree_sitter: false,
                        diagnostics: false,
                    },
                    HighlightStyles::default(),
                )
                .map(|chunk| chunk.text)
                .collect::<String>(),
            "******\n*****\n****"
        );
    }

    #[test]
    fn test_mask_chunks_splits_chunks_longer_than_bullets() {
        let first_len = BULLETS.len() + 22;
        let second_len = BULLETS.len() * 2 + 44;
        let text = format!("{}\n{}", "α".repeat(first_len), "x".repeat(second_len));
        let chunk = Chunk {
            text: &text,
            ..Chunk::default()
        };

        let masked = mask_chunks(std::iter::once(chunk)).collect::<Vec<_>>();

        for chunk in &masked {
            assert!(chunk.text.len() <= BULLETS.len());
            assert_eq!(
                chunk.chars,
                1u128.unbounded_shl(chunk.text.len() as u32).wrapping_sub(1)
            );
        }
        assert_eq!(
            masked.iter().map(|chunk| chunk.text).collect::<String>(),
            format!("{}\n{}", "*".repeat(first_len), "*".repeat(second_len))
        );
    }

    #[gpui::test]
    async fn test_columnar_selection_on_huge_unwrapped_line_uses_monospace_grid(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        let long_len = MAX_LINE_LEN * 2;
        cx.set_state(&format!(
            "ˇ{}\n{}",
            "x".repeat(long_len),
            "α".repeat(long_len)
        ));

        cx.update_editor(|editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window, cx);
            let snapshot = editor.snapshot(window, cx);
            let cell_width = snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(0), 1), &text_layout_details);
            assert!(cell_width > 0.);

            let positions = cell_width * 100.0..cell_width * 200.0;
            let selection = editor
                .selections
                .build_columnar_selection(
                    &snapshot,
                    DisplayRow(0),
                    &positions,
                    false,
                    &text_layout_details,
                )
                .unwrap();
            assert_eq!(selection.start, Point::new(0, 100));
            assert_eq!(selection.end, Point::new(0, 200));

            let positions = cell_width * 101.0..cell_width * 200.0;
            let selection = editor
                .selections
                .build_columnar_selection(
                    &snapshot,
                    DisplayRow(1),
                    &positions,
                    false,
                    &text_layout_details,
                )
                .unwrap();
            assert_eq!(selection.start, Point::new(1, 101 * 'α'.len_utf8() as u32));
            assert_eq!(selection.end, Point::new(1, 200 * 'α'.len_utf8() as u32));
        });
    }

    #[gpui::test]
    async fn test_navigation_on_huge_unwrapped_lines_uses_monospace_grid(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        let long_len = MAX_LINE_LEN * 2;
        cx.set_state(&format!(
            "ˇ{}\n{}\nshort",
            "x".repeat(long_len),
            "α".repeat(long_len)
        ));

        cx.update_editor(|editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window, cx);
            let snapshot = editor.snapshot(window, cx);

            let column = (long_len - 10) as u32;
            let x = snapshot.x_for_display_point(
                DisplayPoint::new(DisplayRow(0), column),
                &text_layout_details,
            );
            assert!(x > 0.);
            assert_eq!(
                snapshot.display_column_for_x(DisplayRow(0), x, &text_layout_details),
                column
            );

            let odd_x = snapshot.x_for_display_point(
                DisplayPoint::new(DisplayRow(0), (long_len - 9) as u32),
                &text_layout_details,
            );
            assert_eq!(
                snapshot.display_column_for_x(DisplayRow(1), odd_x, &text_layout_details),
                ((long_len - 9) * 'α'.len_utf8()) as u32
            );
        });
    }

    #[gpui::test]
    async fn test_layout_row_shapes_long_rows_when_soft_wrapped(cx: &mut gpui::TestAppContext) {
        cx.background_executor
            .set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.update(|cx| init_test(cx, &|_| {}));

        let mut cx = crate::test::editor_test_context::EditorTestContext::new(cx).await;
        let editor = cx.editor.clone();
        let window = cx.window;

        cx.update_window(window, |_, window, cx| {
            let text_layout_details =
                editor.update(cx, |editor, cx| editor.text_layout_details(window, cx));

            let buffer = MultiBuffer::build_simple(&"x".repeat(MAX_LINE_LEN * 3), cx);
            let map = cx.new(|cx| {
                DisplayMap::new(
                    buffer,
                    font("Helvetica"),
                    px(14.0),
                    Some(px(12_000.0)),
                    1,
                    1,
                    FoldPlaceholder::test(),
                    DiagnosticSeverity::Warning,
                    cx,
                )
            });
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert!(snapshot.has_soft_wraps());
            assert!(snapshot.line_len(DisplayRow(0)) as usize > MAX_LINE_LEN);

            assert!(matches!(
                snapshot.layout_row(DisplayRow(0), &text_layout_details),
                RowLayout::Shaped(_)
            ));
        })
        .unwrap();
    }

    #[gpui::test]
    async fn test_masked_soft_wrapped_line_with_deep_indent(cx: &mut gpui::TestAppContext) {
        cx.background_executor
            .set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.update(|cx| init_test(cx, &|_| {}));

        let text = format!("{}{}", " ".repeat(200), "x".repeat(2000));
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Helvetica"),
                px(14.0),
                Some(px(3000.0)),
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        let snapshot = cx.update(|cx| {
            map.update(cx, |map, cx| {
                map.masked = true;
                map.snapshot(cx)
            })
        });
        assert!(snapshot.max_point().row().0 > 0);

        let masked_text = snapshot.text_chunks(DisplayRow(0)).collect::<String>();
        let unmasked_line_count = snapshot.max_point().row().0 as usize + 1;
        assert_eq!(masked_text.split('\n').count(), unmasked_line_count);
    }

    #[gpui::test]
    async fn test_chunks_with_syntax_highlighting_across_blocks(cx: &mut gpui::TestAppContext) {
        cx.background_executor
            .set_block_on_ticks(usize::MAX..=usize::MAX);

        let text = r#"
            const A: &str = "
                one
                two
                three
            ";
            const B: &str = "four";
        "#
        .unindent();

        let theme = SyntaxTheme::new_test(vec![
            ("string", Hsla::red()),
            ("punctuation", Hsla::blue()),
            ("keyword", Hsla::green()),
        ]);
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_highlights_query(
                r#"
                (string_literal) @string
                "const" @keyword
                [":" ";"] @punctuation
                "#,
            )
            .unwrap(),
        );
        language.set_theme(&theme);

        cx.update(|cx| init_test(cx, &|_| {}));

        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.condition(&buffer, |buf, _| !buf.is_parsing()).await;
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));

        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                px(16.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        // Insert two blocks in the middle of a multi-line string literal.
        // The second block has zero height.
        map.update(cx, |map, cx| {
            map.insert_blocks(
                [
                    BlockProperties {
                        placement: BlockPlacement::Below(
                            buffer_snapshot.anchor_before(Point::new(1, 0)),
                        ),
                        height: Some(1),
                        style: BlockStyle::Sticky,
                        render: Arc::new(|_| div().into_any()),
                        priority: 0,
                    },
                    BlockProperties {
                        placement: BlockPlacement::Below(
                            buffer_snapshot.anchor_before(Point::new(2, 0)),
                        ),
                        height: None,
                        style: BlockStyle::Sticky,
                        render: Arc::new(|_| div().into_any()),
                        priority: 0,
                    },
                ],
                cx,
            )
        });

        pretty_assertions::assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(0)..DisplayRow(7), &map, &theme, cx)),
            [
                ("const".into(), Some(Hsla::green())),
                (" A".into(), None),
                (":".into(), Some(Hsla::blue())),
                (" &str = ".into(), None),
                ("\"\n    one\n".into(), Some(Hsla::red())),
                ("\n".into(), None),
                ("    two\n    three\n\"".into(), Some(Hsla::red())),
                (";".into(), Some(Hsla::blue())),
                ("\n".into(), None),
                ("const".into(), Some(Hsla::green())),
                (" B".into(), None),
                (":".into(), Some(Hsla::blue())),
                (" &str = ".into(), None),
                ("\"four\"".into(), Some(Hsla::red())),
                (";".into(), Some(Hsla::blue())),
                ("\n".into(), None),
            ]
        );
    }

    #[gpui::test]
    async fn test_cropped_chunks_inherit_inlay_diagnostics_and_rulers_follow_severity(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| init_test(cx, &|_| {}));
        let text = format!("To{}", "x".repeat(3_000));
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        buffer.update(cx, |buffer, cx| {
            buffer.update_diagnostics(
                LanguageServerId(0),
                DiagnosticSet::new(
                    [DiagnosticEntry::new(
                        PointUtf16::new(0, 0)..PointUtf16::new(0, 1),
                        Diagnostic {
                            severity: lsp::DiagnosticSeverity::ERROR,
                            group_id: 1,
                            message: "hi".into(),
                            ..Default::default()
                        },
                    )],
                    buffer,
                ),
                cx,
            )
        });
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                px(16.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        let hint = "h".repeat(400);
        map.update(cx, |map, cx| {
            map.splice_inlays(
                &[],
                vec![Inlay::mock_hint(
                    0,
                    buffer_snapshot.anchor_before(MultiBufferOffset(1)),
                    hint.as_str(),
                )],
                cx,
            );
        });
        let style = EditorStyle::default();
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: true,
        };
        let underlined = |snapshot: &DisplaySnapshot, columns: Range<u32>| {
            snapshot
                .highlighted_chunks_in_range(
                    DisplayPoint::new(DisplayRow(0), columns.start)
                        ..DisplayPoint::new(DisplayRow(0), columns.end),
                    language_aware,
                    &style,
                )
                .filter(|chunk| chunk.diagnostic_underline_severity.is_some())
                .map(|chunk| chunk.text.len())
                .sum::<usize>()
        };
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(underlined(&snapshot, 0..401), 401);
        assert_eq!(underlined(&snapshot, 100..350), 250);
        assert_eq!(underlined(&snapshot, 401..500), 0);
        let with_diagnostics = snapshot.row_rulers;

        map.update(cx, |map, _| {
            map.diagnostics_max_severity = DiagnosticSeverity::Off;
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(underlined(&snapshot, 100..350), 0);
        assert!(!Arc::ptr_eq(&with_diagnostics, &snapshot.row_rulers));
    }

    #[gpui::test]
    async fn test_cropped_inlays_after_a_fold_do_not_inherit_folded_diagnostics(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| init_test(cx, &|_| {}));
        let buffer = cx.new(|cx| Buffer::local("aBADz", cx));
        buffer.update(cx, |buffer, cx| {
            buffer.update_diagnostics(
                LanguageServerId(0),
                DiagnosticSet::new(
                    [DiagnosticEntry::new(
                        PointUtf16::new(0, 1)..PointUtf16::new(0, 4),
                        Diagnostic {
                            severity: lsp::DiagnosticSeverity::ERROR,
                            group_id: 1,
                            message: "hi".into(),
                            ..Default::default()
                        },
                    )],
                    buffer,
                ),
                cx,
            )
        });
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                px(16.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        let hint = "h".repeat(4_096);
        map.update(cx, |map, cx| {
            map.splice_inlays(
                &[],
                vec![Inlay::mock_hint(
                    0,
                    buffer_snapshot.anchor_after(MultiBufferOffset(4)),
                    hint.as_str(),
                )],
                cx,
            );
        });
        let style = EditorStyle::default();
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: true,
        };
        let underlined = |snapshot: &DisplaySnapshot, columns: Range<u32>| {
            snapshot
                .highlighted_chunks_in_range(
                    DisplayPoint::new(DisplayRow(0), columns.start)
                        ..DisplayPoint::new(DisplayRow(0), columns.end),
                    language_aware,
                    &style,
                )
                .filter(|chunk| chunk.diagnostic_underline_severity.is_some())
                .map(|chunk| chunk.text.len())
                .sum::<usize>()
        };

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.text(), format!("aBAD{hint}z"));
        assert_eq!(underlined(&snapshot, 0..4_100), 3 + 4_096);
        assert_eq!(underlined(&snapshot, 100..350), 250);

        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    MultiBufferPoint::new(0, 1)..MultiBufferPoint::new(0, 4),
                    FoldPlaceholder::test(),
                )],
                cx,
            )
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.text(), format!("a⋯{hint}z"));
        let hint_start = "a⋯".len() as u32;
        assert_eq!(underlined(&snapshot, 0..hint_start + 4_096), 0);
        assert_eq!(underlined(&snapshot, hint_start + 100..hint_start + 350), 0);
        assert_eq!(underlined(&snapshot, hint_start..hint_start + 4_096), 0);
    }

    #[gpui::test]
    async fn test_chunks_with_diagnostics_across_blocks(cx: &mut gpui::TestAppContext) {
        cx.background_executor
            .set_block_on_ticks(usize::MAX..=usize::MAX);

        let text = r#"
            struct A {
                b: usize;
            }
            const c: usize = 1;
        "#
        .unindent();

        cx.update(|cx| init_test(cx, &|_| {}));

        let buffer = cx.new(|cx| Buffer::local(text, cx));

        buffer.update(cx, |buffer, cx| {
            buffer.update_diagnostics(
                LanguageServerId(0),
                DiagnosticSet::new(
                    [DiagnosticEntry::new(
                        PointUtf16::new(0, 0)..PointUtf16::new(2, 1),
                        Diagnostic {
                            severity: lsp::DiagnosticSeverity::ERROR,
                            group_id: 1,
                            message: "hi".into(),
                            ..Default::default()
                        },
                    )],
                    buffer,
                ),
                cx,
            )
        });

        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));

        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                px(16.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        let black = gpui::black().to_rgb();
        let red = gpui::red().to_rgb();

        // Insert a block in the middle of a multi-line diagnostic.
        map.update(cx, |map, cx| {
            map.highlight_text(
                HighlightKey::Editor,
                vec![
                    buffer_snapshot.anchor_before(Point::new(3, 9))
                        ..buffer_snapshot.anchor_after(Point::new(3, 14)),
                    buffer_snapshot.anchor_before(Point::new(3, 17))
                        ..buffer_snapshot.anchor_after(Point::new(3, 18)),
                ],
                red.into(),
                false,
                cx,
            );
            map.insert_blocks(
                [BlockProperties {
                    placement: BlockPlacement::Below(
                        buffer_snapshot.anchor_before(Point::new(1, 0)),
                    ),
                    height: Some(1),
                    style: BlockStyle::Sticky,
                    render: Arc::new(|_| div().into_any()),
                    priority: 0,
                }],
                cx,
            )
        });

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut chunks = Vec::<(String, Option<lsp::DiagnosticSeverity>, Rgba)>::new();
        for chunk in snapshot.chunks(
            DisplayRow(0)..DisplayRow(5),
            LanguageAwareStyling {
                tree_sitter: true,
                diagnostics: true,
            },
            Default::default(),
        ) {
            let color = chunk
                .highlight_style
                .and_then(|style| style.color)
                .map_or(black, |color| color.to_rgb());
            if let Some((last_chunk, last_severity, last_color)) = chunks.last_mut()
                && *last_severity == chunk.diagnostic_severity
                && *last_color == color
            {
                last_chunk.push_str(chunk.text);
                continue;
            }

            chunks.push((chunk.text.to_string(), chunk.diagnostic_severity, color));
        }

        assert_eq!(
            chunks,
            [
                (
                    "struct A {\n    b: usize;\n".into(),
                    Some(lsp::DiagnosticSeverity::ERROR),
                    black
                ),
                ("\n".into(), None, black),
                ("}".into(), Some(lsp::DiagnosticSeverity::ERROR), black),
                ("\nconst c: ".into(), None, black),
                ("usize".into(), None, red),
                (" = ".into(), None, black),
                ("1".into(), None, red),
                (";\n".into(), None, black),
            ]
        );
    }

    #[gpui::test]
    async fn test_point_translation_with_replace_blocks(cx: &mut gpui::TestAppContext) {
        cx.background_executor
            .set_block_on_ticks(usize::MAX..=usize::MAX);

        cx.update(|cx| init_test(cx, &|_| {}));

        let buffer = cx.update(|cx| MultiBuffer::build_simple("abcde\nfghij\nklmno\npqrst", cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Courier"),
                px(16.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        let snapshot = map.update(cx, |map, cx| {
            map.insert_blocks(
                [BlockProperties {
                    placement: BlockPlacement::Replace(
                        buffer_snapshot.anchor_before(Point::new(1, 2))
                            ..=buffer_snapshot.anchor_after(Point::new(2, 3)),
                    ),
                    height: Some(4),
                    style: BlockStyle::Fixed,
                    render: Arc::new(|_| div().into_any()),
                    priority: 0,
                }],
                cx,
            );
            map.snapshot(cx)
        });

        assert_eq!(snapshot.text(), "abcde\n\n\n\n\npqrst");

        let point_to_display_points = [
            (Point::new(1, 0), DisplayPoint::new(DisplayRow(1), 0)),
            (Point::new(2, 0), DisplayPoint::new(DisplayRow(1), 0)),
            (Point::new(3, 0), DisplayPoint::new(DisplayRow(5), 0)),
        ];
        for (buffer_point, display_point) in point_to_display_points {
            assert_eq!(
                snapshot.point_to_display_point(buffer_point, Bias::Left),
                display_point,
                "point_to_display_point({:?}, Bias::Left)",
                buffer_point
            );
            assert_eq!(
                snapshot.point_to_display_point(buffer_point, Bias::Right),
                display_point,
                "point_to_display_point({:?}, Bias::Right)",
                buffer_point
            );
        }

        let display_points_to_points = [
            (
                DisplayPoint::new(DisplayRow(1), 0),
                Point::new(1, 0),
                Point::new(2, 5),
            ),
            (
                DisplayPoint::new(DisplayRow(2), 0),
                Point::new(1, 0),
                Point::new(2, 5),
            ),
            (
                DisplayPoint::new(DisplayRow(3), 0),
                Point::new(1, 0),
                Point::new(2, 5),
            ),
            (
                DisplayPoint::new(DisplayRow(4), 0),
                Point::new(1, 0),
                Point::new(2, 5),
            ),
            (
                DisplayPoint::new(DisplayRow(5), 0),
                Point::new(3, 0),
                Point::new(3, 0),
            ),
        ];
        for (display_point, left_buffer_point, right_buffer_point) in display_points_to_points {
            assert_eq!(
                snapshot.display_point_to_point(display_point, Bias::Left),
                left_buffer_point,
                "display_point_to_point({:?}, Bias::Left)",
                display_point
            );
            assert_eq!(
                snapshot.display_point_to_point(display_point, Bias::Right),
                right_buffer_point,
                "display_point_to_point({:?}, Bias::Right)",
                display_point
            );
        }
    }

    #[gpui::test]
    async fn test_chunks_with_soft_wrapping(cx: &mut gpui::TestAppContext) {
        cx.background_executor
            .set_block_on_ticks(usize::MAX..=usize::MAX);

        let text = r#"
            fn outer() {}

            mod module {
                fn inner() {}
            }"#
        .unindent();

        let theme =
            SyntaxTheme::new_test(vec![("mod.body", Hsla::red()), ("fn.name", Hsla::blue())]);
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Test".into(),
                    matcher: (LanguageMatcher {
                        path_suffixes: vec![".test".to_string()],
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_highlights_query(
                r#"
                (mod_item name: (identifier) body: _ @mod.body)
                (function_item name: (identifier) @fn.name)
                "#,
            )
            .unwrap(),
        );
        language.set_theme(&theme);

        cx.update(|cx| init_test(cx, &|_| {}));

        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.condition(&buffer, |buf, _| !buf.is_parsing()).await;
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let font_size = px(16.0);

        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                font_size,
                Some(px(40.0)),
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(0)..DisplayRow(5), &map, &theme, cx)),
            [
                ("fn \n".to_string(), None),
                ("oute".to_string(), Some(Hsla::blue())),
                ("\n".to_string(), None),
                ("r".to_string(), Some(Hsla::blue())),
                ("() \n{}\n\n".to_string(), None),
            ]
        );
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(3)..DisplayRow(5), &map, &theme, cx)),
            [("{}\n\n".to_string(), None)]
        );

        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    MultiBufferPoint::new(0, 6)..MultiBufferPoint::new(3, 2),
                    FoldPlaceholder::test(),
                )],
                cx,
            )
        });
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(1)..DisplayRow(4), &map, &theme, cx)),
            [
                ("out".to_string(), Some(Hsla::blue())),
                ("⋯\n".to_string(), None),
                ("  ".to_string(), Some(Hsla::red())),
                ("\n".to_string(), None),
                ("fn ".to_string(), Some(Hsla::red())),
                ("i".to_string(), Some(Hsla::blue())),
                ("\n".to_string(), None)
            ]
        );
    }

    #[gpui::test]
    async fn test_chunks_with_text_highlights(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let theme =
            SyntaxTheme::new_test(vec![("operator", Hsla::red()), ("string", Hsla::green())]);
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Test".into(),
                    matcher: (LanguageMatcher {
                        path_suffixes: vec![".test".to_string()],
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_highlights_query(
                r#"
                ":" @operator
                (string_literal) @string
                "#,
            )
            .unwrap(),
        );
        language.set_theme(&theme);

        let (text, highlighted_ranges) = marked_text_ranges(r#"constˇ «a»«:» B = "c «d»""#, false);

        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.condition(&buffer, |buf, _| !buf.is_parsing()).await;

        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));

        let font_size = px(16.0);
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        let style = HighlightStyle {
            color: Some(Hsla::blue()),
            ..Default::default()
        };

        map.update(cx, |map, cx| {
            map.highlight_text(
                HighlightKey::Editor,
                highlighted_ranges
                    .into_iter()
                    .map(|range| MultiBufferOffset(range.start)..MultiBufferOffset(range.end))
                    .map(|range| {
                        buffer_snapshot.anchor_before(range.start)
                            ..buffer_snapshot.anchor_before(range.end)
                    })
                    .collect(),
                style,
                false,
                cx,
            );
        });

        assert_eq!(
            cx.update(|cx| chunks(DisplayRow(0)..DisplayRow(10), &map, &theme, cx)),
            [
                ("const ".to_string(), None, None),
                ("a".to_string(), None, Some(Hsla::blue())),
                (":".to_string(), Some(Hsla::red()), Some(Hsla::blue())),
                (" B = ".to_string(), None, None),
                ("\"c ".to_string(), Some(Hsla::green()), None),
                ("d".to_string(), Some(Hsla::green()), Some(Hsla::blue())),
                ("\"".to_string(), Some(Hsla::green()), None),
            ]
        );
    }

    #[gpui::test]
    fn test_clip_point(cx: &mut gpui::App) {
        init_test(cx, &|_| {});

        fn assert(text: &str, shift_right: bool, bias: Bias, cx: &mut gpui::App) {
            let (unmarked_snapshot, mut markers) = marked_display_snapshot(text, cx);

            match bias {
                Bias::Left => {
                    if shift_right {
                        *markers[1].column_mut() += 1;
                    }

                    assert_eq!(unmarked_snapshot.clip_point(markers[1], bias), markers[0])
                }
                Bias::Right => {
                    if shift_right {
                        *markers[0].column_mut() += 1;
                    }

                    assert_eq!(unmarked_snapshot.clip_point(markers[0], bias), markers[1])
                }
            };
        }

        use Bias::{Left, Right};
        assert("ˇˇα", false, Left, cx);
        assert("ˇˇα", true, Left, cx);
        assert("ˇˇα", false, Right, cx);
        assert("ˇαˇ", true, Right, cx);
        assert("ˇˇ✋", false, Left, cx);
        assert("ˇˇ✋", true, Left, cx);
        assert("ˇˇ✋", false, Right, cx);
        assert("ˇ✋ˇ", true, Right, cx);
        assert("ˇˇ🍐", false, Left, cx);
        assert("ˇˇ🍐", true, Left, cx);
        assert("ˇˇ🍐", false, Right, cx);
        assert("ˇ🍐ˇ", true, Right, cx);
        assert("ˇˇ\t", false, Left, cx);
        assert("ˇˇ\t", true, Left, cx);
        assert("ˇˇ\t", false, Right, cx);
        assert("ˇ\tˇ", true, Right, cx);
        assert(" ˇˇ\t", false, Left, cx);
        assert(" ˇˇ\t", true, Left, cx);
        assert(" ˇˇ\t", false, Right, cx);
        assert(" ˇ\tˇ", true, Right, cx);
        assert("   ˇˇ\t", false, Left, cx);
        assert("   ˇˇ\t", false, Right, cx);
    }

    #[gpui::test]
    fn test_clip_at_line_ends(cx: &mut gpui::App) {
        init_test(cx, &|_| {});

        fn assert(text: &str, cx: &mut gpui::App) {
            let (mut unmarked_snapshot, markers) = marked_display_snapshot(text, cx);
            unmarked_snapshot.clip_at_line_ends = true;
            assert_eq!(
                unmarked_snapshot.clip_point(markers[1], Bias::Left),
                markers[0]
            );
        }

        assert("ˇˇ", cx);
        assert("ˇaˇ", cx);
        assert("aˇbˇ", cx);
        assert("aˇαˇ", cx);
    }

    #[gpui::test]
    fn test_creases(cx: &mut gpui::App) {
        init_test(cx, &|_| {});

        let text = "aaa\nbbb\nccc\nddd\neee\nfff\nggg\nhhh\niii\njjj\nkkk\nlll";
        let buffer = MultiBuffer::build_simple(text, cx);
        let font_size = px(14.0);
        cx.new(|cx| {
            let mut map = DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            );
            let snapshot = map.buffer.read(cx).snapshot(cx);
            let range =
                snapshot.anchor_before(Point::new(2, 0))..snapshot.anchor_after(Point::new(3, 3));

            map.crease_map.insert(
                [Crease::inline(
                    range,
                    FoldPlaceholder::test(),
                    |_row, _status, _toggle, _window, _cx| div(),
                    |_row, _status, _window, _cx| div(),
                )],
                &map.buffer.read(cx).snapshot(cx),
            );

            map
        });
    }

    #[gpui::test]
    fn test_tabs_with_multibyte_chars(cx: &mut gpui::App) {
        init_test(cx, &|_| {});

        let text = "✅\t\tα\nβ\t\n🏀β\t\tγ";
        let buffer = MultiBuffer::build_simple(text, cx);
        let font_size = px(14.0);

        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        let map = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(map.text(), "✅       α\nβ   \n🏀β      γ");
        assert_eq!(
            map.text_chunks(DisplayRow(0)).collect::<String>(),
            "✅       α\nβ   \n🏀β      γ"
        );
        assert_eq!(
            map.text_chunks(DisplayRow(1)).collect::<String>(),
            "β   \n🏀β      γ"
        );
        assert_eq!(
            map.text_chunks(DisplayRow(2)).collect::<String>(),
            "🏀β      γ"
        );

        let point = MultiBufferPoint::new(0, "✅\t\t".len() as u32);
        let display_point = DisplayPoint::new(DisplayRow(0), "✅       ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_point(&map), point);

        let point = MultiBufferPoint::new(1, "β\t".len() as u32);
        let display_point = DisplayPoint::new(DisplayRow(1), "β   ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_point(&map), point,);

        let point = MultiBufferPoint::new(2, "🏀β\t\t".len() as u32);
        let display_point = DisplayPoint::new(DisplayRow(2), "🏀β      ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_point(&map), point,);

        // Display points inside of expanded tabs
        assert_eq!(
            DisplayPoint::new(DisplayRow(0), "✅      ".len() as u32).to_point(&map),
            MultiBufferPoint::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(DisplayRow(0), "✅ ".len() as u32).to_point(&map),
            MultiBufferPoint::new(0, "✅".len() as u32),
        );

        // Clipping display points inside of multi-byte characters
        assert_eq!(
            map.clip_point(
                DisplayPoint::new(DisplayRow(0), "✅".len() as u32 - 1),
                Left
            ),
            DisplayPoint::new(DisplayRow(0), 0)
        );
        assert_eq!(
            map.clip_point(
                DisplayPoint::new(DisplayRow(0), "✅".len() as u32 - 1),
                Bias::Right
            ),
            DisplayPoint::new(DisplayRow(0), "✅".len() as u32)
        );
    }

    #[gpui::test]
    fn test_max_point(cx: &mut gpui::App) {
        init_test(cx, &|_| {});

        let buffer = MultiBuffer::build_simple("aaa\n\t\tbbb", cx);
        let font_size = px(14.0);
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx)).max_point(),
            DisplayPoint::new(DisplayRow(1), 11)
        )
    }

    fn syntax_chunks(
        rows: Range<DisplayRow>,
        map: &Entity<DisplayMap>,
        theme: &SyntaxTheme,
        cx: &mut App,
    ) -> Vec<(String, Option<Hsla>)> {
        chunks(rows, map, theme, cx)
            .into_iter()
            .map(|(text, color, _)| (text, color))
            .collect()
    }

    fn chunks(
        rows: Range<DisplayRow>,
        map: &Entity<DisplayMap>,
        theme: &SyntaxTheme,
        cx: &mut App,
    ) -> Vec<(String, Option<Hsla>, Option<Hsla>)> {
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut chunks: Vec<(String, Option<Hsla>, Option<Hsla>)> = Vec::new();
        for chunk in snapshot.chunks(
            rows,
            LanguageAwareStyling {
                tree_sitter: true,
                diagnostics: true,
            },
            HighlightStyles::default(),
        ) {
            let syntax_color = chunk
                .syntax_highlight_id
                .and_then(|id| theme.get(id)?.color);

            let highlight_color = chunk.highlight_style.and_then(|style| style.color);
            if let Some((last_chunk, last_syntax_color, last_highlight_color)) = chunks.last_mut()
                && syntax_color == *last_syntax_color
                && highlight_color == *last_highlight_color
            {
                last_chunk.push_str(chunk.text);
                continue;
            }
            chunks.push((chunk.text.to_string(), syntax_color, highlight_color));
        }
        chunks
    }

    /// Asserts that every header-like block in the snapshot references a
    /// buffer that is still present in the multibuffer: the invariant whose
    /// violation panics at render time with "buffer snapshot not found for
    /// excerpt boundary" (ZED-7G6).
    #[track_caller]
    fn assert_headers_resolve(snapshot: &DisplaySnapshot) {
        let end_row = DisplayRow(snapshot.max_point().row().0 + 1);
        for (row, block) in snapshot.blocks_in_range(DisplayRow(0)..end_row) {
            let excerpt = match block {
                Block::BufferHeader { excerpt, .. } | Block::ExcerptBoundary { excerpt, .. } => {
                    excerpt
                }
                Block::FoldedBuffer { first_excerpt, .. } => first_excerpt,
                _ => continue,
            };
            assert!(
                snapshot
                    .buffer_snapshot()
                    .buffer_for_id(excerpt.buffer_id())
                    .is_some(),
                "stale header block {:?} at {row:?} references buffer {:?}, \
                 which is no longer in the multibuffer",
                block.id(),
                excerpt.buffer_id(),
            );
        }
    }

    /// Deterministic end-to-end regression test for ZED-7G6 ("buffer snapshot
    /// not found for excerpt boundary"), driving a real `DisplayMap` with
    /// ordinary operations. In a diff-backed multibuffer with all hunks
    /// expanded, two folds inside an expanded deleted hunk are ordered only by
    /// their diff base anchors. Replacing the diff's base text used to invert
    /// that order (comparison filtered diff base anchors on validity, which
    /// the base edit revoked for one anchor of the pair), silently unsorting
    /// the fold map's persistent fold tree; subsequent syncs walked it with
    /// forward-only cursors and emitted edits that misdescribed the changed
    /// rows, until removing a buffer left its header block referencing a
    /// buffer absent from the snapshot -- the state whose render-time
    /// resolution panics.
    ///
    /// On pre-fix code this fails in the display map layers' internal
    /// checks; in production builds, where those checks don't run, the same
    /// corruption propagated to the stale header instead.
    #[gpui::test]
    async fn test_removing_buffer_removes_header_after_diff_base_changes(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| init_test(cx, &|_| {}));

        fn excerpt_buffer(
            multibuffer: &Entity<MultiBuffer>,
            path: u64,
            buffer: &Entity<Buffer>,
            cx: &mut gpui::TestAppContext,
        ) {
            multibuffer.update(cx, |multibuffer, cx| {
                let max_point = buffer.read(cx).max_point();
                multibuffer.set_excerpts_for_path(
                    PathKey::sorted(path),
                    buffer.clone(),
                    [Point::zero()..max_point],
                    0,
                    cx,
                );
            });
            cx.run_until_parked();
        }

        async fn set_base_text(
            diff: &Entity<buffer_diff::BufferDiff>,
            buffer: &Entity<Buffer>,
            base_text: &str,
            cx: &mut gpui::TestAppContext,
        ) {
            let snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
            diff.update(cx, |diff, cx| {
                diff.set_base_text(Some(base_text.to_string().into()), snapshot, cx)
            })
            .await;
            cx.run_until_parked();
        }

        #[track_caller]
        fn assert_headers(display_map: &Entity<DisplayMap>, cx: &mut gpui::TestAppContext) {
            cx.run_until_parked();
            let snapshot = display_map.update(cx, |display_map, cx| display_map.snapshot(cx));
            assert_headers_resolve(&snapshot);
        }

        let buffer_a = cx.new(|cx| Buffer::local("bbb\nccc\nddd\n", cx));
        let diff_a = cx.new(|cx| {
            buffer_diff::BufferDiff::new_with_base_text(
                "DEL1\nDEL2\nbbb\nccc\nddd\n",
                &buffer_a.read(cx).text_snapshot(),
                cx,
            )
        });
        let buffer_b = cx.new(|cx| Buffer::local("xxx\nyyy\n", cx));
        let buffer_b_id = buffer_b.read_with(cx, |buffer, _| buffer.remote_id());
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        excerpt_buffer(&multibuffer, 0, &buffer_a, cx);
        excerpt_buffer(&multibuffer, 1, &buffer_b, cx);
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.add_diff(diff_a.clone(), cx);
        });
        cx.run_until_parked();

        let display_map = cx.new(|cx| {
            DisplayMap::new(
                multibuffer.clone(),
                test_font(),
                px(14.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });
        assert_headers(&display_map, cx);

        // Two folds inside the expanded deleted hunk (rows 0 and 1 are
        // materialized from the base text), sharing a buffer position and
        // ordered only by their diff base anchors: a narrow fold within
        // "DEL1", then a wider fold from "DEL2" into the buffer's own rows.
        display_map.update(cx, |display_map, cx| {
            display_map.fold(
                vec![
                    Crease::simple(Point::new(0, 1)..Point::new(1, 0), FoldPlaceholder::test()),
                    Crease::simple(Point::new(1, 1)..Point::new(2, 2), FoldPlaceholder::test()),
                ],
                cx,
            );
        });
        assert_headers(&display_map, cx);

        // Keep "DEL1" (the first fold's base anchors survive) but delete
        // "DEL2" (the second fold's start anchor is tombstoned): the folds'
        // relative order must not change.
        set_base_text(&diff_a, &buffer_a, "DEL1\nbbb\nccc\nddd\n", cx).await;
        assert_headers(&display_map, cx);

        // Churn the buffers, the diff base, and buffer B's excerpts the way
        // the original fuzz sequence did, syncing the display map after each
        // group of operations.
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.remove_excerpts_for_buffer(buffer_b_id, cx);
        });
        buffer_a.update(cx, |buffer, cx| {
            buffer.edit([(4..5, "")], None, cx);
        });
        assert_headers(&display_map, cx);

        excerpt_buffer(&multibuffer, 1, &buffer_b, cx);
        set_base_text(&diff_a, &buffer_a, "DEL1\nbbb\nccc\nddd\n", cx).await;
        buffer_a.update(cx, |buffer, cx| {
            buffer.edit([(1..1, "Q\n")], None, cx);
        });
        assert_headers(&display_map, cx);

        set_base_text(&diff_a, &buffer_a, "DEL1\nbbb\nccc\nddd\n", cx).await;
        assert_headers(&display_map, cx);

        set_base_text(&diff_a, &buffer_a, "DEL2\nbbb\nccc\nddd\n", cx).await;
        buffer_a.update(cx, |buffer, cx| {
            buffer.edit([(2..2, "Q\n")], None, cx);
        });
        assert_headers(&display_map, cx);

        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.remove_excerpts_for_buffer(buffer_b_id, cx);
        });
        excerpt_buffer(&multibuffer, 1, &buffer_b, cx);
        assert_headers(&display_map, cx);

        // Removing B must remove its header block: with the fold tree
        // corrupted, the removal edit's rows were misdescribed by the time
        // they reached the block map, B's header row went uncovered, and the
        // header survived pointing at a buffer absent from the snapshot.
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.remove_excerpts_for_buffer(buffer_b_id, cx);
        });
        assert_headers(&display_map, cx);
    }

    fn init_test(cx: &mut App, f: &dyn Fn(&mut SettingsContent)) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        crate::init(cx);
        theme_settings::init(LoadThemes::JustBase, cx);
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, f);
        });
    }

    #[gpui::test]
    fn test_isomorphic_display_point_ranges_for_buffer_range(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, &|_| {}));

        let buffer = cx.new(|cx| Buffer::local("let x = 5;\n", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));

        let font_size = px(14.0);
        let map = cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        // Without inlays, a buffer range maps to a single display range.
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let ranges = snapshot.isomorphic_display_point_ranges_for_buffer_range(
            MultiBufferOffset(4)..MultiBufferOffset(9),
        );
        assert_eq!(ranges.len(), 1);
        // "x = 5" is columns 4..9 with no inlays shifting anything.
        assert_eq!(ranges[0].start, DisplayPoint::new(DisplayRow(0), 4));
        assert_eq!(ranges[0].end, DisplayPoint::new(DisplayRow(0), 9));

        // Insert a 4-char inlay hint ": i32" at buffer offset 5 (after "x").
        map.update(cx, |map, cx| {
            map.splice_inlays(
                &[],
                vec![Inlay::mock_hint(
                    0,
                    buffer_snapshot.anchor_after(MultiBufferOffset(5)),
                    ": i32",
                )],
                cx,
            );
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.text(), "let x: i32 = 5;\n");

        // A buffer range [4..9] ("x = 5") now spans across the inlay.
        // It should be split into two display ranges that skip the inlay text.
        let ranges = snapshot.isomorphic_display_point_ranges_for_buffer_range(
            MultiBufferOffset(4)..MultiBufferOffset(9),
        );
        assert_eq!(
            ranges.len(),
            2,
            "expected the range to be split around the inlay, got: {:?}",
            ranges,
        );
        // First sub-range: buffer [4, 5) → "x" at display columns 4..5
        assert_eq!(ranges[0].start, DisplayPoint::new(DisplayRow(0), 4));
        assert_eq!(ranges[0].end, DisplayPoint::new(DisplayRow(0), 5));
        // Second sub-range: buffer [5, 9) → " = 5" at display columns 10..14
        // (shifted right by the 5-char ": i32" inlay)
        assert_eq!(ranges[1].start, DisplayPoint::new(DisplayRow(0), 10));
        assert_eq!(ranges[1].end, DisplayPoint::new(DisplayRow(0), 14));

        // A range entirely before the inlay is not split.
        let ranges = snapshot.isomorphic_display_point_ranges_for_buffer_range(
            MultiBufferOffset(0)..MultiBufferOffset(5),
        );
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start, DisplayPoint::new(DisplayRow(0), 0));
        assert_eq!(ranges[0].end, DisplayPoint::new(DisplayRow(0), 5));

        // A range entirely after the inlay is not split.
        let ranges = snapshot.isomorphic_display_point_ranges_for_buffer_range(
            MultiBufferOffset(5)..MultiBufferOffset(9),
        );
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start, DisplayPoint::new(DisplayRow(0), 10));
        assert_eq!(ranges[0].end, DisplayPoint::new(DisplayRow(0), 14));

        map.update(cx, |map, cx| {
            map.splice_inlays(
                &[InlayId::Hint(0)],
                vec![
                    Inlay::mock_hint(1, buffer_snapshot.anchor_after(MultiBufferOffset(5)), "L"),
                    Inlay::mock_hint(2, buffer_snapshot.anchor_before(MultiBufferOffset(5)), "R"),
                    Inlay::mock_hint(3, buffer_snapshot.anchor_after(MultiBufferOffset(7)), "I"),
                ],
                cx,
            );
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));

        assert_eq!(
            snapshot.contiguous_display_point_range_for_buffer_range(
                MultiBufferOffset(4)..MultiBufferOffset(5),
            ),
            Some(DisplayPoint::new(DisplayRow(0), 4)..DisplayPoint::new(DisplayRow(0), 5)),
        );
        assert_eq!(
            snapshot.contiguous_display_point_range_for_buffer_range(
                MultiBufferOffset(5)..MultiBufferOffset(6),
            ),
            Some(DisplayPoint::new(DisplayRow(0), 7)..DisplayPoint::new(DisplayRow(0), 8)),
        );
        assert_eq!(
            snapshot.contiguous_display_point_range_for_buffer_range(
                MultiBufferOffset(4)..MultiBufferOffset(6),
            ),
            Some(DisplayPoint::new(DisplayRow(0), 4)..DisplayPoint::new(DisplayRow(0), 8)),
        );
        assert_eq!(
            snapshot.contiguous_display_point_range_for_buffer_range(
                MultiBufferOffset(6)..MultiBufferOffset(7),
            ),
            Some(DisplayPoint::new(DisplayRow(0), 8)..DisplayPoint::new(DisplayRow(0), 9)),
        );
        assert_eq!(
            snapshot.contiguous_display_point_range_for_buffer_range(
                MultiBufferOffset(7)..MultiBufferOffset(8),
            ),
            Some(DisplayPoint::new(DisplayRow(0), 10)..DisplayPoint::new(DisplayRow(0), 11)),
        );
        assert_eq!(
            snapshot.contiguous_display_point_range_for_buffer_range(
                MultiBufferOffset(4)..MultiBufferOffset(9),
            ),
            Some(DisplayPoint::new(DisplayRow(0), 4)..DisplayPoint::new(DisplayRow(0), 12)),
        );
        assert_eq!(
            snapshot.contiguous_display_point_range_for_buffer_range(
                MultiBufferOffset(5)..MultiBufferOffset(5),
            ),
            None,
        );
    }

    #[test]
    fn test_highlight_invisibles_preserves_compound_emojis() {
        let editor_style = EditorStyle::default();

        let pilot_emoji = "🧑\u{200d}✈\u{fe0f}";
        let chunk = HighlightedChunk {
            text: pilot_emoji,
            style: None,
            diagnostic_underline_severity: None,
            is_tab: false,
            is_inlay: false,
            replacement: None,
        };

        let chunks: Vec<_> = chunk
            .highlight_invisibles(&editor_style)
            .map(|chunk| chunk.text.to_string())
            .collect();

        assert_eq!(
            chunks.concat(),
            pilot_emoji,
            "all text bytes must be preserved"
        );
        assert_eq!(
            chunks.len(),
            1,
            "compound emoji should not be split into multiple chunks, got: {:?}",
            chunks,
        );
    }

    /// Regression test: Creating a DisplayMap when the MultiBuffer has pending
    /// unsynced changes should not cause a desync between the subscription edits
    /// and the InlayMap's buffer state.
    ///
    /// The bug occurred because:
    /// 1. DisplayMap::new created a subscription first
    /// 2. Then called snapshot() which synced and published edits
    /// 3. InlayMap was created with the post-sync snapshot
    /// 4. But the subscription captured the sync edits, leading to double-application
    #[gpui::test]
    fn test_display_map_subscription_ordering(cx: &mut gpui::App) {
        init_test(cx, &|_| {});

        // Create a buffer with some initial text
        let buffer = cx.new(|cx| Buffer::local("initial", cx));
        let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        // Edit the buffer. This sets buffer_changed_since_sync = true.
        // Importantly, do NOT call multibuffer.snapshot() yet.
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "prefix ")], None, cx);
        });

        // Create the DisplayMap. In the buggy code, this would:
        // 1. Create subscription (empty)
        // 2. Call snapshot() which syncs and publishes edits E1
        // 3. Create InlayMap with post-E1 snapshot
        // 4. Subscription now has E1, but InlayMap is already at post-E1 state
        let map = cx.new(|cx| {
            DisplayMap::new(
                multibuffer.clone(),
                font("Helvetica"),
                px(14.0),
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        // Verify initial state is correct
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.text(), "prefix initial");

        // Make another edit
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(7..7, "more ")], None, cx);
        });

        // This would crash in the buggy code because:
        // - InlayMap expects edits from V1 to V2
        // - But subscription has E1 ∘ E2 (from V0 to V2)
        // - The calculation `buffer_edit.new.end + (cursor.end().0 - buffer_edit.old.end)`
        //   would produce an offset exceeding the buffer length
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.text(), "prefix more initial");
    }
}
