use crate::{
    App, Bounds, Edges, Font, FontId, GlyphId, Half, Hsla, LayoutId, LineLayout, Pixels, Point,
    Result, SharedString, Size, StrikethroughStyle, TextRun, UnderlineStyle, WhiteSpace, Window,
    black, fill, point, px, size,
};
use collections::FxHashMap;
use derive_more::{Deref, DerefMut};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    cell::Cell,
    hash::{Hash, Hasher},
    ops::Range,
    sync::{Arc, OnceLock},
};

use super::{DecorationRun, FontRun, LineWrapper, WindowTextSystem};

pub(crate) const INLINE_BOX_PLACEHOLDER: char = '\u{001F}';

#[derive(Clone)]
pub(crate) enum InlineFlowItem {
    Text {
        text: SharedString,
        runs: Arc<[TextRun]>,
    },
    InlineBox {
        layout_id: LayoutId,
        metrics: Option<InlineBoxMetrics>,
        logical_len: usize,
    },
    HardBreak,
}

#[derive(Clone)]
pub(crate) struct InlineBoxMetrics {
    pub width: Pixels,
    pub height: Pixels,
    pub margin: Edges<Pixels>,
    pub baseline: Pixels,
}

#[derive(Clone)]
pub(crate) struct InlineFlowLayout {
    pub lines: Vec<InlineLine>,
    pub islands: Vec<InlineIsland>,
    pub boxes: Vec<InlineBoxPlacement>,
    pub logical_text: SharedString,
    pub content_size: Size<Pixels>,
    pub logical_len: usize,
    pub intrinsic_min_width: Pixels,
    pub intrinsic_max_width: Pixels,
    pub truncation: Option<InlineTruncationPlan>,
    shaped: Arc<OnceLock<ShapedInlineCache>>,
}

#[derive(Clone)]
pub(crate) struct InlineBoxPlacement {
    pub index: usize,
    pub relative_bounds: Bounds<Pixels>,
}

#[derive(Clone)]
pub(crate) struct InlineIsland {
    pub layout: Arc<LineLayout>,
    pub source_range: Range<usize>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct DecorationSignature {
    pub color: Hsla,
    pub background: Option<Hsla>,
    pub underline: Option<UnderlineStyle>,
    pub strikethrough: Option<StrikethroughStyle>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct EllipsisStyleKey {
    pub font_id: FontId,
    pub font_size: Pixels,
    pub decoration: DecorationSignature,
    pub ellipsis_text: SharedString,
}

#[derive(Clone)]
pub(crate) struct EllipsisLayout {
    pub layout: Arc<LineLayout>,
    pub runs: SmallVec<[DecorationRun; 1]>,
    pub width: Pixels,
}

#[derive(Clone)]
pub(crate) struct InlineTruncationPlan {
    pub line_ix: usize,
    pub clip_x: Pixels,
    pub ellipsis_x: Pixels,
    pub visible_width: Pixels,
    pub visible_height: Pixels,
    pub visible_text_height: Pixels,
    pub visible_logical_end: usize,
    pub ellipsis_style: EllipsisStyleKey,
    pub truncate_segment_ix: Option<usize>,
    pub truncate_text_end: Option<usize>,
    pub clip_first_item: bool,
}

#[derive(Clone)]
pub(crate) struct InlineLine {
    pub segments: Vec<InlineSegment>,
    pub y: Pixels,
    pub width: Pixels,
    pub height: Pixels,
    pub text_height: Pixels,
}

#[derive(Clone)]
pub(crate) enum InlineSegment {
    Text {
        island_ix: usize,
        text_range: Range<usize>,
        logical_range: Range<usize>,
        layout_start_x: Pixels,
        x: Pixels,
        width: Pixels,
    },
    InlineBox {
        index: usize,
        logical_range: Range<usize>,
        x: Pixels,
        width: Pixels,
    },
    HardBreak {
        logical_range: Range<usize>,
        x: Pixels,
    },
}

#[derive(Clone, Copy)]
pub(crate) struct DecorationSliceSpec {
    pub run_start: usize,
    pub run_end: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct TextGlyphRange {
    pub start_run: usize,
    pub start_glyph: usize,
    pub end_run: usize,
    pub end_glyph: usize,
    pub start_position: Point<Pixels>,
}

#[derive(Clone)]
pub(crate) struct InlineIslandDecorations {
    pub decoration_runs: SmallVec<[DecorationRun; 32]>,
}

#[derive(Clone)]
pub(crate) struct DecorationIndex {
    pub prefix_end: SmallVec<[usize; 64]>,
}

#[derive(Clone, Deref, DerefMut)]
pub(crate) struct ShapedInline {
    #[deref]
    #[deref_mut]
    pub(crate) layout: Arc<InlineFlowLayout>,
    pub(crate) decorations: Arc<[InlineIslandDecorations]>,
    pub(crate) decoration_index: Arc<[DecorationIndex]>,
}

impl ShapedInline {
    pub(crate) fn island(&self, island_ix: usize) -> ShapedInlineIsland<'_> {
        ShapedInlineIsland {
            layout: &self.islands[island_ix].layout,
            decorations: &self.decorations[island_ix].decoration_runs,
            decoration_index: &self.decoration_index[island_ix],
        }
    }
}

#[derive(Copy, Clone, Deref)]
pub(crate) struct ShapedInlineIsland<'a> {
    #[deref]
    pub(crate) layout: &'a LineLayout,
    pub(crate) decorations: &'a [DecorationRun],
    pub(crate) decoration_index: &'a DecorationIndex,
}

struct ShapedInlineCache {
    decorations: Arc<[InlineIslandDecorations]>,
    decoration_index: Arc<[DecorationIndex]>,
}

impl ShapedInlineIsland<'_> {
    pub(crate) fn decoration_slice_spec(&self, text_range: Range<usize>) -> DecorationSliceSpec {
        decoration_slice_spec_for_range_indexed(self.decoration_index, text_range)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct InlineCacheKey {
    pub items: Vec<InlineCacheItem>,
    pub font_size: Pixels,
    pub line_height: Pixels,
    pub wrap_width: Option<Pixels>,
    pub truncate_width: Option<Pixels>,
    pub white_space: crate::WhiteSpace,
    pub line_clamp: Option<usize>,
    pub text_overflow: Option<SharedString>,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum InlineCacheItem {
    Text {
        text: SharedString,
        font_runs: SmallVec<[FontRun; 1]>,
    },
    InlineBox {
        metrics: InlineBoxMetricsKey,
        logical_len: usize,
    },
    HardBreak,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct InlineBoxMetricsKey {
    pub width: Pixels,
    pub height: Pixels,
    pub margin: Edges<Pixels>,
    pub baseline: Pixels,
}

#[derive(Clone)]
enum InlineCacheItemRef<'a> {
    Text {
        text: &'a str,
        font_runs: SmallVec<[FontRun; 1]>,
    },
    InlineBox {
        metrics: InlineBoxMetricsKey,
        logical_len: usize,
    },
    HardBreak,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum InlineCacheItemView<'a> {
    Text {
        text: &'a str,
        font_runs: &'a [FontRun],
    },
    InlineBox {
        metrics: &'a InlineBoxMetricsKey,
        logical_len: usize,
    },
    HardBreak,
}

impl Hash for InlineCacheItemView<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            InlineCacheItemView::Text { text, font_runs } => {
                0u8.hash(state);
                text.hash(state);
                font_runs.hash(state);
            }
            InlineCacheItemView::InlineBox {
                metrics,
                logical_len,
            } => {
                1u8.hash(state);
                metrics.hash(state);
                logical_len.hash(state);
            }
            InlineCacheItemView::HardBreak => {
                2u8.hash(state);
            }
        }
    }
}

impl InlineCacheItem {
    fn as_view(&self) -> InlineCacheItemView<'_> {
        match self {
            InlineCacheItem::Text { text, font_runs } => InlineCacheItemView::Text {
                text: text.as_ref(),
                font_runs: font_runs.as_slice(),
            },
            InlineCacheItem::InlineBox {
                metrics,
                logical_len,
            } => InlineCacheItemView::InlineBox {
                metrics,
                logical_len: *logical_len,
            },
            InlineCacheItem::HardBreak => InlineCacheItemView::HardBreak,
        }
    }
}

impl InlineCacheItemRef<'_> {
    fn as_view(&self) -> InlineCacheItemView<'_> {
        match self {
            InlineCacheItemRef::Text { text, font_runs } => InlineCacheItemView::Text {
                text,
                font_runs: font_runs.as_slice(),
            },
            InlineCacheItemRef::InlineBox {
                metrics,
                logical_len,
            } => InlineCacheItemView::InlineBox {
                metrics,
                logical_len: *logical_len,
            },
            InlineCacheItemRef::HardBreak => InlineCacheItemView::HardBreak,
        }
    }
}

#[derive(Copy, Clone)]
enum InlineCacheItems<'a> {
    Owned(&'a [InlineCacheItem]),
    Borrowed(&'a [InlineCacheItemRef<'a>]),
}

impl<'a> InlineCacheItems<'a> {
    fn len(self) -> usize {
        match self {
            InlineCacheItems::Owned(items) => items.len(),
            InlineCacheItems::Borrowed(items) => items.len(),
        }
    }

    fn iter(self) -> InlineCacheItemsIter<'a> {
        match self {
            InlineCacheItems::Owned(items) => InlineCacheItemsIter::Owned(items.iter()),
            InlineCacheItems::Borrowed(items) => InlineCacheItemsIter::Borrowed(items.iter()),
        }
    }
}

enum InlineCacheItemsIter<'a> {
    Owned(std::slice::Iter<'a, InlineCacheItem>),
    Borrowed(std::slice::Iter<'a, InlineCacheItemRef<'a>>),
}

impl<'a> Iterator for InlineCacheItemsIter<'a> {
    type Item = InlineCacheItemView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            InlineCacheItemsIter::Owned(iter) => iter.next().map(|item| item.as_view()),
            InlineCacheItemsIter::Borrowed(iter) => iter.next().map(|item| item.as_view()),
        }
    }
}

