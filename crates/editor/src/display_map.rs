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
//! Each one of those builds on top of preceding map.
//!
//! [Editor]: crate::Editor
//! [EditorElement]: crate::element::EditorElement

mod block_map;
mod flap_map;
mod fold_map;
mod inlay_map;
mod tab_map;
mod wrap_map;

use crate::{
    hover_links::InlayHighlight, movement::TextLayoutDetails, EditorStyle, InlayId, RowExt,
};
pub use block_map::{
    BlockBufferRows, BlockChunks as DisplayChunks, BlockContext, BlockDisposition, BlockId,
    BlockMap, BlockPoint, BlockProperties, BlockStyle, RenderBlock, TransformBlock,
};
use block_map::{BlockRow, BlockSnapshot};
use collections::{HashMap, HashSet};
pub use flap_map::*;
pub use fold_map::{Fold, FoldId, FoldPlaceholder, FoldPoint};
use fold_map::{FoldMap, FoldSnapshot};
use gpui::{
    AnyElement, Font, HighlightStyle, LineLayout, Model, ModelContext, Pixels, UnderlineStyle,
};
pub(crate) use inlay_map::Inlay;
use inlay_map::{InlayMap, InlaySnapshot};
pub use inlay_map::{InlayOffset, InlayPoint};
use language::{
    language_settings::language_settings, ChunkRenderer, OffsetUtf16, Point,
    Subscription as BufferSubscription,
};
use lsp::DiagnosticSeverity;
use multi_buffer::{
    Anchor, AnchorRangeExt, MultiBuffer, MultiBufferPoint, MultiBufferRow, MultiBufferSnapshot,
    ToOffset, ToPoint,
};
use serde::Deserialize;
use std::ops::Add;
use std::{any::TypeId, borrow::Cow, fmt::Debug, num::NonZeroU32, ops::Range, sync::Arc};
use sum_tree::{Bias, TreeMap};
use tab_map::{TabMap, TabSnapshot};
use text::LineIndent;
use ui::WindowContext;
use wrap_map::{WrapMap, WrapSnapshot};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FoldStatus {
    Folded,
    Foldable,
}

pub type RenderFoldToggle = Arc<dyn Fn(FoldStatus, &mut WindowContext) -> AnyElement>;

const UNNECESSARY_CODE_FADE: f32 = 0.3;

pub trait ToDisplayPoint {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint;
}

type TextHighlights = TreeMap<Option<TypeId>, Arc<(HighlightStyle, Vec<Range<Anchor>>)>>;
type InlayHighlights = TreeMap<TypeId, TreeMap<InlayId, (HighlightStyle, InlayHighlight)>>;

/// Decides how text in a [`MultiBuffer`] should be displayed in a buffer, handling inlay hints,
/// folding, hard tabs, soft wrapping, custom blocks (like diagnostics), and highlighting.
///
/// See the [module level documentation](self) for more information.
pub struct DisplayMap {
    /// The buffer that we are displaying.
    buffer: Model<MultiBuffer>,
    buffer_subscription: BufferSubscription,
    /// Decides where the [`Inlay`]s should be displayed.
    inlay_map: InlayMap,
    /// Decides where the fold indicators should be and tracks parts of a source file that are currently folded.
    fold_map: FoldMap,
    /// Keeps track of hard tabs in a buffer.
    tab_map: TabMap,
    /// Handles soft wrapping.
    wrap_map: Model<WrapMap>,
    /// Tracks custom blocks such as diagnostics that should be displayed within buffer.
    block_map: BlockMap,
    /// Regions of text that should be highlighted.
    text_highlights: TextHighlights,
    /// Regions of inlays that should be highlighted.
    inlay_highlights: InlayHighlights,
    /// A container for explicitly foldable ranges, which supersede indentation based fold range suggestions.
    flap_map: FlapMap,
    fold_placeholder: FoldPlaceholder,
    pub clip_at_line_ends: bool,
}

impl DisplayMap {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        buffer: Model<MultiBuffer>,
        font: Font,
        font_size: Pixels,
        wrap_width: Option<Pixels>,
        show_excerpt_controls: bool,
        buffer_header_height: u8,
        excerpt_header_height: u8,
        excerpt_footer_height: u8,
        fold_placeholder: FoldPlaceholder,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let buffer_subscription = buffer.update(cx, |buffer, _| buffer.subscribe());

        let tab_size = Self::tab_size(&buffer, cx);
        let (inlay_map, snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
        let (fold_map, snapshot) = FoldMap::new(snapshot);
        let (tab_map, snapshot) = TabMap::new(snapshot, tab_size);
        let (wrap_map, snapshot) = WrapMap::new(snapshot, font, font_size, wrap_width, cx);
        let block_map = BlockMap::new(
            snapshot,
            show_excerpt_controls,
            buffer_header_height,
            excerpt_header_height,
            excerpt_footer_height,
        );
        let flap_map = FlapMap::default();

        cx.observe(&wrap_map, |_, _, cx| cx.notify()).detach();

        DisplayMap {
            buffer,
            buffer_subscription,
            fold_map,
            inlay_map,
            tab_map,
            wrap_map,
            block_map,
            flap_map,
            fold_placeholder,
            text_highlights: Default::default(),
            inlay_highlights: Default::default(),
            clip_at_line_ends: false,
        }
    }