#[derive(Copy, Clone)]
struct InlineCacheKeyRef<'a> {
    items: InlineCacheItems<'a>,
    font_size: Pixels,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    white_space: WhiteSpace,
    line_clamp: Option<usize>,
    text_overflow: Option<&'a str>,
}

impl PartialEq for InlineCacheKeyRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.font_size == other.font_size
            && self.line_height == other.line_height
            && self.wrap_width == other.wrap_width
            && self.truncate_width == other.truncate_width
            && self.white_space == other.white_space
            && self.line_clamp == other.line_clamp
            && self.text_overflow == other.text_overflow
            && self.items.len() == other.items.len()
            && self
                .items
                .iter()
                .zip(other.items.iter())
                .all(|(a, b)| a == b)
    }
}

impl Eq for InlineCacheKeyRef<'_> {}

impl Hash for InlineCacheKeyRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.items.iter() {
            item.hash(state);
        }
        self.font_size.hash(state);
        self.line_height.hash(state);
        self.wrap_width.hash(state);
        self.truncate_width.hash(state);
        self.white_space.hash(state);
        self.line_clamp.hash(state);
        self.text_overflow.hash(state);
    }
}

#[derive(Clone)]
struct InlineCacheKeyRefOwned<'a> {
    items: SmallVec<[InlineCacheItemRef<'a>; 8]>,
    font_size: Pixels,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    white_space: WhiteSpace,
    line_clamp: Option<usize>,
    text_overflow: Option<&'a str>,
}

trait AsInlineCacheKeyRef {
    fn as_inline_cache_key_ref(&self) -> InlineCacheKeyRef<'_>;
}

impl Hash for InlineBoxMetricsKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.margin.top.hash(state);
        self.margin.right.hash(state);
        self.margin.bottom.hash(state);
        self.margin.left.hash(state);
        self.baseline.hash(state);
    }
}

impl Hash for InlineCacheItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            InlineCacheItem::Text { text, font_runs } => {
                0u8.hash(state);
                text.hash(state);
                font_runs.hash(state);
            }
            InlineCacheItem::InlineBox {
                metrics,
                logical_len,
            } => {
                1u8.hash(state);
                metrics.hash(state);
                logical_len.hash(state);
            }
            InlineCacheItem::HardBreak => {
                2u8.hash(state);
            }
        }
    }
}

impl Hash for InlineCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_inline_cache_key_ref().hash(state);
    }
}

impl AsInlineCacheKeyRef for InlineCacheKey {
    fn as_inline_cache_key_ref(&self) -> InlineCacheKeyRef<'_> {
        InlineCacheKeyRef {
            items: InlineCacheItems::Owned(&self.items),
            font_size: self.font_size,
            line_height: self.line_height,
            wrap_width: self.wrap_width,
            truncate_width: self.truncate_width,
            white_space: self.white_space,
            line_clamp: self.line_clamp,
            text_overflow: self.text_overflow.as_ref().map(SharedString::as_ref),
        }
    }
}

impl AsInlineCacheKeyRef for InlineCacheKeyRefOwned<'_> {
    fn as_inline_cache_key_ref(&self) -> InlineCacheKeyRef<'_> {
        InlineCacheKeyRef {
            items: InlineCacheItems::Borrowed(self.items.as_slice()),
            font_size: self.font_size,
            line_height: self.line_height,
            wrap_width: self.wrap_width,
            truncate_width: self.truncate_width,
            white_space: self.white_space,
            line_clamp: self.line_clamp,
            text_overflow: self.text_overflow,
        }
    }
}

impl AsInlineCacheKeyRef for InlineCacheKeyRef<'_> {
    fn as_inline_cache_key_ref(&self) -> InlineCacheKeyRef<'_> {
        *self
    }
}

impl PartialEq for dyn AsInlineCacheKeyRef + '_ {
    fn eq(&self, other: &dyn AsInlineCacheKeyRef) -> bool {
        self.as_inline_cache_key_ref() == other.as_inline_cache_key_ref()
    }
}

impl Eq for dyn AsInlineCacheKeyRef + '_ {}

impl Hash for dyn AsInlineCacheKeyRef + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_inline_cache_key_ref().hash(state);
    }
}

impl<'a> Borrow<dyn AsInlineCacheKeyRef + 'a> for Arc<InlineCacheKey> {
    fn borrow(&self) -> &(dyn AsInlineCacheKeyRef + 'a) {
        self.as_ref() as &dyn AsInlineCacheKeyRef
    }
}

pub(crate) struct InlineLayoutCache {
    previous_frame: Mutex<InlineFrameCache>,
    current_frame: RwLock<InlineFrameCache>,
}

#[derive(Default)]
struct InlineFrameCache {
    layouts: FxHashMap<Arc<InlineCacheKey>, Arc<InlineFlowLayout>>,
    used_layouts: Vec<Arc<InlineCacheKey>>,
}

#[derive(Clone, Default)]
pub(crate) struct InlineLayoutIndex {
    pub(crate) inline_layout_index: usize,
}

impl InlineLayoutCache {
    pub fn new() -> Self {
        Self {
            previous_frame: Mutex::default(),
            current_frame: RwLock::default(),
        }
    }

    pub fn layout_index(&self) -> InlineLayoutIndex {
        let frame = self.current_frame.read();
        InlineLayoutIndex {
            inline_layout_index: frame.used_layouts.len(),
        }
    }

    pub fn reuse_layouts(&self, range: Range<InlineLayoutIndex>) {
        let mut previous_frame = &mut *self.previous_frame.lock();
        let mut current_frame = &mut *self.current_frame.write();

        for key in &previous_frame.used_layouts
            [range.start.inline_layout_index..range.end.inline_layout_index]
        {
            if let Some((key, layout)) = previous_frame.layouts.remove_entry(key) {
                current_frame.layouts.insert(key, layout);
            }
            current_frame.used_layouts.push(key.clone());
        }
    }

    pub fn truncate_layouts(&self, index: InlineLayoutIndex) {
        let mut current_frame = &mut *self.current_frame.write();
        current_frame
            .used_layouts
            .truncate(index.inline_layout_index);
    }

    pub fn finish_frame(&self) {
        let mut prev_frame = self.previous_frame.lock();
        let mut curr_frame = self.current_frame.write();
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);
        curr_frame.layouts.clear();
        curr_frame.used_layouts.clear();
    }

    fn get_or_insert_with_ref<F, K>(
        &self,
        key_ref: &dyn AsInlineCacheKeyRef,
        build_key: K,
        build: F,
    ) -> Arc<InlineFlowLayout>
    where
        F: FnOnce() -> InlineFlowLayout,
        K: FnOnce() -> InlineCacheKey,
    {
        let current_frame = self.current_frame.upgradable_read();
        if let Some((cached_key, layout)) = current_frame.layouts.get_key_value(key_ref) {
            let layout = layout.clone();
            let cached_key = cached_key.clone();
            let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
            current_frame.used_layouts.push(cached_key);
            return layout;
        }

        if let Some((cached_key, layout)) = self.previous_frame.lock().layouts.remove_entry(key_ref)
        {
            let layout = layout.clone();
            let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
            current_frame
                .layouts
                .insert(cached_key.clone(), layout.clone());
            current_frame.used_layouts.push(cached_key);
            return layout;
        }

        drop(current_frame);

        let layout = Arc::new(build());
        let key = Arc::new(build_key());
        let mut current_frame = self.current_frame.write();
        current_frame.layouts.insert(key.clone(), layout.clone());
        current_frame.used_layouts.push(key);
        layout
    }
}

struct TextIslandSource {
    text: SharedString,
    runs: Vec<TextRun>,
    source_range: Range<usize>,
}

struct GlyphInfo {
    text_ix: usize,
    x: Pixels,
    ch: char,
}

struct TextIsland {
    text: SharedString,
    layout: Arc<LineLayout>,
    glyphs: Vec<GlyphInfo>,
    source_range: Range<usize>,
}

struct BoxEntry {
    metrics: InlineBoxMetrics,
}

enum FlowEntry {
    Island(usize),
    InlineBox { index: usize, logical_len: usize },
    HardBreak,
}

struct LineState {
    segments: Vec<InlineSegment>,
    line_x: Pixels,
    max_ascent: Pixels,
    max_descent: Pixels,
    max_box_above: Pixels,
    max_box_below: Pixels,
    has_text: bool,
}

impl Default for LineState {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            line_x: Pixels::ZERO,
            max_ascent: Pixels::ZERO,
            max_descent: Pixels::ZERO,
            max_box_above: Pixels::ZERO,
            max_box_below: Pixels::ZERO,
            has_text: false,
        }
    }
}

impl WindowTextSystem {
    pub(crate) fn shape_inline_flow(
        &self,
        items: &[InlineFlowItem],
        font_size: Pixels,
        line_height: Pixels,
        wrap_width: Option<Pixels>,
        truncate_width: Option<Pixels>,
        white_space: WhiteSpace,
        line_clamp: Option<usize>,
        text_overflow: Option<SharedString>,
    ) -> Arc<InlineFlowLayout> {
        let text_overflow_ref = text_overflow.clone();
        let key_ref = build_inline_cache_key_ref(
            self,
            items,
            font_size,
            line_height,
            wrap_width,
            truncate_width,
            white_space,
            line_clamp,
            text_overflow_ref.as_ref(),
        );
        let text_overflow_for_key = text_overflow.clone();

        self.inline_layout_cache.get_or_insert_with_ref(
            &key_ref,
            || {
                build_inline_cache_key(
                    self,
                    items,
                    font_size,
                    line_height,
                    wrap_width,
                    truncate_width,
                    white_space,
                    line_clamp,
                    text_overflow_for_key,
                )
            },
            || {
                build_inline_flow_layout(
                    self,
                    items,
                    font_size,
                    line_height,
                    wrap_width,
                    truncate_width,
                    white_space,
                    line_clamp,
                    text_overflow,
                )
            },
        )
    }

    pub(crate) fn shape_inline(
        &self,
        items: &[InlineFlowItem],
        layout: Arc<InlineFlowLayout>,
    ) -> ShapedInline {
        let shaped = layout.shaped.get_or_init(|| {
            let flat_runs = build_flat_decoration_runs(items);
            let flat_index = build_decoration_index(&flat_runs);
            let mut decorations = Vec::with_capacity(layout.islands.len());
            let mut decoration_index = Vec::with_capacity(layout.islands.len());

            for island in &layout.islands {
                let runs =
                    slice_decoration_runs(&flat_runs, &flat_index, island.source_range.clone());
                decoration_index.push(build_decoration_index(&runs));
                decorations.push(InlineIslandDecorations {
                    decoration_runs: runs,
                });
            }

            ShapedInlineCache {
                decorations: decorations.into(),
                decoration_index: decoration_index.into(),
            }
        });

        ShapedInline {
            layout: Arc::clone(&layout),
            decorations: Arc::clone(&shaped.decorations),
            decoration_index: Arc::clone(&shaped.decoration_index),
        }
    }

    pub(crate) fn ellipsis_layout_for_style(
        &self,
        style: &EllipsisStyleKey,
    ) -> Arc<EllipsisLayout> {
        ellipsis_layout_for_style(self, style)
    }
}

fn build_inline_cache_key(
    text_system: &WindowTextSystem,
    items: &[InlineFlowItem],
    font_size: Pixels,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    white_space: WhiteSpace,
    line_clamp: Option<usize>,
    text_overflow: Option<SharedString>,
) -> InlineCacheKey {
    let mut cache_items = Vec::with_capacity(items.len());

    for item in items {
        match item {
            InlineFlowItem::Text { text, runs } => {
                let font_runs = font_runs_for_text_runs(text_system, runs);
                cache_items.push(InlineCacheItem::Text {
                    text: text.clone(),
                    font_runs,
                });
            }
            InlineFlowItem::InlineBox {
                metrics,
                logical_len,
                ..
            } => {
                let metrics_key = inline_box_metrics_key(metrics.as_ref());
                cache_items.push(InlineCacheItem::InlineBox {
                    metrics: metrics_key,
                    logical_len: *logical_len,
                });
            }
            InlineFlowItem::HardBreak => {
                cache_items.push(InlineCacheItem::HardBreak);
            }
        }
    }

    InlineCacheKey {
        items: cache_items,
        font_size,
        line_height,
        wrap_width,
        truncate_width,
        white_space,
        line_clamp,
        text_overflow,
    }
}

fn inline_box_metrics_key(metrics: Option<&InlineBoxMetrics>) -> InlineBoxMetricsKey {
    match metrics {
        Some(metrics) => InlineBoxMetricsKey {
            width: metrics.width,
            height: metrics.height,
            margin: metrics.margin.clone(),
            baseline: metrics.baseline,
        },
        None => InlineBoxMetricsKey {
            width: Pixels::ZERO,
            height: Pixels::ZERO,
            margin: Edges::default(),
            baseline: Pixels::ZERO,
        },
    }
}

fn build_inline_cache_key_ref<'a>(
    text_system: &WindowTextSystem,
    items: &'a [InlineFlowItem],
    font_size: Pixels,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    white_space: WhiteSpace,
    line_clamp: Option<usize>,
    text_overflow: Option<&'a SharedString>,
) -> InlineCacheKeyRefOwned<'a> {
    let mut cache_items = SmallVec::with_capacity(items.len());

    for item in items {
        match item {
            InlineFlowItem::Text { text, runs } => {
                let font_runs = font_runs_for_text_runs(text_system, runs);
                cache_items.push(InlineCacheItemRef::Text {
                    text: text.as_ref(),
                    font_runs,
                });
            }
            InlineFlowItem::InlineBox {
                metrics,
                logical_len,
                ..
            } => {
                let metrics_key = inline_box_metrics_key(metrics.as_ref());
                cache_items.push(InlineCacheItemRef::InlineBox {
                    metrics: metrics_key,
                    logical_len: *logical_len,
                });
            }
            InlineFlowItem::HardBreak => {
                cache_items.push(InlineCacheItemRef::HardBreak);
            }
        }
    }

    InlineCacheKeyRefOwned {
        items: cache_items,
        font_size,
        line_height,
        wrap_width,
        truncate_width,
        white_space,
        line_clamp,
        text_overflow: text_overflow.map(SharedString::as_ref),
    }
}

fn build_inline_flow_layout(
    text_system: &WindowTextSystem,
    items: &[InlineFlowItem],
    font_size: Pixels,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    white_space: WhiteSpace,
    line_clamp: Option<usize>,
    text_overflow: Option<SharedString>,
) -> InlineFlowLayout {
    let (island_sources, flow_entries, box_entries) = build_inline_flow_entries(items);
    let wrap_enabled = wrap_width.is_some();
    let wrap_width_limit = wrap_width.unwrap_or(Pixels::MAX);

    let mut inline_islands = Vec::with_capacity(island_sources.len());
    let mut text_islands = Vec::with_capacity(island_sources.len());
    for source in island_sources {
        let TextIslandSource {
            text,
            runs,
            source_range,
        } = source;
        debug_assert!(!text.as_ref().contains('\n'));
        let font_runs = font_runs_for_text_runs(text_system, &runs);
        let layout = text_system
            .line_layout_cache
            .layout_line(&text, font_size, &font_runs, None);
        let mut glyphs = Vec::new();
        if !text.is_empty() {
            for run in layout.runs.iter() {
                for glyph in run.glyphs.iter() {
                    let ch = text[glyph.index..].chars().next().unwrap();
                    glyphs.push(GlyphInfo {
                        text_ix: glyph.index,
                        x: glyph.position.x,
                        ch,
                    });
                }
            }
        }
        inline_islands.push(InlineIsland {
            layout: layout.clone(),
            source_range: source_range.clone(),
        });
        text_islands.push(TextIsland {
            text,
            layout,
            glyphs,
            source_range,
        });
    }

    let (intrinsic_min_width, intrinsic_max_width) =
        compute_intrinsic_widths(&text_islands, &box_entries, &flow_entries);

    let mut logical_text = String::new();
    let mut logical_offset = 0;
    let mut lines = Vec::new();
    let mut boxes = Vec::new();
    let mut y_cursor = Pixels::ZERO;
    let mut max_line_width = Pixels::ZERO;
    let max_lines = line_clamp.unwrap_or(usize::MAX);
    let mut stop = false;
    let mut line_clamped = false;
    let mut line_state = LineState::default();
    let mut last_event_hard_break = false;
    let line_count = Cell::new(0usize);

    let mut finish_line = |line_state: &mut LineState| -> bool {
        let line = LineState {
            segments: std::mem::take(&mut line_state.segments),
            line_x: line_state.line_x,
            max_ascent: line_state.max_ascent,
            max_descent: line_state.max_descent,
            max_box_above: line_state.max_box_above,
            max_box_below: line_state.max_box_below,
            has_text: line_state.has_text,
        };

        y_cursor = push_line(
            &mut lines,
            &mut boxes,
            &box_entries,
            line,
            y_cursor,
            line_height,
            &mut max_line_width,
        );
        line_count.set(line_count.get() + 1);
        *line_state = LineState::default();
        lines.len() < max_lines
    };

    'flow: for entry in flow_entries.iter() {
        if stop {
            break;
        }
        match entry {
            FlowEntry::Island(island_ix) => {
                let island = &text_islands[*island_ix];
                if island.glyphs.is_empty() {
                    continue;
                }
                let mut glyph_ix = 0;
                while glyph_ix < island.glyphs.len() {
                    if wrap_enabled
                        && line_state.line_x > Pixels::ZERO
                        && line_state.line_x >= wrap_width_limit
                    {
                        if !finish_line(&mut line_state) {
                            line_clamped = true;
                            stop = true;
                            break 'flow;
                        }
                    }

                    let start_x = island.glyphs[glyph_ix].x;
                    let available_width = if wrap_enabled {
                        (wrap_width_limit - line_state.line_x).max(Pixels::ZERO)
                    } else {
                        Pixels::MAX
                    };
                    if wrap_enabled && line_state.line_x > Pixels::ZERO {
                        let next_x = if glyph_ix + 1 < island.glyphs.len() {
                            island.glyphs[glyph_ix + 1].x
                        } else {
                            island.layout.width
                        };
                        let first_glyph_width = next_x - start_x;
                        if first_glyph_width > available_width {
                            if !finish_line(&mut line_state) {
                                line_clamped = true;
                                stop = true;
                                break 'flow;
                            }
                            continue;
                        }
                    }
                    let mut end_glyph_ix = if wrap_enabled {
                        find_wrap_boundary(
                            &island.glyphs,
                            glyph_ix,
                            start_x,
                            available_width,
                            island.layout.width,
                            !line_state.segments.is_empty(),
                        )
                        .unwrap_or(island.glyphs.len())
                    } else {
                        island.glyphs.len()
                    };

                    if end_glyph_ix <= glyph_ix {
                        end_glyph_ix = (glyph_ix + 1).min(island.glyphs.len());
                    }

                    let text_start = island.glyphs[glyph_ix].text_ix;
                    let text_end = if end_glyph_ix < island.glyphs.len() {
                        island.glyphs[end_glyph_ix].text_ix
                    } else {
                        island.text.len()
                    };
                    let end_x = if end_glyph_ix < island.glyphs.len() {
                        island.glyphs[end_glyph_ix].x
                    } else {
                        island.layout.width
                    };
                    let segment_width = end_x - start_x;
                    let logical_start = logical_offset;
                    logical_offset += text_end - text_start;
                    logical_text.push_str(&island.text[text_start..text_end]);
                    line_state.segments.push(InlineSegment::Text {
                        island_ix: *island_ix,
                        text_range: text_start..text_end,
                        logical_range: logical_start..logical_offset,
                        layout_start_x: start_x,
                        x: line_state.line_x,
                        width: segment_width,
                    });
                    line_state.line_x += segment_width;
                    line_state.has_text = true;
                    line_state.max_ascent = line_state.max_ascent.max(island.layout.ascent);
                    line_state.max_descent = line_state.max_descent.max(island.layout.descent);
                    last_event_hard_break = false;

                    glyph_ix = end_glyph_ix;
                    if wrap_enabled && end_glyph_ix < island.glyphs.len() {
                        if !finish_line(&mut line_state) {
                            line_clamped = true;
                            stop = true;
                            break 'flow;
                        }
                    }
                }
            }
            FlowEntry::InlineBox { index, logical_len } => {
                let metrics = &box_entries[*index].metrics;
                let width_with_margins = metrics.width + metrics.margin.left + metrics.margin.right;
                if wrap_enabled
                    && line_state.line_x > Pixels::ZERO
                    && line_state.line_x + width_with_margins > wrap_width_limit
                {
                    if !finish_line(&mut line_state) {
                        line_clamped = true;
                        stop = true;
                        break;
                    }
                }

                let logical_start = logical_offset;
                logical_offset += *logical_len;
                for _ in 0..*logical_len {
                    logical_text.push(INLINE_BOX_PLACEHOLDER);
                }
                line_state.segments.push(InlineSegment::InlineBox {
                    index: *index,
                    logical_range: logical_start..logical_offset,
                    x: line_state.line_x,
                    width: width_with_margins,
                });
                line_state.line_x += width_with_margins;
                line_state.max_box_above = line_state
                    .max_box_above
                    .max(metrics.baseline + metrics.margin.top);
                let below =
                    (metrics.height - metrics.baseline).max(Pixels::ZERO) + metrics.margin.bottom;
                line_state.max_box_below = line_state.max_box_below.max(below);
                last_event_hard_break = false;
            }
            FlowEntry::HardBreak => {
                let logical_start = logical_offset;
                logical_offset += 1;
                logical_text.push('\n');
                line_state.segments.push(InlineSegment::HardBreak {
                    logical_range: logical_start..logical_offset,
                    x: line_state.line_x,
                });
                last_event_hard_break = true;
                if !finish_line(&mut line_state) {
                    line_clamped = true;
                    stop = true;
                    break;
                }
            }
        }
    }

    if !stop {
        if !line_state.segments.is_empty() {
            finish_line(&mut line_state);
        } else if line_count.get() == 0 || last_event_hard_break {
            finish_line(&mut line_state);
        }
    }

    let truncation = compute_truncation_plan(
        text_system,
        items,
        &lines,
        &text_islands,
        &box_entries,
        font_size,
        line_height,
        wrap_width,
        truncate_width,
        white_space,
        line_clamped,
        text_overflow.as_ref(),
    );

    let (content_width, content_height) = if let Some(truncation) = &truncation {
        let mut width = Pixels::ZERO;
        for (ix, line) in lines.iter().enumerate() {
            let line_width = if ix == truncation.line_ix {
                truncation.visible_width
            } else {
                line.width
            };
            width = width.max(line_width);
        }
        let height = lines
            .get(truncation.line_ix)
            .map(|line| line.y + truncation.visible_height)
            .unwrap_or(y_cursor);
        (width, height)
    } else {
        (max_line_width, y_cursor)
    };

    InlineFlowLayout {
        lines,
        islands: inline_islands,
        boxes,
        logical_text: logical_text.into(),
        content_size: Size {
            width: content_width,
            height: content_height,
        },
        logical_len: logical_offset,
        intrinsic_min_width,
        intrinsic_max_width,
        truncation,
        shaped: Arc::new(OnceLock::new()),
    }
}