    pub fn snapshot(&mut self, cx: &mut ModelContext<Self>) -> DisplaySnapshot {
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let (inlay_snapshot, edits) = self.inlay_map.sync(buffer_snapshot, edits);
        let (fold_snapshot, edits) = self.fold_map.read(inlay_snapshot.clone(), edits);
        let tab_size = Self::tab_size(&self.buffer, cx);
        let (tab_snapshot, edits) = self.tab_map.sync(fold_snapshot.clone(), edits, tab_size);
        let (wrap_snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(tab_snapshot.clone(), edits, cx));
        let block_snapshot = self.block_map.read(wrap_snapshot.clone(), edits);

        DisplaySnapshot {
            buffer_snapshot: self.buffer.read(cx).snapshot(cx),
            fold_snapshot,
            inlay_snapshot,
            tab_snapshot,
            wrap_snapshot,
            block_snapshot,
            flap_snapshot: self.flap_map.snapshot(),
            text_highlights: self.text_highlights.clone(),
            inlay_highlights: self.inlay_highlights.clone(),
            clip_at_line_ends: self.clip_at_line_ends,
            fold_placeholder: self.fold_placeholder.clone(),
        }
    }

    pub fn set_state(&mut self, other: &DisplaySnapshot, cx: &mut ModelContext<Self>) {
        self.fold(
            other
                .folds_in_range(0..other.buffer_snapshot.len())
                .map(|fold| {
                    (
                        fold.range.to_offset(&other.buffer_snapshot),
                        fold.placeholder.clone(),
                    )
                }),
            cx,
        );
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = (Range<T>, FoldPlaceholder)>,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let tab_size = Self::tab_size(&self.buffer, cx);
        let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
        let (snapshot, edits) = fold_map.fold(ranges);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        inclusive: bool,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let tab_size = Self::tab_size(&self.buffer, cx);
        let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
        let (snapshot, edits) = fold_map.unfold(ranges, inclusive);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
    }

    pub fn insert_flaps(
        &mut self,
        flaps: impl IntoIterator<Item = Flap>,
        cx: &mut ModelContext<Self>,
    ) -> Vec<FlapId> {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        self.flap_map.insert(flaps, &snapshot)
    }

    pub fn remove_flaps(
        &mut self,
        flap_ids: impl IntoIterator<Item = FlapId>,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        self.flap_map.remove(flap_ids, &snapshot)
    }

    pub fn insert_blocks(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
        cx: &mut ModelContext<Self>,
    ) -> Vec<BlockId> {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let tab_size = Self::tab_size(&self.buffer, cx);
        let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        let mut block_map = self.block_map.write(snapshot, edits);
        block_map.insert(blocks)
    }

    pub fn replace_blocks(&mut self, styles: HashMap<BlockId, RenderBlock>) {
        self.block_map.replace(styles);
    }

    pub fn remove_blocks(&mut self, ids: HashSet<BlockId>, cx: &mut ModelContext<Self>) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let tab_size = Self::tab_size(&self.buffer, cx);
        let (snapshot, edits) = self.inlay_map.sync(snapshot, edits);
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        let mut block_map = self.block_map.write(snapshot, edits);
        block_map.remove(ids);
    }

    pub fn highlight_text(
        &mut self,
        type_id: TypeId,
        ranges: Vec<Range<Anchor>>,
        style: HighlightStyle,
    ) {
        self.text_highlights
            .insert(Some(type_id), Arc::new((style, ranges)));
    }

    pub(crate) fn highlight_inlays(
        &mut self,
        type_id: TypeId,
        highlights: Vec<InlayHighlight>,
        style: HighlightStyle,
    ) {
        for highlight in highlights {
            let update = self.inlay_highlights.update(&type_id, |highlights| {
                highlights.insert(highlight.inlay, (style, highlight.clone()))
            });
            if update.is_none() {
                self.inlay_highlights.insert(
                    type_id,
                    TreeMap::from_ordered_entries([(highlight.inlay, (style, highlight))]),
                );
            }
        }
    }

    pub fn text_highlights(&self, type_id: TypeId) -> Option<(HighlightStyle, &[Range<Anchor>])> {
        let highlights = self.text_highlights.get(&Some(type_id))?;
        Some((highlights.0, &highlights.1))
    }
    pub fn clear_highlights(&mut self, type_id: TypeId) -> bool {
        let mut cleared = self.text_highlights.remove(&Some(type_id)).is_some();
        cleared |= self.inlay_highlights.remove(&type_id).is_some();
        cleared
    }

    pub fn set_font(&self, font: Font, font_size: Pixels, cx: &mut ModelContext<Self>) -> bool {
        self.wrap_map
            .update(cx, |map, cx| map.set_font_with_size(font, font_size, cx))
    }

    pub fn set_wrap_width(&self, width: Option<Pixels>, cx: &mut ModelContext<Self>) -> bool {
        self.wrap_map
            .update(cx, |map, cx| map.set_wrap_width(width, cx))
    }

    pub(crate) fn current_inlays(&self) -> impl Iterator<Item = &Inlay> {
        self.inlay_map.current_inlays()
    }

    pub(crate) fn splice_inlays(
        &mut self,
        to_remove: Vec<InlayId>,
        to_insert: Vec<Inlay>,
        cx: &mut ModelContext<Self>,
    ) {
        if to_remove.is_empty() && to_insert.is_empty() {
            return;
        }
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let (snapshot, edits) = self.inlay_map.sync(buffer_snapshot, edits);
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let tab_size = Self::tab_size(&self.buffer, cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);

        let (snapshot, edits) = self.inlay_map.splice(to_remove, to_insert);
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits, tab_size);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
    }

    fn tab_size(buffer: &Model<MultiBuffer>, cx: &mut ModelContext<Self>) -> NonZeroU32 {
        let language = buffer
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).language());
        language_settings(language, None, cx).tab_size
    }

    #[cfg(test)]
    pub fn is_rewrapping(&self, cx: &gpui::AppContext) -> bool {
        self.wrap_map.read(cx).is_rewrapping()
    }

    pub fn show_excerpt_controls(&self) -> bool {
        self.block_map.show_excerpt_controls()
    }
}

#[derive(Debug, Default)]
pub(crate) struct Highlights<'a> {
    pub text_highlights: Option<&'a TextHighlights>,
    pub inlay_highlights: Option<&'a InlayHighlights>,
    pub styles: HighlightStyles,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct HighlightStyles {
    pub inlay_hint: Option<HighlightStyle>,
    pub suggestion: Option<HighlightStyle>,
}