fn font_runs_for_text_runs(
    text_system: &WindowTextSystem,
    runs: &[TextRun],
) -> SmallVec<[FontRun; 1]> {
    let mut font_runs = SmallVec::<[FontRun; 1]>::new();
    let mut last_run = None::<&TextRun>;

    for run in runs.iter() {
        let decoration_changed = if let Some(last_run) = last_run
            && last_run.color == run.color
            && last_run.underline == run.underline
            && last_run.strikethrough == run.strikethrough
        // we do not consider differing background color relevant, as it does not affect glyphs
        // && last_run.background_color == run.background_color
        {
            false
        } else {
            last_run = Some(run);
            true
        };

        let font_id = text_system.resolve_font(&run.font);
        if let Some(font_run) = font_runs.last_mut()
            && font_id == font_run.font_id
            && !decoration_changed
        {
            font_run.len += run.len;
        } else {
            font_runs.push(FontRun {
                len: run.len,
                font_id,
            });
        }
    }

    font_runs
}

fn build_inline_flow_entries(
    items: &[InlineFlowItem],
) -> (Vec<TextIslandSource>, Vec<FlowEntry>, Vec<BoxEntry>) {
    let mut islands = Vec::new();
    let mut flow_entries = Vec::new();
    let mut box_entries = Vec::new();
    let mut current_text = String::new();
    let mut current_runs = Vec::new();
    let mut island_start_offset = 0;
    let mut text_offset = 0;
    let mut box_index = 0;

    let mut flush_island = |current_text: &mut String,
                            current_runs: &mut Vec<TextRun>,
                            islands: &mut Vec<TextIslandSource>,
                            flow_entries: &mut Vec<FlowEntry>,
                            island_start_offset: usize,
                            text_offset: usize| {
        if current_text.is_empty() {
            return;
        }
        let text = SharedString::from(std::mem::take(current_text));
        let runs = std::mem::take(current_runs);
        islands.push(TextIslandSource {
            text,
            runs,
            source_range: island_start_offset..text_offset,
        });
        flow_entries.push(FlowEntry::Island(islands.len() - 1));
    };

    for item in items {
        match item {
            InlineFlowItem::Text { text, runs } => {
                if current_text.is_empty() {
                    island_start_offset = text_offset;
                }
                current_text.push_str(text.as_ref());
                current_runs.extend_from_slice(runs);
                text_offset += text.len();
            }
            InlineFlowItem::InlineBox {
                metrics,
                logical_len,
                ..
            } => {
                flush_island(
                    &mut current_text,
                    &mut current_runs,
                    &mut islands,
                    &mut flow_entries,
                    island_start_offset,
                    text_offset,
                );
                let metrics = metrics.clone().unwrap_or(InlineBoxMetrics {
                    width: Pixels::ZERO,
                    height: Pixels::ZERO,
                    margin: Edges::default(),
                    baseline: Pixels::ZERO,
                });
                box_entries.push(BoxEntry { metrics });
                flow_entries.push(FlowEntry::InlineBox {
                    index: box_index,
                    logical_len: *logical_len,
                });
                box_index += 1;
            }
            InlineFlowItem::HardBreak => {
                flush_island(
                    &mut current_text,
                    &mut current_runs,
                    &mut islands,
                    &mut flow_entries,
                    island_start_offset,
                    text_offset,
                );
                flow_entries.push(FlowEntry::HardBreak);
            }
        }
    }

    flush_island(
        &mut current_text,
        &mut current_runs,
        &mut islands,
        &mut flow_entries,
        island_start_offset,
        text_offset,
    );

    (islands, flow_entries, box_entries)
}

fn find_wrap_boundary(
    glyphs: &[GlyphInfo],
    start_glyph_ix: usize,
    line_start_x: Pixels,
    wrap_width: Pixels,
    layout_width: Pixels,
    line_has_content: bool,
) -> Option<usize> {
    let mut first_non_whitespace_ix = if line_has_content {
        Some(start_glyph_ix)
    } else {
        None
    };
    let mut last_candidate_ix = None;
    let mut last_boundary_ix = start_glyph_ix;
    let mut last_boundary_x = line_start_x;
    let mut prev_ch = '\0';

    for ix in start_glyph_ix..glyphs.len() {
        let glyph = &glyphs[ix];
        let ch = glyph.ch;
        if ch == '\n' {
            continue;
        }

        if LineWrapper::is_word_char(ch) {
            if prev_ch == ' ' && ch != ' ' && first_non_whitespace_ix.is_some() {
                last_candidate_ix = Some(ix);
            }
        } else if ch != ' ' && first_non_whitespace_ix.is_some() {
            last_candidate_ix = Some(ix);
        }

        if ch != ' ' && first_non_whitespace_ix.is_none() {
            first_non_whitespace_ix = Some(ix);
        }

        let next_x = if ix + 1 < glyphs.len() {
            glyphs[ix + 1].x
        } else {
            layout_width
        };
        let width = next_x - last_boundary_x;
        if width > wrap_width && ix > last_boundary_ix {
            return Some(last_candidate_ix.unwrap_or(ix));
        }
        prev_ch = ch;
    }

    None
}

fn push_line(
    lines: &mut Vec<InlineLine>,
    boxes: &mut Vec<InlineBoxPlacement>,
    box_entries: &[BoxEntry],
    line: LineState,
    y_cursor: Pixels,
    target_line_height: Pixels,
    max_line_width: &mut Pixels,
) -> Pixels {
    let text_height = line.max_ascent + line.max_descent;
    let extra_leading = (target_line_height - text_height).max(Pixels::ZERO);
    let line_ascent = if line.has_text {
        line.max_ascent
    } else {
        Pixels::ZERO
    };
    let line_descent = (target_line_height - line_ascent)
        .max((line.max_box_above + line.max_box_below - line_ascent).max(Pixels::ZERO));
    let line_height = line_ascent + line_descent;
    let text_height = line.max_ascent + line.max_descent + extra_leading;

    for segment in &line.segments {
        if let InlineSegment::InlineBox { index, x, .. } = segment {
            let box_entry = &box_entries[*index];
            let box_x = *x + box_entry.metrics.margin.left;
            let box_y = y_cursor + box_entry.metrics.margin.top;
            boxes.push(InlineBoxPlacement {
                index: *index,
                relative_bounds: Bounds {
                    origin: point(box_x, box_y),
                    size: size(box_entry.metrics.width, box_entry.metrics.height),
                },
            });
        }
    }

    let line_width = line.line_x;
    lines.push(InlineLine {
        segments: line.segments,
        y: y_cursor,
        width: line_width,
        height: line_height,
        text_height,
    });
    *max_line_width = (*max_line_width).max(line_width);

    y_cursor + line_height
}

#[derive(Clone)]
struct TruncationCut {
    clip_x: Pixels,
    ellipsis_x: Pixels,
    truncate_segment_ix: Option<usize>,
    truncate_text_end: Option<usize>,
    clip_first_item: bool,
    visible_logical_end: usize,
    text_anchor: Option<(usize, usize)>,
}

fn line_end_logical(line: &InlineLine) -> usize {
    line.segments
        .last()
        .map(|segment| match segment {
            InlineSegment::HardBreak { logical_range, .. } => logical_range.start,
            InlineSegment::Text { logical_range, .. }
            | InlineSegment::InlineBox { logical_range, .. } => logical_range.end,
        })
        .unwrap_or(0)
}

fn decoration_signature_from_run(run: &DecorationRun) -> DecorationSignature {
    DecorationSignature {
        color: run.color,
        background: run.background_color,
        underline: run.underline,
        strikethrough: run.strikethrough,
    }
}

fn decoration_signature_from_text_run(run: &TextRun) -> DecorationSignature {
    DecorationSignature {
        color: run.color,
        background: run.background_color,
        underline: run.underline,
        strikethrough: run.strikethrough,
    }
}

fn fallback_decoration_signature(items: &[InlineFlowItem]) -> DecorationSignature {
    for item in items {
        if let InlineFlowItem::Text { runs, .. } = item
            && let Some(run) = runs.first()
        {
            return decoration_signature_from_text_run(run);
        }
    }
    DecorationSignature {
        color: black(),
        background: None,
        underline: None,
        strikethrough: None,
    }
}

fn fallback_font_id_for_items(text_system: &WindowTextSystem, items: &[InlineFlowItem]) -> FontId {
    for item in items {
        if let InlineFlowItem::Text { runs, .. } = item
            && let Some(run) = runs.first()
        {
            return text_system.resolve_font(&run.font);
        }
    }
    text_system.resolve_font(&Font::default())
}

fn text_offset_for_anchor(
    islands: &[TextIsland],
    island_ix: usize,
    text_end: usize,
) -> Option<usize> {
    let start = islands.get(island_ix)?.source_range.start;
    Some(start + text_end)
}

fn ellipsis_layout_for_style(
    text_system: &WindowTextSystem,
    style: &EllipsisStyleKey,
) -> Arc<EllipsisLayout> {
    let mut cache = text_system.ellipsis_layout_cache.lock();
    cache.get_or_insert_with(style, || {
        let layout = if style.ellipsis_text.is_empty() {
            Arc::new(LineLayout {
                font_size: style.font_size,
                width: Pixels::ZERO,
                ascent: Pixels::ZERO,
                descent: Pixels::ZERO,
                runs: Vec::new(),
                len: 0,
            })
        } else {
            let font_runs = [FontRun {
                len: style.ellipsis_text.len(),
                font_id: style.font_id,
            }];
            text_system.line_layout_cache.layout_line(
                &style.ellipsis_text,
                style.font_size,
                &font_runs,
                None,
            )
        };

        let mut runs = SmallVec::<[DecorationRun; 1]>::new();
        if !style.ellipsis_text.is_empty() {
            runs.push(DecorationRun {
                len: style.ellipsis_text.len() as u32,
                color: style.decoration.color,
                background_color: style.decoration.background,
                underline: style.decoration.underline,
                strikethrough: style.decoration.strikethrough,
            });
        }

        let width = layout.width;
        Arc::new(EllipsisLayout {
            layout,
            runs,
            width,
        })
    })
}

fn compute_truncation_plan(
    text_system: &WindowTextSystem,
    items: &[InlineFlowItem],
    lines: &[InlineLine],
    islands: &[TextIsland],
    box_entries: &[BoxEntry],
    font_size: Pixels,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    white_space: WhiteSpace,
    line_clamped: bool,
    text_overflow: Option<&SharedString>,
) -> Option<InlineTruncationPlan> {
    let ellipsis_text = text_overflow?;
    let line_width_limit = match (truncate_width, wrap_width) {
        (Some(truncate), Some(wrap)) => Some(truncate.min(wrap)),
        (Some(truncate), None) => Some(truncate),
        (None, Some(wrap)) => Some(wrap),
        (None, None) => None,
    }?;

    if lines.is_empty() {
        return None;
    }

    let nowrap_truncate =
        white_space == WhiteSpace::Nowrap && lines.len() == 1 && lines[0].width > line_width_limit;
    if !line_clamped && !nowrap_truncate {
        return None;
    }

    let line_ix = if line_clamped { lines.len() - 1 } else { 0 };
    let Some(line) = lines.get(line_ix) else {
        return None;
    };

    let flat_runs = build_flat_decoration_runs(items);
    let flat_index = build_decoration_index(&flat_runs);
    let fallback_decoration = fallback_decoration_signature(items);
    let mut font_id = fallback_font_id_for_items(text_system, items);
    let mut style_key = EllipsisStyleKey {
        font_id,
        font_size,
        decoration: fallback_decoration.clone(),
        ellipsis_text: ellipsis_text.clone(),
    };
    let mut ellipsis_layout = ellipsis_layout_for_style(text_system, &style_key);
    let mut cut = compute_truncation_cut(
        line,
        islands,
        line_width_limit,
        ellipsis_layout.width,
        line_clamped,
    );

    if let Some((island_ix, text_end)) = cut.text_anchor {
        if let Some(anchor_font_id) = font_id_for_anchor(&islands[island_ix], text_end)
            && anchor_font_id != font_id
        {
            font_id = anchor_font_id;
            style_key.font_id = font_id;
            ellipsis_layout = ellipsis_layout_for_style(text_system, &style_key);
            cut = compute_truncation_cut(
                line,
                islands,
                line_width_limit,
                ellipsis_layout.width,
                line_clamped,
            );
        }
    }

    let decoration = cut
        .text_anchor
        .and_then(|(island_ix, text_end)| text_offset_for_anchor(islands, island_ix, text_end))
        .and_then(|offset| {
            style_at_text_offset(&flat_runs, &flat_index, offset).map(decoration_signature_from_run)
        })
        .unwrap_or_else(|| fallback_decoration.clone());

    let ellipsis_style = EllipsisStyleKey {
        font_id,
        font_size,
        decoration,
        ellipsis_text: ellipsis_text.clone(),
    };

    let (visible_height, visible_text_height) = compute_visible_metrics(
        line,
        islands,
        box_entries,
        line_height,
        cut.clip_x,
        cut.truncate_segment_ix,
        cut.truncate_text_end,
        cut.clip_first_item,
    );

    let mut visible_width = if cut.clip_first_item {
        line_width_limit.min(line.width)
    } else {
        cut.ellipsis_x + ellipsis_layout.width
    };
    visible_width = visible_width.min(line_width_limit);

    Some(InlineTruncationPlan {
        line_ix,
        clip_x: cut.clip_x,
        ellipsis_x: cut.ellipsis_x,
        visible_width,
        visible_height,
        visible_text_height,
        visible_logical_end: cut.visible_logical_end,
        ellipsis_style,
        truncate_segment_ix: cut.truncate_segment_ix,
        truncate_text_end: cut.truncate_text_end,
        clip_first_item: cut.clip_first_item,
    })
}

fn compute_truncation_cut(
    line: &InlineLine,
    islands: &[TextIsland],
    line_width_limit: Pixels,
    ellipsis_width: Pixels,
    line_clamped: bool,
) -> TruncationCut {
    let mut clip_first_item = false;
    let mut clip_x = if line_clamped && line.width + ellipsis_width <= line_width_limit {
        line.width
    } else {
        let mut clip_x = line_width_limit - ellipsis_width;
        if clip_x <= Pixels::ZERO {
            clip_first_item = true;
            clip_x = line_width_limit;
        }
        clip_x
    };

    if clip_x <= Pixels::ZERO {
        return TruncationCut {
            clip_x: Pixels::ZERO,
            ellipsis_x: Pixels::ZERO,
            truncate_segment_ix: None,
            truncate_text_end: None,
            clip_first_item: false,
            visible_logical_end: 0,
            text_anchor: None,
        };
    }

    let mut truncate_segment_ix = None;
    let mut truncate_text_end = None;
    let mut visible_logical_end = line_end_logical(line);
    let mut text_anchor = None;
    let mut visible_end_x = clip_x;

    for (ix, segment) in line.segments.iter().enumerate() {
        let segment_start = match segment {
            InlineSegment::Text { x, .. } => *x,
            InlineSegment::InlineBox { x, .. } => *x,
            InlineSegment::HardBreak { x, .. } => *x,
        };
        if segment_start >= clip_x {
            break;
        }
        match segment {
            InlineSegment::Text {
                island_ix,
                text_range,
                logical_range,
                layout_start_x,
                x,
                width,
                ..
            } => {
                let segment_end = *x + *width;
                if segment_end <= clip_x {
                    visible_logical_end = logical_range.end;
                    text_anchor = Some((*island_ix, text_range.end));
                    visible_end_x = segment_end;
                    continue;
                }

                let available = (clip_x - *x).max(Pixels::ZERO);
                let island = &islands[*island_ix];
                let layout_x = *layout_start_x + available;
                let mut text_ix = island
                    .layout
                    .index_for_x(layout_x)
                    .unwrap_or_else(|| island.layout.closest_index_for_x(layout_x));
                text_ix = text_ix.clamp(text_range.start, text_range.end);

                if clip_first_item || (text_ix <= text_range.start && ix == 0) {
                    clip_first_item = true;
                    clip_x = line_width_limit;
                    if let Some(next_ix) =
                        next_glyph_after(island, text_range.start, text_range.end)
                    {
                        text_ix = next_ix;
                    }
                    truncate_segment_ix = Some(ix);
                    truncate_text_end = Some(text_ix);
                    visible_logical_end = logical_range.start + (text_ix - text_range.start);
                    text_anchor = Some((*island_ix, text_ix));
                    visible_end_x = *x + (island.layout.x_for_index(text_ix) - *layout_start_x);
                    break;
                }

                truncate_segment_ix = Some(ix);
                truncate_text_end = Some(text_ix);
                visible_logical_end = logical_range.start + (text_ix - text_range.start);
                text_anchor = Some((*island_ix, text_ix));
                visible_end_x = *x + (island.layout.x_for_index(text_ix) - *layout_start_x);
                break;
            }
            InlineSegment::InlineBox {
                logical_range,
                x,
                width,
                ..
            } => {
                let segment_end = *x + *width;
                if segment_end <= clip_x {
                    visible_logical_end = logical_range.end;
                    visible_end_x = segment_end;
                    continue;
                }
                visible_end_x = *x;
                if ix == 0 {
                    clip_first_item = true;
                    clip_x = line_width_limit;
                    visible_logical_end = logical_range.end;
                }
                break;
            }
            InlineSegment::HardBreak { logical_range, .. } => {
                visible_logical_end = logical_range.start;
                visible_end_x = segment_start;
                break;
            }
        }
    }

    let ellipsis_x = visible_end_x.min(clip_x);

    TruncationCut {
        clip_x,
        ellipsis_x,
        truncate_segment_ix,
        truncate_text_end,
        clip_first_item,
        visible_logical_end,
        text_anchor,
    }
}