pub struct HighlightedChunk<'a> {
    pub text: &'a str,
    pub style: Option<HighlightStyle>,
    pub is_tab: bool,
    pub renderer: Option<ChunkRenderer>,
}

#[derive(Clone)]
pub struct DisplaySnapshot {
    pub buffer_snapshot: MultiBufferSnapshot,
    pub fold_snapshot: FoldSnapshot,
    pub flap_snapshot: FlapSnapshot,
    inlay_snapshot: InlaySnapshot,
    tab_snapshot: TabSnapshot,
    wrap_snapshot: WrapSnapshot,
    block_snapshot: BlockSnapshot,
    text_highlights: TextHighlights,
    inlay_highlights: InlayHighlights,
    clip_at_line_ends: bool,
    pub(crate) fold_placeholder: FoldPlaceholder,
}

impl DisplaySnapshot {
    #[cfg(test)]
    pub fn fold_count(&self) -> usize {
        self.fold_snapshot.fold_count()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer_snapshot.len() == 0
    }

    pub fn buffer_rows(
        &self,
        start_row: DisplayRow,
    ) -> impl Iterator<Item = Option<MultiBufferRow>> + '_ {
        self.block_snapshot
            .buffer_rows(BlockRow(start_row.0))
            .map(|row| row.map(|row| MultiBufferRow(row.0)))
    }

    pub fn max_buffer_row(&self) -> MultiBufferRow {
        self.buffer_snapshot.max_buffer_row()
    }

    pub fn prev_line_boundary(&self, mut point: MultiBufferPoint) -> (Point, DisplayPoint) {
        loop {
            let mut inlay_point = self.inlay_snapshot.to_inlay_point(point);
            let mut fold_point = self.fold_snapshot.to_fold_point(inlay_point, Bias::Left);
            fold_point.0.column = 0;
            inlay_point = fold_point.to_inlay_point(&self.fold_snapshot);
            point = self.inlay_snapshot.to_buffer_point(inlay_point);

            let mut display_point = self.point_to_display_point(point, Bias::Left);
            *display_point.column_mut() = 0;
            let next_point = self.display_point_to_point(display_point, Bias::Left);
            if next_point == point {
                return (point, display_point);
            }
            point = next_point;
        }
    }

    pub fn next_line_boundary(&self, mut point: MultiBufferPoint) -> (Point, DisplayPoint) {
        loop {
            let mut inlay_point = self.inlay_snapshot.to_inlay_point(point);
            let mut fold_point = self.fold_snapshot.to_fold_point(inlay_point, Bias::Right);
            fold_point.0.column = self.fold_snapshot.line_len(fold_point.row());
            inlay_point = fold_point.to_inlay_point(&self.fold_snapshot);
            point = self.inlay_snapshot.to_buffer_point(inlay_point);

            let mut display_point = self.point_to_display_point(point, Bias::Right);
            *display_point.column_mut() = self.line_len(display_point.row());
            let next_point = self.display_point_to_point(display_point, Bias::Right);
            if next_point == point {
                return (point, display_point);
            }
            point = next_point;
        }
    }

    // used by line_mode selections and tries to match vim behaviour
    pub fn expand_to_line(&self, range: Range<Point>) -> Range<Point> {
        let new_start = if range.start.row == 0 {
            MultiBufferPoint::new(0, 0)
        } else if range.start.row == self.max_buffer_row().0
            || (range.end.column > 0 && range.end.row == self.max_buffer_row().0)
        {
            MultiBufferPoint::new(
                range.start.row - 1,
                self.buffer_snapshot
                    .line_len(MultiBufferRow(range.start.row - 1)),
            )
        } else {
            self.prev_line_boundary(range.start).0
        };

        let new_end = if range.end.column == 0 {
            range.end
        } else if range.end.row < self.max_buffer_row().0 {
            self.buffer_snapshot
                .clip_point(MultiBufferPoint::new(range.end.row + 1, 0), Bias::Left)
        } else {
            self.buffer_snapshot.max_point()
        };

        new_start..new_end
    }

    fn point_to_display_point(&self, point: MultiBufferPoint, bias: Bias) -> DisplayPoint {
        let inlay_point = self.inlay_snapshot.to_inlay_point(point);
        let fold_point = self.fold_snapshot.to_fold_point(inlay_point, bias);
        let tab_point = self.tab_snapshot.to_tab_point(fold_point);
        let wrap_point = self.wrap_snapshot.tab_point_to_wrap_point(tab_point);
        let block_point = self.block_snapshot.to_block_point(wrap_point);
        DisplayPoint(block_point)
    }

    fn display_point_to_point(&self, point: DisplayPoint, bias: Bias) -> Point {
        self.inlay_snapshot
            .to_buffer_point(self.display_point_to_inlay_point(point, bias))
    }

    pub fn display_point_to_inlay_offset(&self, point: DisplayPoint, bias: Bias) -> InlayOffset {
        self.inlay_snapshot
            .to_offset(self.display_point_to_inlay_point(point, bias))
    }

    pub fn anchor_to_inlay_offset(&self, anchor: Anchor) -> InlayOffset {
        self.inlay_snapshot
            .to_inlay_offset(anchor.to_offset(&self.buffer_snapshot))
    }

    pub fn display_point_to_anchor(&self, point: DisplayPoint, bias: Bias) -> Anchor {
        self.buffer_snapshot
            .anchor_at(point.to_offset(&self, bias), bias)
    }

    fn display_point_to_inlay_point(&self, point: DisplayPoint, bias: Bias) -> InlayPoint {
        let block_point = point.0;
        let wrap_point = self.block_snapshot.to_wrap_point(block_point);
        let tab_point = self.wrap_snapshot.to_tab_point(wrap_point);
        let fold_point = self.tab_snapshot.to_fold_point(tab_point, bias).0;
        fold_point.to_inlay_point(&self.fold_snapshot)
    }

    pub fn display_point_to_fold_point(&self, point: DisplayPoint, bias: Bias) -> FoldPoint {
        let block_point = point.0;
        let wrap_point = self.block_snapshot.to_wrap_point(block_point);
        let tab_point = self.wrap_snapshot.to_tab_point(wrap_point);
        self.tab_snapshot.to_fold_point(tab_point, bias).0
    }

    pub fn fold_point_to_display_point(&self, fold_point: FoldPoint) -> DisplayPoint {
        let tab_point = self.tab_snapshot.to_tab_point(fold_point);
        let wrap_point = self.wrap_snapshot.tab_point_to_wrap_point(tab_point);
        let block_point = self.block_snapshot.to_block_point(wrap_point);
        DisplayPoint(block_point)
    }

    pub fn max_point(&self) -> DisplayPoint {
        DisplayPoint(self.block_snapshot.max_point())
    }

    /// Returns text chunks starting at the given display row until the end of the file
    pub fn text_chunks(&self, display_row: DisplayRow) -> impl Iterator<Item = &str> {
        self.block_snapshot
            .chunks(
                display_row.0..self.max_point().row().next_row().0,
                false,
                Highlights::default(),
            )
            .map(|h| h.text)
    }

    /// Returns text chunks starting at the end of the given display row in reverse until the start of the file
    pub fn reverse_text_chunks(&self, display_row: DisplayRow) -> impl Iterator<Item = &str> {
        (0..=display_row.0).rev().flat_map(|row| {
            self.block_snapshot
                .chunks(row..row + 1, false, Highlights::default())
                .map(|h| h.text)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
        })
    }

    pub fn chunks(
        &self,
        display_rows: Range<DisplayRow>,
        language_aware: bool,
        highlight_styles: HighlightStyles,
    ) -> DisplayChunks<'_> {
        self.block_snapshot.chunks(
            display_rows.start.0..display_rows.end.0,
            language_aware,
            Highlights {
                text_highlights: Some(&self.text_highlights),
                inlay_highlights: Some(&self.inlay_highlights),
                styles: highlight_styles,
            },
        )
    }

    pub fn highlighted_chunks<'a>(
        &'a self,
        display_rows: Range<DisplayRow>,
        language_aware: bool,
        editor_style: &'a EditorStyle,
    ) -> impl Iterator<Item = HighlightedChunk<'a>> {
        self.chunks(
            display_rows,
            language_aware,
            HighlightStyles {
                inlay_hint: Some(editor_style.inlay_hints_style),
                suggestion: Some(editor_style.suggestions_style),
            },
        )
        .map(|chunk| {
            let mut highlight_style = chunk
                .syntax_highlight_id
                .and_then(|id| id.style(&editor_style.syntax));

            if let Some(chunk_highlight) = chunk.highlight_style {
                if let Some(highlight_style) = highlight_style.as_mut() {
                    highlight_style.highlight(chunk_highlight);
                } else {
                    highlight_style = Some(chunk_highlight);
                }
            }

            let mut diagnostic_highlight = HighlightStyle::default();

            if chunk.is_unnecessary {
                diagnostic_highlight.fade_out = Some(UNNECESSARY_CODE_FADE);
            }

            if let Some(severity) = chunk.diagnostic_severity {
                // Omit underlines for HINT/INFO diagnostics on 'unnecessary' code.
                if severity <= DiagnosticSeverity::WARNING || !chunk.is_unnecessary {
                    let diagnostic_color =
                        super::diagnostic_style(severity, true, &editor_style.status);
                    diagnostic_highlight.underline = Some(UnderlineStyle {
                        color: Some(diagnostic_color),
                        thickness: 1.0.into(),
                        wavy: true,
                    });
                }
            }

            if let Some(highlight_style) = highlight_style.as_mut() {
                highlight_style.highlight(diagnostic_highlight);
            } else {
                highlight_style = Some(diagnostic_highlight);
            }

            HighlightedChunk {
                text: chunk.text,
                style: highlight_style,
                is_tab: chunk.is_tab,
                renderer: chunk.renderer,
            }
        })
    }

    pub fn layout_row(
        &self,
        display_row: DisplayRow,
        TextLayoutDetails {
            text_system,
            editor_style,
            rem_size,
            scroll_anchor: _,
            visible_rows: _,
            vertical_scroll_margin: _,
        }: &TextLayoutDetails,
    ) -> Arc<LineLayout> {
        let mut runs = Vec::new();
        let mut line = String::new();

        let range = display_row..display_row.next_row();
        for chunk in self.highlighted_chunks(range, false, &editor_style) {
            line.push_str(chunk.text);

            let text_style = if let Some(style) = chunk.style {
                Cow::Owned(editor_style.text.clone().highlight(style))
            } else {
                Cow::Borrowed(&editor_style.text)
            };

            runs.push(text_style.to_run(chunk.text.len()))
        }

        if line.ends_with('\n') {
            line.pop();
            if let Some(last_run) = runs.last_mut() {
                last_run.len -= 1;
                if last_run.len == 0 {
                    runs.pop();
                }
            }
        }

        let font_size = editor_style.text.font_size.to_pixels(*rem_size);
        text_system
            .layout_line(&line, font_size, &runs)
            .expect("we expect the font to be loaded because it's rendered by the editor")
    }

    pub fn x_for_display_point(
        &self,
        display_point: DisplayPoint,
        text_layout_details: &TextLayoutDetails,
    ) -> Pixels {
        let line = self.layout_row(display_point.row(), text_layout_details);
        line.x_for_index(display_point.column() as usize)
    }

    pub fn display_column_for_x(
        &self,
        display_row: DisplayRow,
        x: Pixels,
        details: &TextLayoutDetails,
    ) -> u32 {
        let layout_line = self.layout_row(display_row, details);
        layout_line.closest_index_for_x(x) as u32
    }

    pub fn display_chars_at(
        &self,
        mut point: DisplayPoint,
    ) -> impl Iterator<Item = (char, DisplayPoint)> + '_ {
        point = DisplayPoint(self.block_snapshot.clip_point(point.0, Bias::Left));
        self.text_chunks(point.row())
            .flat_map(str::chars)
            .skip_while({
                let mut column = 0;
                move |char| {
                    let at_point = column >= point.column();
                    column += char.len_utf8() as u32;
                    !at_point
                }
            })
            .map(move |ch| {
                let result = (ch, point);
                if ch == '\n' {
                    *point.row_mut() += 1;
                    *point.column_mut() = 0;
                } else {
                    *point.column_mut() += ch.len_utf8() as u32;
                }
                result
            })
    }

    pub fn buffer_chars_at(&self, mut offset: usize) -> impl Iterator<Item = (char, usize)> + '_ {
        self.buffer_snapshot.chars_at(offset).map(move |ch| {
            let ret = (ch, offset);
            offset += ch.len_utf8();
            ret
        })
    }

    pub fn reverse_buffer_chars_at(
        &self,
        mut offset: usize,
    ) -> impl Iterator<Item = (char, usize)> + '_ {
        self.buffer_snapshot
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

    pub fn clip_at_line_end(&self, point: DisplayPoint) -> DisplayPoint {
        let mut point = point.0;
        if point.column == self.line_len(DisplayRow(point.row)) {
            point.column = point.column.saturating_sub(1);
            point = self.block_snapshot.clip_point(point, Bias::Left);
        }
        DisplayPoint(point)
    }

    pub fn folds_in_range<T>(&self, range: Range<T>) -> impl Iterator<Item = &Fold>
    where
        T: ToOffset,
    {
        self.fold_snapshot.folds_in_range(range)
    }

    pub fn blocks_in_range(
        &self,
        rows: Range<DisplayRow>,
    ) -> impl Iterator<Item = (DisplayRow, &TransformBlock)> {
        self.block_snapshot
            .blocks_in_range(rows.start.0..rows.end.0)
            .map(|(row, block)| (DisplayRow(row), block))
    }

    pub fn intersects_fold<T: ToOffset>(&self, offset: T) -> bool {
        self.fold_snapshot.intersects_fold(offset)
    }

    pub fn is_line_folded(&self, buffer_row: MultiBufferRow) -> bool {
        self.fold_snapshot.is_line_folded(buffer_row)
    }

    pub fn is_block_line(&self, display_row: DisplayRow) -> bool {
        self.block_snapshot.is_block_line(BlockRow(display_row.0))
    }

    pub fn soft_wrap_indent(&self, display_row: DisplayRow) -> Option<u32> {
        let wrap_row = self
            .block_snapshot
            .to_wrap_point(BlockPoint::new(display_row.0, 0))
            .row();
        self.wrap_snapshot.soft_wrap_indent(wrap_row)
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
        let (buffer, range) = self
            .buffer_snapshot
            .buffer_line_for_row(buffer_row)
            .unwrap();

        buffer.line_indent_for_row(range.start.row)
    }

    pub fn line_len(&self, row: DisplayRow) -> u32 {
        self.block_snapshot.line_len(BlockRow(row.0))
    }

    pub fn longest_row(&self) -> DisplayRow {
        DisplayRow(self.block_snapshot.longest_row())
    }

    pub fn starts_indent(&self, buffer_row: MultiBufferRow) -> bool {
        let max_row = self.buffer_snapshot.max_buffer_row();
        if buffer_row >= max_row {
            return false;
        }

        let line_indent = self.line_indent_for_buffer_row(buffer_row);
        if line_indent.is_line_blank() {
            return false;
        }

        for next_row in (buffer_row.0 + 1)..=max_row.0 {
            let next_line_indent = self.line_indent_for_buffer_row(MultiBufferRow(next_row));
            if next_line_indent.raw_len() > line_indent.raw_len() {
                return true;
            } else if !next_line_indent.is_line_blank() {
                break;
            }
        }

        false
    }

    pub fn foldable_range(
        &self,
        buffer_row: MultiBufferRow,
    ) -> Option<(Range<Point>, FoldPlaceholder)> {
        let start = MultiBufferPoint::new(buffer_row.0, self.buffer_snapshot.line_len(buffer_row));
        if let Some(flap) = self
            .flap_snapshot
            .query_row(buffer_row, &self.buffer_snapshot)
        {
            Some((
                flap.range.to_point(&self.buffer_snapshot),
                flap.placeholder.clone(),
            ))
        } else if self.starts_indent(MultiBufferRow(start.row))
            && !self.is_line_folded(MultiBufferRow(start.row))
        {
            let start_line_indent = self.line_indent_for_buffer_row(buffer_row);
            let max_point = self.buffer_snapshot.max_point();
            let mut end = None;

            for row in (buffer_row.0 + 1)..=max_point.row {
                let line_indent = self.line_indent_for_buffer_row(MultiBufferRow(row));
                if !line_indent.is_line_blank()
                    && line_indent.raw_len() <= start_line_indent.raw_len()
                {
                    let prev_row = row - 1;
                    end = Some(Point::new(
                        prev_row,
                        self.buffer_snapshot.line_len(MultiBufferRow(prev_row)),
                    ));
                    break;
                }
            }
            let end = end.unwrap_or(max_point);
            Some((start..end, self.fold_placeholder.clone()))
        } else {
            None
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn text_highlight_ranges<Tag: ?Sized + 'static>(
        &self,
    ) -> Option<Arc<(HighlightStyle, Vec<Range<Anchor>>)>> {
        let type_id = TypeId::of::<Tag>();
        self.text_highlights.get(&Some(type_id)).cloned()
    }

    #[allow(unused)]
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn inlay_highlights<Tag: ?Sized + 'static>(
        &self,
    ) -> Option<&TreeMap<InlayId, (HighlightStyle, InlayHighlight)>> {
        let type_id = TypeId::of::<Tag>();
        self.inlay_highlights.get(&type_id)
    }
}

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