fn compute_visible_metrics(
    line: &InlineLine,
    islands: &[TextIsland],
    box_entries: &[BoxEntry],
    target_line_height: Pixels,
    clip_x: Pixels,
    truncate_segment_ix: Option<usize>,
    truncate_text_end: Option<usize>,
    clip_first_item: bool,
) -> (Pixels, Pixels) {
    let mut max_ascent = Pixels::ZERO;
    let mut max_descent = Pixels::ZERO;
    let mut max_box_above = Pixels::ZERO;
    let mut max_box_below = Pixels::ZERO;
    let mut has_text = false;

    for (ix, segment) in line.segments.iter().enumerate() {
        let segment_start = match segment {
            InlineSegment::Text { x, .. } => *x,
            InlineSegment::InlineBox { x, .. } => *x,
            InlineSegment::HardBreak { x, .. } => *x,
        };
        if segment_start >= clip_x {
            break;
        }
        match segment {
            InlineSegment::Text {
                island_ix,
                text_range,
                ..
            } => {
                if let (Some(trunc_ix), Some(trunc_end)) = (truncate_segment_ix, truncate_text_end)
                {
                    if trunc_ix == ix && trunc_end <= text_range.start && !clip_first_item {
                        break;
                    }
                }
                let Some(island) = islands.get(*island_ix) else {
                    continue;
                };
                max_ascent = max_ascent.max(island.layout.ascent);
                max_descent = max_descent.max(island.layout.descent);
                has_text = true;
            }
            InlineSegment::InlineBox {
                index, x, width, ..
            } => {
                let segment_end = *x + *width;
                if segment_end > clip_x && !clip_first_item {
                    break;
                }
                let metrics = &box_entries[*index].metrics;
                max_box_above = max_box_above.max(metrics.baseline + metrics.margin.top);
                let below =
                    (metrics.height - metrics.baseline).max(Pixels::ZERO) + metrics.margin.bottom;
                max_box_below = max_box_below.max(below);
            }
            InlineSegment::HardBreak { .. } => {}
        }
    }

    let text_height = max_ascent + max_descent;
    let extra_leading = (target_line_height - text_height).max(Pixels::ZERO);
    let line_ascent = if has_text { max_ascent } else { Pixels::ZERO };
    let line_descent = (target_line_height - line_ascent)
        .max((max_box_above + max_box_below - line_ascent).max(Pixels::ZERO));
    let line_height = line_ascent + line_descent;
    let text_height = max_ascent + max_descent + extra_leading;

    (line_height, text_height)
}

fn font_id_for_anchor(island: &TextIsland, text_end: usize) -> Option<FontId> {
    let anchor_ix = last_glyph_before(island, text_end)?;
    island.layout.font_id_for_index(anchor_ix)
}

fn last_glyph_before(island: &TextIsland, text_end: usize) -> Option<usize> {
    let mut last_ix = None;
    for glyph in &island.glyphs {
        if glyph.text_ix < text_end {
            last_ix = Some(glyph.text_ix);
        } else {
            break;
        }
    }
    last_ix
}

fn next_glyph_after(island: &TextIsland, start: usize, end: usize) -> Option<usize> {
    let mut saw_first = false;
    for glyph in &island.glyphs {
        if glyph.text_ix < start {
            continue;
        }
        if glyph.text_ix >= end {
            break;
        }
        if !saw_first {
            saw_first = true;
            continue;
        }
        return Some(glyph.text_ix);
    }

    if saw_first { Some(end) } else { None }
}

fn compute_intrinsic_widths(
    islands: &[TextIsland],
    box_entries: &[BoxEntry],
    flow_entries: &[FlowEntry],
) -> (Pixels, Pixels) {
    let mut max_unbreakable = Pixels::ZERO;
    for island in islands {
        max_unbreakable = max_unbreakable.max(max_unbreakable_width(island));
    }

    let mut widest_box = Pixels::ZERO;
    for entry in box_entries {
        let width = entry.metrics.width + entry.metrics.margin.left + entry.metrics.margin.right;
        widest_box = widest_box.max(width);
    }

    let intrinsic_min_width = max_unbreakable.max(widest_box);

    let mut current_line = Pixels::ZERO;
    let mut max_line = Pixels::ZERO;
    for entry in flow_entries {
        match entry {
            FlowEntry::Island(island_ix) => {
                current_line += islands[*island_ix].layout.width;
            }
            FlowEntry::InlineBox { index, .. } => {
                let entry = &box_entries[*index];
                current_line +=
                    entry.metrics.width + entry.metrics.margin.left + entry.metrics.margin.right;
            }
            FlowEntry::HardBreak => {
                max_line = max_line.max(current_line);
                current_line = Pixels::ZERO;
            }
        }
    }
    max_line = max_line.max(current_line);

    (intrinsic_min_width, max_line)
}

fn max_unbreakable_width(island: &TextIsland) -> Pixels {
    if island.glyphs.is_empty() {
        return Pixels::ZERO;
    }

    let mut max_width = Pixels::ZERO;
    let mut last_boundary_x = Pixels::ZERO;
    let mut prev_ch = '\0';
    let mut first_non_whitespace = false;

    for glyph in &island.glyphs {
        let ch = glyph.ch;
        if ch == '\n' {
            continue;
        }

        if ch != ' ' && !first_non_whitespace {
            first_non_whitespace = true;
        }

        if first_non_whitespace {
            let is_word = LineWrapper::is_word_char(ch);
            let candidate = if is_word {
                prev_ch == ' ' && ch != ' '
            } else {
                ch != ' '
            };
            if candidate {
                let width = glyph.x - last_boundary_x;
                max_width = max_width.max(width);
                last_boundary_x = glyph.x;
            }
        }

        prev_ch = ch;
    }

    let width = island.layout.width - last_boundary_x;
    max_width.max(width)
}

fn build_flat_decoration_runs(items: &[InlineFlowItem]) -> SmallVec<[DecorationRun; 32]> {
    let mut runs = SmallVec::<[DecorationRun; 32]>::new();
    for item in items {
        if let InlineFlowItem::Text {
            runs: text_runs, ..
        } = item
        {
            for run in text_runs.iter() {
                if let Some(last) = runs.last_mut()
                    && last.color == run.color
                    && last.underline == run.underline
                    && last.strikethrough == run.strikethrough
                    && last.background_color == run.background_color
                {
                    last.len += run.len as u32;
                    continue;
                }
                runs.push(DecorationRun {
                    len: run.len as u32,
                    color: run.color,
                    background_color: run.background_color,
                    underline: run.underline,
                    strikethrough: run.strikethrough,
                });
            }
        }
    }
    runs
}

fn build_decoration_index(runs: &[DecorationRun]) -> DecorationIndex {
    let mut prefix_end = SmallVec::<[usize; 64]>::with_capacity(runs.len());
    let mut offset = 0;
    for run in runs {
        offset += run.len as usize;
        prefix_end.push(offset);
    }
    DecorationIndex { prefix_end }
}

fn slice_decoration_runs(
    runs: &[DecorationRun],
    index: &DecorationIndex,
    range: Range<usize>,
) -> SmallVec<[DecorationRun; 32]> {
    if runs.is_empty() || range.is_empty() {
        return SmallVec::new();
    }

    let run_start = index.prefix_end.iter().position(|end| *end > range.start);
    let run_end = index.prefix_end.iter().position(|end| *end >= range.end);
    let (Some(run_start), Some(run_end)) = (run_start, run_end) else {
        return SmallVec::new();
    };

    let run_start_byte = if run_start == 0 {
        0
    } else {
        index.prefix_end[run_start - 1]
    };
    let run_end_byte = if run_end == 0 {
        0
    } else {
        index.prefix_end[run_end - 1]
    };
    let start_offset = range.start - run_start_byte;
    let end_offset = range.end - run_end_byte;

    let mut sliced = SmallVec::<[DecorationRun; 32]>::new();
    for run_ix in run_start..=run_end {
        let run = &runs[run_ix];
        let run_start_offset = if run_ix == run_start { start_offset } else { 0 };
        let run_end_offset = if run_ix == run_end {
            end_offset
        } else {
            run.len as usize
        };
        if run_end_offset <= run_start_offset {
            continue;
        }
        sliced.push(DecorationRun {
            len: (run_end_offset - run_start_offset) as u32,
            color: run.color,
            background_color: run.background_color,
            underline: run.underline,
            strikethrough: run.strikethrough,
        });
    }

    sliced
}

fn style_at_text_offset<'a>(
    runs: &'a [DecorationRun],
    index: &'a DecorationIndex,
    offset: usize,
) -> Option<&'a DecorationRun> {
    let run_ix = index.prefix_end.iter().position(|end| *end >= offset)?;
    runs.get(run_ix)
}