#[derive(Debug, Copy, Clone, Default, Eq, Ord, PartialOrd, PartialEq, Deserialize, Hash)]
#[serde(transparent)]
pub struct DisplayRow(pub u32);

impl Add for DisplayRow {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        DisplayRow(self.0 + other.0)
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

    pub fn to_offset(self, map: &DisplaySnapshot, bias: Bias) -> usize {
        let wrap_point = map.block_snapshot.to_wrap_point(self.0);
        let tab_point = map.wrap_snapshot.to_tab_point(wrap_point);
        let fold_point = map.tab_snapshot.to_fold_point(tab_point, bias).0;
        let inlay_point = fold_point.to_inlay_point(&map.fold_snapshot);
        map.inlay_snapshot
            .to_buffer_offset(map.inlay_snapshot.to_offset(inlay_point))
    }
}

impl ToDisplayPoint for usize {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        map.point_to_display_point(self.to_point(&map.buffer_snapshot), Bias::Left)
    }
}

impl ToDisplayPoint for OffsetUtf16 {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        self.to_offset(&map.buffer_snapshot).to_display_point(map)
    }
}

impl ToDisplayPoint for Point {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        map.point_to_display_point(*self, Bias::Left)
    }
}

impl ToDisplayPoint for Anchor {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        self.to_point(&map.buffer_snapshot).to_display_point(map)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        movement,
        test::{editor_test_context::EditorTestContext, marked_display_snapshot},
    };
    use gpui::{div, font, observe, px, AppContext, BorrowAppContext, Context, Element, Hsla};
    use language::{
        language_settings::{AllLanguageSettings, AllLanguageSettingsContent},
        Buffer, Language, LanguageConfig, LanguageMatcher, SelectionGoal,
    };
    use project::Project;
    use rand::{prelude::*, Rng};
    use settings::SettingsStore;
    use smol::stream::StreamExt;
    use std::{env, sync::Arc};
    use theme::{LoadThemes, SyntaxTheme};
    use util::test::{marked_text_ranges, sample_text};
    use Bias::*;

    #[gpui::test(iterations = 100)]
    async fn test_random_display_map(cx: &mut gpui::TestAppContext, mut rng: StdRng) {
        cx.background_executor.set_block_on_ticks(0..=50);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut tab_size = rng.gen_range(1..=4);
        let buffer_start_excerpt_header_height = rng.gen_range(1..=5);
        let excerpt_header_height = rng.gen_range(1..=5);
        let font_size = px(14.0);
        let max_wrap_width = 300.0;
        let mut wrap_width = if rng.gen_bool(0.1) {
            None
        } else {
            Some(px(rng.gen_range(0.0..=max_wrap_width)))
        };

        log::info!("tab size: {}", tab_size);
        log::info!("wrap width: {:?}", wrap_width);

        cx.update(|cx| {
            init_test(cx, |s| s.defaults.tab_size = NonZeroU32::new(tab_size));
        });

        let buffer = cx.update(|cx| {
            if rng.gen() {
                let len = rng.gen_range(0..10);
                let text = util::RandomCharIter::new(&mut rng)
                    .take(len)
                    .collect::<String>();
                MultiBuffer::build_simple(&text, cx)
            } else {
                MultiBuffer::build_random(&mut rng, cx)
            }
        });

        let map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                wrap_width,
                true,
                buffer_start_excerpt_header_height,
                excerpt_header_height,
                0,
                FoldPlaceholder::test(),
                cx,
            )
        });
        let mut notifications = observe(&map, cx);
        let mut fold_count = 0;
        let mut blocks = Vec::new();

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        log::info!("buffer text: {:?}", snapshot.buffer_snapshot.text());
        log::info!("fold text: {:?}", snapshot.fold_snapshot.text());
        log::info!("tab text: {:?}", snapshot.tab_snapshot.text());
        log::info!("wrap text: {:?}", snapshot.wrap_snapshot.text());
        log::info!("block text: {:?}", snapshot.block_snapshot.text());
        log::info!("display text: {:?}", snapshot.text());

        for _i in 0..operations {
            match rng.gen_range(0..100) {
                0..=19 => {
                    wrap_width = if rng.gen_bool(0.2) {
                        None
                    } else {
                        Some(px(rng.gen_range(0.0..=max_wrap_width)))
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
                            store.update_user_settings::<AllLanguageSettings>(cx, |s| {
                                s.defaults.tab_size = NonZeroU32::new(tab_size);
                            });
                        });
                    });
                }
                30..=44 => {
                    map.update(cx, |map, cx| {
                        if rng.gen() || blocks.is_empty() {
                            let buffer = map.snapshot(cx).buffer_snapshot;
                            let block_properties = (0..rng.gen_range(1..=1))
                                .map(|_| {
                                    let position =
                                        buffer.anchor_after(buffer.clip_offset(
                                            rng.gen_range(0..=buffer.len()),
                                            Bias::Left,
                                        ));

                                    let disposition = if rng.gen() {
                                        BlockDisposition::Above
                                    } else {
                                        BlockDisposition::Below
                                    };
                                    let height = rng.gen_range(1..5);
                                    log::info!(
                                        "inserting block {:?} {:?} with height {}",
                                        disposition,
                                        position.to_point(&buffer),
                                        height
                                    );
                                    BlockProperties {
                                        style: BlockStyle::Fixed,
                                        position,
                                        height,
                                        disposition,
                                        render: Box::new(|_| div().into_any()),
                                    }
                                })
                                .collect::<Vec<_>>();
                            blocks.extend(map.insert_blocks(block_properties, cx));
                        } else {
                            blocks.shuffle(&mut rng);
                            let remove_count = rng.gen_range(1..=4.min(blocks.len()));
                            let block_ids_to_remove = (0..remove_count)
                                .map(|_| blocks.remove(rng.gen_range(0..blocks.len())))
                                .collect();
                            log::info!("removing block ids {:?}", block_ids_to_remove);
                            map.remove_blocks(block_ids_to_remove, cx);
                        }
                    });
                }
                45..=79 => {
                    let mut ranges = Vec::new();
                    for _ in 0..rng.gen_range(1..=3) {
                        buffer.read_with(cx, |buffer, cx| {
                            let buffer = buffer.read(cx);
                            let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                            let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                            ranges.push(start..end);
                        });
                    }

                    if rng.gen() && fold_count > 0 {
                        log::info!("unfolding ranges: {:?}", ranges);
                        map.update(cx, |map, cx| {
                            map.unfold(ranges, true, cx);
                        });
                    } else {
                        log::info!("folding ranges: {:?}", ranges);
                        map.update(cx, |map, cx| {
                            map.fold(
                                ranges
                                    .into_iter()
                                    .map(|range| (range, FoldPlaceholder::test())),
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
            log::info!("buffer text: {:?}", snapshot.buffer_snapshot.text());
            log::info!("fold text: {:?}", snapshot.fold_snapshot.text());
            log::info!("tab text: {:?}", snapshot.tab_snapshot.text());
            log::info!("wrap text: {:?}", snapshot.wrap_snapshot.text());
            log::info!("block text: {:?}", snapshot.block_snapshot.text());
            log::info!("display text: {:?}", snapshot.text());

            // Line boundaries
            let buffer = &snapshot.buffer_snapshot;
            for _ in 0..5 {
                let row = rng.gen_range(0..=buffer.max_point().row);
                let column = rng.gen_range(0..=buffer.line_len(MultiBufferRow(row)));
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
                let row = rng.gen_range(0..=snapshot.max_point().row().0);
                let column = rng.gen_range(0..=snapshot.line_len(DisplayRow(row)));
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
            init_test(cx, |_| {});
        });

        let mut cx = EditorTestContext::new(cx).await;
        let editor = cx.editor.clone();
        let window = cx.window;

        _ = cx.update_window(window, |_, cx| {
            let text_layout_details =
                editor.update(cx, |editor, cx| editor.text_layout_details(cx));

            let font_size = px(12.0);
            let wrap_width = Some(px(64.));

            let text = "one two three four five\nsix seven eight";
            let buffer = MultiBuffer::build_simple(text, cx);
            let map = cx.new_model(|cx| {
                DisplayMap::new(
                    buffer.clone(),
                    font("Helvetica"),
                    font_size,
                    wrap_width,
                    true,
                    1,
                    1,
                    0,
                    FoldPlaceholder::test(),
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
                    SelectionGoal::None,
                    false,
                    &text_layout_details,
                ),
                (
                    DisplayPoint::new(DisplayRow(0), 7),
                    SelectionGoal::HorizontalPosition(x.0)
                )
            );
            assert_eq!(
                movement::down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(0), 7),
                    SelectionGoal::HorizontalPosition(x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(1), 10),
                    SelectionGoal::HorizontalPosition(x.0)
                )
            );
            assert_eq!(
                movement::down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(1), 10),
                    SelectionGoal::HorizontalPosition(x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(2), 4),
                    SelectionGoal::HorizontalPosition(x.0)
                )
            );

            let ix = snapshot.buffer_snapshot.text().find("seven").unwrap();
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
                map.set_font(font("Helvetica"), px(font_size.0 + 3.), cx)
            });

            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                snapshot.text_chunks(DisplayRow(1)).collect::<String>(),
                "three \nfour five\nsix and \nseven \neight"
            )
        });
    }

    #[gpui::test]
    fn test_text_chunks(cx: &mut gpui::AppContext) {
        init_test(cx, |_| {});

        let text = sample_text(6, 6, 'a');
        let buffer = MultiBuffer::build_simple(&text, cx);

        let font_size = px(14.0);
        let map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                true,
                1,
                1,
                0,
                FoldPlaceholder::test(),
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
    async fn test_chunks(cx: &mut gpui::TestAppContext) {
        use unindent::Unindent as _;

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
                    matcher: LanguageMatcher {
                        path_suffixes: vec![".test".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
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

        cx.update(|cx| init_test(cx, |s| s.defaults.tab_size = Some(2.try_into().unwrap())));

        let buffer = cx.new_model(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.condition(&buffer, |buf, _| !buf.is_parsing()).await;
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));

        let font_size = px(14.0);

        let map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer,
                font("Helvetica"),
                font_size,
                None,
                true,
                1,
                1,
                1,
                FoldPlaceholder::test(),
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
                vec![(
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
    async fn test_chunks_with_soft_wrapping(cx: &mut gpui::TestAppContext) {
        use unindent::Unindent as _;

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
                    matcher: LanguageMatcher {
                        path_suffixes: vec![".test".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
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

        cx.update(|cx| init_test(cx, |_| {}));

        let buffer = cx.new_model(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.condition(&buffer, |buf, _| !buf.is_parsing()).await;
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));

        let font_size = px(16.0);

        let map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                font_size,
                Some(px(40.0)),
                true,
                1,
                1,
                0,
                FoldPlaceholder::test(),
                cx,
            )
        });
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(0)..DisplayRow(5), &map, &theme, cx)),
            [
                ("fn \n".to_string(), None),
                ("oute\nr".to_string(), Some(Hsla::blue())),
                ("() \n{}\n\n".to_string(), None),
            ]
        );
        assert_eq!(
            cx.update(|cx| syntax_chunks(DisplayRow(3)..DisplayRow(5), &map, &theme, cx)),
            [("{}\n\n".to_string(), None)]
        );

        map.update(cx, |map, cx| {
            map.fold(
                vec![(
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
                ("  \nfn ".to_string(), Some(Hsla::red())),
                ("i\n".to_string(), Some(Hsla::blue()))
            ]
        );
    }

    #[gpui::test]
    async fn test_chunks_with_text_highlights(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx, |_| {}));

        let theme =
            SyntaxTheme::new_test(vec![("operator", Hsla::red()), ("string", Hsla::green())]);
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Test".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec![".test".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
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

        let (text, highlighted_ranges) = marked_text_ranges(r#"constˇ «a»: B = "c «d»""#, false);

        let buffer = cx.new_model(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.condition(&buffer, |buf, _| !buf.is_parsing()).await;

        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));

        let font_size = px(16.0);
        let map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer,
                font("Courier"),
                font_size,
                None,
                true,
                1,
                1,
                1,
                FoldPlaceholder::test(),
                cx,
            )
        });

        enum MyType {}

        let style = HighlightStyle {
            color: Some(Hsla::blue()),
            ..Default::default()
        };

        map.update(cx, |map, _cx| {
            map.highlight_text(
                TypeId::of::<MyType>(),
                highlighted_ranges
                    .into_iter()
                    .map(|range| {
                        buffer_snapshot.anchor_before(range.start)
                            ..buffer_snapshot.anchor_before(range.end)
                    })
                    .collect(),
                style,
            );
        });

        assert_eq!(
            cx.update(|cx| chunks(DisplayRow(0)..DisplayRow(10), &map, &theme, cx)),
            [
                ("const ".to_string(), None, None),
                ("a".to_string(), None, Some(Hsla::blue())),
                (":".to_string(), Some(Hsla::red()), None),
                (" B = ".to_string(), None, None),
                ("\"c ".to_string(), Some(Hsla::green()), None),
                ("d".to_string(), Some(Hsla::green()), Some(Hsla::blue())),
                ("\"".to_string(), Some(Hsla::green()), None),
            ]
        );
    }

    #[gpui::test]
    fn test_clip_point(cx: &mut gpui::AppContext) {
        init_test(cx, |_| {});

        fn assert(text: &str, shift_right: bool, bias: Bias, cx: &mut gpui::AppContext) {
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
    fn test_clip_at_line_ends(cx: &mut gpui::AppContext) {
        init_test(cx, |_| {});

        fn assert(text: &str, cx: &mut gpui::AppContext) {
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
    fn test_flaps(cx: &mut gpui::AppContext) {
        init_test(cx, |_| {});

        let text = "aaa\nbbb\nccc\nddd\neee\nfff\nggg\nhhh\niii\njjj\nkkk\nlll";
        let buffer = MultiBuffer::build_simple(text, cx);
        let font_size = px(14.0);
        cx.new_model(|cx| {
            let mut map = DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                true,
                1,
                1,
                0,
                FoldPlaceholder::test(),
                cx,
            );
            let snapshot = map.buffer.read(cx).snapshot(cx);
            let range =
                snapshot.anchor_before(Point::new(2, 0))..snapshot.anchor_after(Point::new(3, 3));

            map.flap_map.insert(
                [Flap::new(
                    range,
                    FoldPlaceholder::test(),
                    |_row, _status, _toggle, _cx| div(),
                    |_row, _status, _cx| div(),
                )],
                &map.buffer.read(cx).snapshot(cx),
            );

            map
        });
    }

    #[gpui::test]
    fn test_tabs_with_multibyte_chars(cx: &mut gpui::AppContext) {
        init_test(cx, |_| {});

        let text = "✅\t\tα\nβ\t\n🏀β\t\tγ";
        let buffer = MultiBuffer::build_simple(text, cx);
        let font_size = px(14.0);

        let map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                true,
                1,
                1,
                0,
                FoldPlaceholder::test(),
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
    fn test_max_point(cx: &mut gpui::AppContext) {
        init_test(cx, |_| {});

        let buffer = MultiBuffer::build_simple("aaa\n\t\tbbb", cx);
        let font_size = px(14.0);
        let map = cx.new_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                None,
                true,
                1,
                1,
                0,
                FoldPlaceholder::test(),
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
        map: &Model<DisplayMap>,
        theme: &SyntaxTheme,
        cx: &mut AppContext,
    ) -> Vec<(String, Option<Hsla>)> {
        chunks(rows, map, theme, cx)
            .into_iter()
            .map(|(text, color, _)| (text, color))
            .collect()
    }

    fn chunks(
        rows: Range<DisplayRow>,
        map: &Model<DisplayMap>,
        theme: &SyntaxTheme,
        cx: &mut AppContext,
    ) -> Vec<(String, Option<Hsla>, Option<Hsla>)> {
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut chunks: Vec<(String, Option<Hsla>, Option<Hsla>)> = Vec::new();
        for chunk in snapshot.chunks(rows, true, HighlightStyles::default()) {
            let syntax_color = chunk
                .syntax_highlight_id
                .and_then(|id| id.style(theme)?.color);
            let highlight_color = chunk.highlight_style.and_then(|style| style.color);
            if let Some((last_chunk, last_syntax_color, last_highlight_color)) = chunks.last_mut() {
                if syntax_color == *last_syntax_color && highlight_color == *last_highlight_color {
                    last_chunk.push_str(chunk.text);
                    continue;
                }
            }
            chunks.push((chunk.text.to_string(), syntax_color, highlight_color));
        }
        chunks
    }

    fn init_test(cx: &mut AppContext, f: impl Fn(&mut AllLanguageSettingsContent)) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
        theme::init(LoadThemes::JustBase, cx);
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, f);
        });
    }
}