pub(crate) fn decoration_slice_spec_for_range_indexed(
    index: &DecorationIndex,
    range: Range<usize>,
) -> DecorationSliceSpec {
    if index.prefix_end.is_empty() {
        return DecorationSliceSpec {
            run_start: 0,
            run_end: 0,
            start_offset: 0,
            end_offset: 0,
        };
    }

    let run_start = index.prefix_end.partition_point(|end| *end <= range.start);
    let mut run_end = index.prefix_end.partition_point(|end| *end < range.end);
    if run_end >= index.prefix_end.len() {
        run_end = index.prefix_end.len().saturating_sub(1);
    }

    let run_start_byte = if run_start == 0 {
        0
    } else {
        index.prefix_end[run_start - 1]
    };
    let run_end_byte = if run_end == 0 {
        0
    } else {
        index.prefix_end[run_end - 1]
    };

    DecorationSliceSpec {
        run_start,
        run_end,
        start_offset: range.start - run_start_byte,
        end_offset: range.end - run_end_byte,
    }
}

struct DecorationSliceIter<'a> {
    runs: &'a [DecorationRun],
    spec: DecorationSliceSpec,
    run_end: usize,
    run_ix: usize,
}

impl<'a> DecorationSliceIter<'a> {
    fn new(runs: &'a [DecorationRun], spec: DecorationSliceSpec) -> Self {
        let run_end = if runs.is_empty() {
            0
        } else {
            spec.run_end.min(runs.len() - 1)
        };
        Self {
            runs,
            spec,
            run_end,
            run_ix: spec.run_start,
        }
    }
}

impl Iterator for DecorationSliceIter<'_> {
    type Item = DecorationRun;

    fn next(&mut self) -> Option<Self::Item> {
        if self.runs.is_empty() || self.spec.run_start >= self.runs.len() {
            return None;
        }

        while self.run_ix <= self.run_end {
            let run_ix = self.run_ix;
            let run = &self.runs[run_ix];
            self.run_ix += 1;

            let start_offset = if run_ix == self.spec.run_start {
                self.spec.start_offset
            } else {
                0
            };
            let end_offset = if run_ix == self.run_end {
                self.spec.end_offset
            } else {
                run.len as usize
            };
            let end_offset = end_offset.min(run.len as usize);
            if end_offset <= start_offset {
                continue;
            }
            return Some(DecorationRun {
                len: (end_offset - start_offset) as u32,
                color: run.color,
                background_color: run.background_color,
                underline: run.underline,
                strikethrough: run.strikethrough,
            });
        }

        None
    }
}

pub(crate) fn paint_inline_text_range(
    layout: &LineLayout,
    decoration_runs: &[DecorationRun],
    decoration_span: DecorationSliceSpec,
    text_range: Range<usize>,
    origin: Point<Pixels>,
    line_height: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if text_range.start >= text_range.end || layout.len == 0 {
        return Ok(());
    }

    let range_end = text_range.end.min(layout.len);
    let range_start = text_range.start.min(range_end);
    if range_start >= range_end {
        return Ok(());
    }

    let range_len = range_end - range_start;
    let padding_top = (line_height - layout.ascent - layout.descent) / 2.;
    let baseline_offset = point(px(0.), padding_top + layout.ascent);
    let mut decoration_runs = DecorationSliceIter::new(decoration_runs, decoration_span);
    let mut run_end = 0;
    let mut color = black();
    let mut current_underline: Option<(Point<Pixels>, UnderlineStyle)> = None;
    let mut current_strikethrough: Option<(Point<Pixels>, StrikethroughStyle)> = None;
    let text_system = cx.text_system().clone();
    let mut glyph_origin = origin;
    let mut prev_glyph_position = Point::default();
    let mut max_glyph_size = size(px(0.), px(0.));

    'paint: for run in &layout.runs {
        max_glyph_size = text_system.bounding_box(run.font_id, layout.font_size).size;

        for glyph in &run.glyphs {
            glyph_origin.x += glyph.position.x - prev_glyph_position.x;
            prev_glyph_position = glyph.position;

            if glyph.index < range_start {
                continue;
            }
            if glyph.index >= range_end {
                break 'paint;
            }

            let glyph_offset = glyph.index - range_start;
            let mut finished_underline: Option<(Point<Pixels>, UnderlineStyle)> = None;
            let mut finished_strikethrough: Option<(Point<Pixels>, StrikethroughStyle)> = None;
            if glyph_offset >= run_end {
                let mut style_run = decoration_runs.next();

                // ignore style runs that apply to a partial glyph
                while let Some(ref run) = style_run {
                    if glyph_offset < run_end + (run.len as usize) {
                        break;
                    }
                    run_end += run.len as usize;
                    style_run = decoration_runs.next();
                }

                if let Some(style_run) = style_run {
                    if let Some((_, underline_style)) = &mut current_underline
                        && style_run.underline.as_ref() != Some(underline_style)
                    {
                        finished_underline = current_underline.take();
                    }
                    if let Some(run_underline) = style_run.underline.as_ref() {
                        current_underline.get_or_insert((
                            point(
                                glyph_origin.x,
                                glyph_origin.y + baseline_offset.y + (layout.descent * 0.618),
                            ),
                            UnderlineStyle {
                                color: Some(run_underline.color.unwrap_or(style_run.color)),
                                thickness: run_underline.thickness,
                                wavy: run_underline.wavy,
                            },
                        ));
                    }
                    if let Some((_, strikethrough_style)) = &mut current_strikethrough
                        && style_run.strikethrough.as_ref() != Some(strikethrough_style)
                    {
                        finished_strikethrough = current_strikethrough.take();
                    }
                    if let Some(run_strikethrough) = style_run.strikethrough.as_ref() {
                        current_strikethrough.get_or_insert((
                            point(
                                glyph_origin.x,
                                glyph_origin.y
                                    + (((layout.ascent * 0.5) + baseline_offset.y) * 0.5),
                            ),
                            StrikethroughStyle {
                                color: Some(run_strikethrough.color.unwrap_or(style_run.color)),
                                thickness: run_strikethrough.thickness,
                            },
                        ));
                    }

                    run_end += style_run.len as usize;
                    color = style_run.color;
                } else {
                    run_end = range_len;
                    finished_underline = current_underline.take();
                    finished_strikethrough = current_strikethrough.take();
                }
            }

            if let Some((mut underline_origin, underline_style)) = finished_underline {
                if underline_origin.x == glyph_origin.x {
                    underline_origin.x -= max_glyph_size.width.half();
                };
                window.paint_underline(
                    underline_origin,
                    glyph_origin.x - underline_origin.x,
                    &underline_style,
                );
            }

            if let Some((mut strikethrough_origin, strikethrough_style)) = finished_strikethrough {
                if strikethrough_origin.x == glyph_origin.x {
                    strikethrough_origin.x -= max_glyph_size.width.half();
                };
                window.paint_strikethrough(
                    strikethrough_origin,
                    glyph_origin.x - strikethrough_origin.x,
                    &strikethrough_style,
                );
            }

            let max_glyph_bounds = Bounds {
                origin: glyph_origin,
                size: max_glyph_size,
            };

            let content_mask = window.content_mask();
            if max_glyph_bounds.intersects(&content_mask.bounds) {
                let vertical_offset = point(px(0.0), glyph.position.y);
                if glyph.is_emoji {
                    window.paint_emoji(
                        glyph_origin + baseline_offset + vertical_offset,
                        run.font_id,
                        glyph.id,
                        layout.font_size,
                    )?;
                } else {
                    window.paint_glyph(
                        glyph_origin + baseline_offset + vertical_offset,
                        run.font_id,
                        glyph.id,
                        layout.font_size,
                        color,
                    )?;
                }
            }
        }
    }

    let range_end_x = origin.x + layout.x_for_index(range_end);
    if let Some((mut underline_start, underline_style)) = current_underline.take() {
        if range_end_x == underline_start.x {
            underline_start.x -= max_glyph_size.width.half()
        };
        window.paint_underline(
            underline_start,
            range_end_x - underline_start.x,
            &underline_style,
        );
    }

    if let Some((mut strikethrough_start, strikethrough_style)) = current_strikethrough.take() {
        if range_end_x == strikethrough_start.x {
            strikethrough_start.x -= max_glyph_size.width.half()
        };
        window.paint_strikethrough(
            strikethrough_start,
            range_end_x - strikethrough_start.x,
            &strikethrough_style,
        );
    }

    Ok(())
}

pub(crate) fn paint_inline_background_range(
    layout: &LineLayout,
    decoration_runs: &[DecorationRun],
    decoration_span: DecorationSliceSpec,
    text_range: Range<usize>,
    origin: Point<Pixels>,
    line_height: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if text_range.start >= text_range.end || layout.len == 0 {
        return Ok(());
    }

    let range_end = text_range.end.min(layout.len);
    let range_start = text_range.start.min(range_end);
    if range_start >= range_end {
        return Ok(());
    }

    let range_len = range_end - range_start;
    let mut decoration_runs = DecorationSliceIter::new(decoration_runs, decoration_span);
    let mut run_end = 0;
    let mut current_background: Option<(Point<Pixels>, Hsla)> = None;
    let text_system = cx.text_system().clone();
    let mut glyph_origin = origin;
    let mut prev_glyph_position = Point::default();
    let mut max_glyph_size = size(px(0.), px(0.));

    'paint: for run in &layout.runs {
        max_glyph_size = text_system.bounding_box(run.font_id, layout.font_size).size;

        for glyph in &run.glyphs {
            glyph_origin.x += glyph.position.x - prev_glyph_position.x;
            prev_glyph_position = glyph.position;

            if glyph.index < range_start {
                continue;
            }
            if glyph.index >= range_end {
                break 'paint;
            }

            let glyph_offset = glyph.index - range_start;
            let mut finished_background: Option<(Point<Pixels>, Hsla)> = None;
            if glyph_offset >= run_end {
                let mut style_run = decoration_runs.next();

                // ignore style runs that apply to a partial glyph
                while let Some(ref run) = style_run {
                    if glyph_offset < run_end + (run.len as usize) {
                        break;
                    }
                    run_end += run.len as usize;
                    style_run = decoration_runs.next();
                }

                if let Some(style_run) = style_run {
                    if let Some((_, background_color)) = &mut current_background
                        && style_run.background_color.as_ref() != Some(background_color)
                    {
                        finished_background = current_background.take();
                    }
                    if let Some(run_background) = style_run.background_color {
                        current_background
                            .get_or_insert((point(glyph_origin.x, glyph_origin.y), run_background));
                    }
                    run_end += style_run.len as usize;
                } else {
                    run_end = range_len;
                    finished_background = current_background.take();
                }
            }

            if let Some((mut background_origin, background_color)) = finished_background {
                if background_origin.x == glyph_origin.x {
                    background_origin.x -= max_glyph_size.width.half();
                };
                window.paint_quad(fill(
                    Bounds {
                        origin: background_origin,
                        size: size(glyph_origin.x - background_origin.x, line_height),
                    },
                    background_color,
                ));
            }
        }
    }

    let range_end_x = origin.x + layout.x_for_index(range_end);
    if let Some((mut background_origin, background_color)) = current_background.take() {
        if range_end_x == background_origin.x {
            background_origin.x -= max_glyph_size.width.half()
        };
        window.paint_quad(fill(
            Bounds {
                origin: background_origin,
                size: size(range_end_x - background_origin.x, line_height),
            },
            background_color,
        ));
    }

    Ok(())
}

#[derive(Clone)]
struct BufferedGlyph {
    origin: Point<Pixels>,
    vertical_offset: Pixels,
    font_id: FontId,
    glyph_id: GlyphId,
    is_emoji: bool,
    paint: bool,
}

fn flush_inline_span_run(
    layout: &LineLayout,
    baseline_offset: Point<Pixels>,
    line_height: Pixels,
    window: &mut Window,
    glyphs: &mut SmallVec<[BufferedGlyph; 32]>,
    run_origin: &mut Option<Point<Pixels>>,
    current_background: &mut Option<Hsla>,
    current_underline: &mut Option<UnderlineStyle>,
    underline_origin: &mut Option<Point<Pixels>>,
    current_strikethrough: &mut Option<StrikethroughStyle>,
    strikethrough_origin: &mut Option<Point<Pixels>>,
    color: Hsla,
    end_x: Pixels,
    max_glyph_size: Size<Pixels>,
    finish_underline: bool,
    finish_strikethrough: bool,
) -> Result<()> {
    if let Some(run_origin) = *run_origin {
        if let Some(background_color) = *current_background {
            let mut background_origin = run_origin;
            if end_x == background_origin.x {
                background_origin.x -= max_glyph_size.width.half();
            }
            window.paint_quad(fill(
                Bounds {
                    origin: background_origin,
                    size: size(end_x - background_origin.x, line_height),
                },
                background_color,
            ));
        }

        for glyph in glyphs.iter() {
            if !glyph.paint {
                continue;
            }
            let vertical_offset = point(px(0.0), glyph.vertical_offset);
            if glyph.is_emoji {
                window.paint_emoji(
                    glyph.origin + baseline_offset + vertical_offset,
                    glyph.font_id,
                    glyph.glyph_id,
                    layout.font_size,
                )?;
            } else {
                window.paint_glyph(
                    glyph.origin + baseline_offset + vertical_offset,
                    glyph.font_id,
                    glyph.glyph_id,
                    layout.font_size,
                    color,
                )?;
            }
        }
    }

    if finish_underline
        && let (Some(mut origin), Some(style)) = (*underline_origin, *current_underline)
    {
        if end_x == origin.x {
            origin.x -= max_glyph_size.width.half();
        }
        window.paint_underline(origin, end_x - origin.x, &style);
    }

    if finish_strikethrough
        && let (Some(mut origin), Some(style)) = (*strikethrough_origin, *current_strikethrough)
    {
        if end_x == origin.x {
            origin.x -= max_glyph_size.width.half();
        }
        window.paint_strikethrough(origin, end_x - origin.x, &style);
    }

    glyphs.clear();
    *run_origin = None;
    *current_background = None;
    if finish_underline {
        *current_underline = None;
        *underline_origin = None;
    }
    if finish_strikethrough {
        *current_strikethrough = None;
        *strikethrough_origin = None;
    }

    Ok(())
}

pub(crate) fn paint_inline_span(
    layout: &LineLayout,
    decoration_runs: &[DecorationRun],
    decoration_span: DecorationSliceSpec,
    text_range: Range<usize>,
    glyph_range: TextGlyphRange,
    origin: Point<Pixels>,
    line_height: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if text_range.start >= text_range.end || layout.len == 0 {
        return Ok(());
    }

    let range_end = text_range.end.min(layout.len);
    let range_start = text_range.start.min(range_end);
    if range_start >= range_end {
        return Ok(());
    }

    let range_len = range_end - range_start;
    let padding_top = (line_height - layout.ascent - layout.descent) / 2.;
    let baseline_offset = point(px(0.), padding_top + layout.ascent);
    let mut decoration_runs = DecorationSliceIter::new(decoration_runs, decoration_span);
    let mut run_end = 0;
    let mut color = black();
    let mut current_background: Option<Hsla> = None;
    let mut current_underline: Option<UnderlineStyle> = None;
    let mut current_strikethrough: Option<StrikethroughStyle> = None;
    let mut underline_origin: Option<Point<Pixels>> = None;
    let mut strikethrough_origin: Option<Point<Pixels>> = None;
    let text_system = cx.text_system().clone();
    let content_mask = window.content_mask();
    let mut glyph_origin = origin + point(glyph_range.start_position.x, px(0.));
    let mut prev_glyph_position = glyph_range.start_position;
    let mut max_glyph_size = size(px(0.), px(0.));
    let mut run_origin: Option<Point<Pixels>> = None;
    let mut glyphs: SmallVec<[BufferedGlyph; 32]> = SmallVec::new();

    if glyph_range.start_run > glyph_range.end_run || glyph_range.start_run >= layout.runs.len() {
        return Ok(());
    }

    for (run_offset, run) in layout.runs[glyph_range.start_run..=glyph_range.end_run]
        .iter()
        .enumerate()
    {
        let run_ix = glyph_range.start_run + run_offset;
        max_glyph_size = text_system.bounding_box(run.font_id, layout.font_size).size;

        let glyphs_in_run = &run.glyphs;
        let start = if run_ix == glyph_range.start_run {
            glyph_range.start_glyph
        } else {
            0
        };
        let end = if run_ix == glyph_range.end_run {
            glyph_range.end_glyph.min(glyphs_in_run.len())
        } else {
            glyphs_in_run.len()
        };

        if start >= end {
            continue;
        }

        for glyph in &glyphs_in_run[start..end] {
            glyph_origin.x += glyph.position.x - prev_glyph_position.x;
            prev_glyph_position = glyph.position;

            let glyph_offset = glyph.index - range_start;
            if glyph_offset >= run_end {
                let mut style_run = decoration_runs.next();
                while let Some(ref run) = style_run {
                    if glyph_offset < run_end + (run.len as usize) {
                        break;
                    }
                    run_end += run.len as usize;
                    style_run = decoration_runs.next();
                }

                let next_underline = style_run.as_ref().and_then(|run| {
                    run.underline.as_ref().map(|run_underline| UnderlineStyle {
                        color: Some(run_underline.color.unwrap_or(run.color)),
                        thickness: run_underline.thickness,
                        wavy: run_underline.wavy,
                    })
                });
                let next_strikethrough = style_run.as_ref().and_then(|run| {
                    run.strikethrough
                        .as_ref()
                        .map(|run_strikethrough| StrikethroughStyle {
                            color: Some(run_strikethrough.color.unwrap_or(run.color)),
                            thickness: run_strikethrough.thickness,
                        })
                });
                let finish_underline = current_underline != next_underline;
                let finish_strikethrough = current_strikethrough != next_strikethrough;

                flush_inline_span_run(
                    layout,
                    baseline_offset,
                    line_height,
                    window,
                    &mut glyphs,
                    &mut run_origin,
                    &mut current_background,
                    &mut current_underline,
                    &mut underline_origin,
                    &mut current_strikethrough,
                    &mut strikethrough_origin,
                    color,
                    glyph_origin.x,
                    max_glyph_size,
                    finish_underline,
                    finish_strikethrough,
                )?;

                if let Some(ref style_run) = style_run {
                    if let Some(run_background) = style_run.background_color {
                        current_background = Some(run_background);
                    }
                    if finish_underline {
                        current_underline = next_underline;
                        underline_origin = current_underline.as_ref().map(|_| {
                            point(
                                glyph_origin.x,
                                glyph_origin.y + baseline_offset.y + (layout.descent * 0.618),
                            )
                        });
                    }
                    if finish_strikethrough {
                        current_strikethrough = next_strikethrough;
                        strikethrough_origin = current_strikethrough.as_ref().map(|_| {
                            point(
                                glyph_origin.x,
                                glyph_origin.y
                                    + (((layout.ascent * 0.5) + baseline_offset.y) * 0.5),
                            )
                        });
                    }
                    run_end += style_run.len as usize;
                    color = style_run.color;
                } else {
                    run_end = range_len;
                }

                run_origin = Some(glyph_origin);
            }

            let max_glyph_bounds = Bounds {
                origin: glyph_origin,
                size: max_glyph_size,
            };
            let paint = max_glyph_bounds.intersects(&content_mask.bounds);
            glyphs.push(BufferedGlyph {
                origin: glyph_origin,
                vertical_offset: glyph.position.y,
                font_id: run.font_id,
                glyph_id: glyph.id,
                is_emoji: glyph.is_emoji,
                paint,
            });
        }
    }

    let range_end_x = origin.x + layout.x_for_index(range_end);
    let finish_underline = current_underline.is_some();
    let finish_strikethrough = current_strikethrough.is_some();
    flush_inline_span_run(
        layout,
        baseline_offset,
        line_height,
        window,
        &mut glyphs,
        &mut run_origin,
        &mut current_background,
        &mut current_underline,
        &mut underline_origin,
        &mut current_strikethrough,
        &mut strikethrough_origin,
        color,
        range_end_x,
        max_glyph_size,
        finish_underline,
        finish_strikethrough,
    )?;

    Ok(())
}
