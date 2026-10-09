use std::{fmt::Debug, ops::Range, sync::Arc};

use collections::HashMap;
use gpui::{Font, HighlightStyle, LineLayout, Pixels, TextAlign, WindowTextSystem};
use multi_buffer::{Anchor, MultiBufferOffset, MultiBufferRow, MultiBufferSnapshot, ToOffset};
use parking_lot::Mutex;
use project::InlayId;
use sum_tree::{Bias, TreeMap};
use util::debug_panic;

use crate::{
    EditorStyle, MAX_LINE_LEN,
    hover_links::InlayHighlight,
    scroll::{ScrollOffset, ScrollPixelOffset},
};

use super::{
    DisplayMap, DisplayPoint, DisplayRow, DisplaySnapshot, FoldPoint, HighlightKey,
    HighlightStyleInterner, WrapPoint, WrapRow,
    block_map::BlockSnapshot,
    inlay_map::has_chunk_renderer,
    row_ruler::{RenderPiece, RowRuler, RowRulerCache, RulerCacheVersion, RulerShaper},
};

const MAX_TRACKED_RULER_DIRT: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct GridCell {
    pub(crate) width: Pixels,
    pub(crate) monospace: bool,
}

impl GridCell {
    const FIT_TOLERANCE: ScrollPixelOffset = 0.5;

    pub(crate) fn measure(
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

    pub(crate) fn fits(&self, columns: &Range<u32>, shaped_width: Pixels) -> bool {
        let grid_width = ScrollPixelOffset::from(self.width) * columns.len() as ScrollPixelOffset;
        (ScrollPixelOffset::from(shaped_width) - grid_width).abs() <= Self::FIT_TOLERANCE
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HorizontalViewport {
    pub(crate) scroll_columns: ScrollOffset,
    pub(crate) visible_columns: ScrollOffset,
    pub(crate) text_align: TextAlign,
    pub(crate) content_width: Pixels,
}

impl HorizontalViewport {
    pub(crate) fn aligned(&self, line_width: ScrollPixelOffset, cell: GridCell) -> Self {
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

    pub(crate) fn first_column(&self) -> usize {
        self.scroll_columns.max(0.).floor() as usize
    }

    pub(crate) fn column_count(&self) -> usize {
        (self.visible_columns.max(0.).ceil() as usize).max(1)
    }

    pub(crate) fn shaping_window(&self, row_len: u32) -> Range<u32> {
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
pub(crate) struct RuledRow {
    snapshot: Arc<DisplaySnapshot>,
    row: DisplayRow,
    pub(super) ruler: Arc<RowRuler>,
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
    pub(crate) fn columns_for_viewport(
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

    pub(crate) fn render_pieces(
        &self,
        columns: Range<u32>,
    ) -> impl Iterator<Item = RenderPiece> + '_ {
        self.ruler.render_pieces(columns)
    }

    pub(crate) fn x_range_for_columns(&self, columns: Range<u32>) -> Range<ScrollPixelOffset> {
        self.ruler.x_range_for_columns(columns)
    }

    pub(crate) fn is_rtl(&self) -> bool {
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
    pub(crate) fn new(row_len: u32, cell: GridCell, window: Range<u32>) -> Self {
        Self {
            row_len,
            cell,
            window,
            ruled: None,
        }
    }

    pub(crate) fn ruled(ruled: RuledRow, row_len: u32, cell: GridCell, window: Range<u32>) -> Self {
        Self {
            row_len,
            cell,
            window,
            ruled: Some(ruled),
        }
    }

    pub(crate) fn is_ruled(&self) -> bool {
        self.ruled.is_some()
    }

    pub(crate) fn row_len(&self) -> u32 {
        self.row_len
    }

    pub(crate) fn window(&self) -> &Range<u32> {
        &self.window
    }

    pub(crate) fn cell(&self) -> GridCell {
        self.cell
    }

    pub(crate) fn start_x(&self) -> ScrollPixelOffset {
        match &self.ruled {
            Some(ruled) => ruled.x_range_for_columns(self.window.clone()).start,
            None => self.column_x(self.window.start),
        }
    }

    pub(crate) fn end_x(&self) -> ScrollPixelOffset {
        match &self.ruled {
            Some(ruled) => ruled.x_range_for_columns(self.window.clone()).end,
            None => self.column_x(self.window.end),
        }
    }

    pub(crate) fn width(&self, shaped_width: Pixels) -> ScrollPixelOffset {
        let row_width = self.column_x(self.row_len);
        if self.ruled.is_some() {
            return row_width;
        }
        row_width.max(self.start_x() + ScrollPixelOffset::from(shaped_width))
    }

    pub(crate) fn column_x(&self, column: u32) -> ScrollPixelOffset {
        if let Some(ruled) = &self.ruled {
            return ruled.x_for_column(column.min(self.row_len));
        }
        ScrollPixelOffset::from(self.cell.width) * column as ScrollPixelOffset
    }

    pub(crate) fn column_for_x(&self, x: ScrollPixelOffset) -> u32 {
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

    pub(crate) fn reaches_left_edge(&self) -> bool {
        if self.is_rtl() {
            self.window.end >= self.row_len
        } else {
            self.window.start == 0
        }
    }

    pub(crate) fn reaches_right_edge(&self) -> bool {
        if self.is_rtl() {
            self.window.start == 0
        } else {
            self.window.end >= self.row_len
        }
    }

    pub(crate) fn column_left_of_window_for_x(&self, x: ScrollPixelOffset) -> u32 {
        if self.is_rtl() {
            self.column_for_x(x).max(self.window.end)
        } else {
            self.column_for_x(x).min(self.window.start)
        }
    }

    pub(crate) fn column_right_of_window_for_x(&self, x: ScrollPixelOffset) -> u32 {
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

    pub(crate) fn x_for_index(&self, index: usize) -> ScrollPixelOffset {
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

impl DisplaySnapshot {
    #[cfg(test)]
    pub(crate) fn is_windowed_row(&self, display_row: DisplayRow, cell: GridCell) -> bool {
        self.is_long_unwrapped_row(display_row) && self.row_has_exact_grid(display_row, cell)
    }

    pub(crate) fn is_long_unwrapped_row(&self, display_row: DisplayRow) -> bool {
        self.long_unwrapped_row_len(display_row).is_some()
    }

    pub(crate) fn long_unwrapped_row_len(&self, display_row: DisplayRow) -> Option<u32> {
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

    pub(crate) fn grid_window(
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

    pub(crate) fn ruled_row(&self, display_row: DisplayRow, shaper: RulerShaper) -> RuledRow {
        let wrap_row = self.wrap_row(display_row);
        let ruler = self
            .row_rulers
            .get_or_build(wrap_row, &shaper, |previous, renderer_widths| {
                RowRuler::new(self, display_row, &shaper, previous, renderer_widths)
            });
        RuledRow {
            snapshot: self
                .ruled_row_snapshot
                .get_or_init(|| Arc::new(self.clone()))
                .clone(),
            row: display_row,
            ruler,
            shaper,
        }
    }

    pub(crate) fn row_has_exact_grid(&self, display_row: DisplayRow, cell: GridCell) -> bool {
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
        fold_snapshot.folds_in_range(buffer_range).next().is_none()
            && !inlay_snapshot.has_inlays_matching(inlay_range, |inlay, text_range| {
                has_chunk_renderer(inlay)
                    || !inlay
                        .text()
                        .chunks_in_range(text_range)
                        .all(|chunk| all_grid_bytes(chunk.as_bytes()))
            })
            && !self.control_rows.contains_control(buffer, buffer_offsets)
    }

    fn wrap_row(&self, display_row: DisplayRow) -> u32 {
        self.block_snapshot
            .to_wrap_point(DisplayPoint::new(display_row, 0).0, Bias::Left)
            .row()
            .0
    }
}

#[derive(Default)]
pub(super) struct RulerDirt {
    all: bool,
    ranges: Vec<Range<Anchor>>,
}

#[derive(Default)]
pub(super) struct ControlRows(Mutex<HashMap<MultiBufferOffset, ControlRow>>);

struct ControlRow {
    range: Range<Anchor>,
    end: MultiBufferOffset,
    has_control: bool,
}

impl ControlRows {
    fn edited(&self, buffer: &MultiBufferSnapshot, dirty: &[Range<MultiBufferOffset>]) -> Self {
        let rows = self
            .0
            .lock()
            .values()
            .filter_map(|row| {
                let start = row.range.start.to_offset(buffer);
                let end = row.range.end.to_offset(buffer);
                let start_point = buffer.offset_to_point(start);
                if start_point.column != 0
                    || start + buffer.line_len(MultiBufferRow(start_point.row)) as usize != end
                {
                    return None;
                }
                let mut touched = dirty
                    .iter()
                    .filter(|dirty| dirty.start <= end && start <= dirty.end)
                    .peekable();
                if row.has_control && touched.peek().is_some() {
                    return None;
                }
                let still_clean = touched.all(|dirty| {
                    buffer
                        .bytes_in_range(dirty.start.max(start)..dirty.end.min(end))
                        .all(all_grid_bytes)
                });
                still_clean.then(|| {
                    (
                        start,
                        ControlRow {
                            range: row.range.clone(),
                            end,
                            has_control: row.has_control,
                        },
                    )
                })
            })
            .collect::<HashMap<_, _>>();
        Self(Mutex::new(rows))
    }

    fn contains_control(
        &self,
        buffer: &MultiBufferSnapshot,
        range: Range<MultiBufferOffset>,
    ) -> bool {
        if range.is_empty() {
            return false;
        }
        if let Some(row) = self.0.lock().get(&range.start)
            && row.end == range.end
        {
            return row.has_control;
        }
        let has_control = !buffer.bytes_in_range(range.clone()).all(all_grid_bytes);
        self.0.lock().insert(
            range.start,
            ControlRow {
                range: buffer.anchor_before(range.start)..buffer.anchor_after(range.end),
                end: range.end,
                has_control,
            },
        );
        has_control
    }
}

pub(super) fn affects_shaping(style: &HighlightStyle) -> bool {
    style.color.is_some()
        || style.font_weight.is_some()
        || style.font_style.is_some()
        || style.underline.is_some()
        || style.strikethrough.is_some()
        || style.fade_out.is_some()
}

impl DisplayMap {
    pub(super) fn refresh_row_caches(&mut self, block_snapshot: &BlockSnapshot) {
        let tab_snapshot = &block_snapshot.wrap_snapshot.tab_snapshot;
        let buffer = &tab_snapshot.fold_snapshot.inlay_snapshot.buffer;
        let version = RulerCacheVersion {
            tabs: tab_snapshot.version,
            highlights: self.highlight_version,
            non_text_state: buffer.non_text_state_update_count(),
            diagnostics_max_severity: self.diagnostics_max_severity,
            tab_size: tab_snapshot.tab_size,
            masked: self.masked,
        };
        let previous_version = self.row_rulers.version();
        if previous_version == version {
            return;
        }
        let dirt = std::mem::take(&mut self.ruler_dirt);
        let old_buffer = self.ruler_old_buffer.replace(buffer.clone());
        let dirty_offsets = dirt
            .ranges
            .iter()
            .map(|range| range.start.to_offset(buffer)..range.end.to_offset(buffer))
            .collect::<Vec<_>>();
        self.control_rows = if dirt.all {
            Arc::default()
        } else {
            Arc::new(self.control_rows.edited(buffer, &dirty_offsets))
        };
        let retain_any = !dirt.all
            && previous_version.diagnostics_max_severity == version.diagnostics_max_severity
            && previous_version.tab_size == version.tab_size
            && previous_version.masked == version.masked;
        let diagnostics_changed = previous_version.non_text_state != version.non_text_state;
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
            let old_range =
                row_range.start.to_offset(old_buffer)..row_range.end.to_offset(old_buffer);
            if diagnostics_changed
                && !same_diagnostics(old_buffer, old_range.clone(), buffer, start..end)
            {
                return None;
            }
            if ruler.uses_syntax() && !same_syntax(old_buffer, old_range, buffer, start..end) {
                return None;
            }
            let wrap_row = wrap_row_for_offset(block_snapshot, start);
            (row_buffer_range(block_snapshot, wrap_row) == (start..end)).then_some(wrap_row)
        };
        self.row_rulers = Arc::new(RowRulerCache::new(version, Some(&self.row_rulers), retain));
    }

    pub(super) fn mark_rulers_dirty(&mut self, ranges: impl IntoIterator<Item = Range<Anchor>>) {
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

    pub(super) fn mark_rulers_dirty_in<T: ToOffset>(
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

    pub(super) fn mark_all_rulers_dirty(&mut self) {
        self.ruler_dirt.all = true;
        self.ruler_dirt.ranges.clear();
    }

    pub(super) fn mark_text_highlight_dirty(
        &mut self,
        key: HighlightKey,
        ranges: &[Range<Anchor>],
        style: HighlightStyle,
        merge: bool,
        buffer: &MultiBufferSnapshot,
    ) {
        let previous = self.text_highlights.get(&key);
        if !affects_shaping(&style)
            && !previous.is_some_and(|previous| affects_shaping(&previous.0))
        {
            return;
        }
        let previous_hull = previous
            .filter(|previous| !merge || previous.0 != style)
            .and_then(|previous| anchor_hull(&previous.1, buffer));
        let dirty = previous_hull
            .into_iter()
            .chain(anchor_hull(ranges, buffer))
            .collect::<Vec<_>>();
        self.highlight_version += 1;
        self.mark_rulers_dirty(dirty);
    }

    pub(super) fn mark_inlay_highlight_dirty(
        &mut self,
        key: HighlightKey,
        highlights: &[InlayHighlight],
        style: HighlightStyle,
    ) {
        let previous_affects_shaping = self.inlay_highlights.get(&key).is_some_and(|highlights| {
            highlights
                .iter()
                .any(|(_, (style, _))| affects_shaping(style))
        });
        if !affects_shaping(&style) && !previous_affects_shaping {
            return;
        }
        let dirty = highlights
            .iter()
            .map(|highlight| highlight.inlay_position..highlight.inlay_position)
            .collect::<Vec<_>>();
        self.highlight_version += 1;
        self.mark_rulers_dirty(dirty);
    }

    pub(super) fn mark_removed_highlights_dirty(
        &mut self,
        text: Option<&(HighlightStyle, Vec<Range<Anchor>>)>,
        inlays: Option<&TreeMap<InlayId, (HighlightStyle, InlayHighlight)>>,
    ) {
        let dirty = text
            .filter(|(style, _)| affects_shaping(style))
            .into_iter()
            .flat_map(|(_, ranges)| ranges.iter().cloned())
            .chain(inlays.into_iter().flat_map(|inlays| {
                inlays
                    .iter()
                    .filter(|(_, (style, _))| affects_shaping(style))
                    .map(|(_, (_, highlight))| highlight.inlay_position..highlight.inlay_position)
            }))
            .collect::<Vec<_>>();
        if !dirty.is_empty() {
            self.highlight_version += 1;
            self.mark_rulers_dirty(dirty);
        }
    }

    pub(super) fn mark_semantic_styles_dirty(&mut self, interner: &HighlightStyleInterner) {
        if interner.styles().any(affects_shaping) {
            self.highlight_version += 1;
            self.mark_all_rulers_dirty();
        }
    }
}

fn is_grid_byte(byte: u8) -> bool {
    (byte >= 0x20 && byte != 0x7f) || byte == b'\t' || byte == b'\n'
}

fn all_grid_bytes(bytes: &[u8]) -> bool {
    let found_control = bytes.iter().fold(false, |found_control, byte| {
        found_control | !is_grid_byte(*byte)
    });
    !found_control
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

fn same_syntax(
    old_buffer: &MultiBufferSnapshot,
    old_range: Range<MultiBufferOffset>,
    buffer: &MultiBufferSnapshot,
    range: Range<MultiBufferOffset>,
) -> bool {
    let old_ranges = old_buffer.range_to_buffer_ranges(old_range);
    let new_ranges = buffer.range_to_buffer_ranges(range);
    let unstyled = |buffer: &language::BufferSnapshot,
                    range: &Range<multi_buffer::BufferOffset>| {
        buffer
            .syntax_layers_for_range(range.start.0..range.end.0, true)
            .next()
            .is_none()
    };
    old_ranges.len() == new_ranges.len()
        && old_ranges.iter().zip(&new_ranges).all(
            |((old_buffer, old_range, _), (new_buffer, new_range, _))| {
                old_buffer.remote_id() == new_buffer.remote_id()
                    && (old_buffer.syntax_update_count() == new_buffer.syntax_update_count()
                        || unstyled(old_buffer, old_range) && unstyled(new_buffer, new_range))
            },
        )
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

fn font_is_grid_exact(
    text_system: &WindowTextSystem,
    font: &Font,
    font_size: Pixels,
    cell_width: Pixels,
) -> bool {
    let font_id = text_system.resolve_font(font);
    text_system.ascii_shaping_preserves_advances(font)
        && (0x20u8..=0x7E)
            .all(|byte| text_system.layout_width(font_id, font_size, byte as char) == cell_width)
}

fn column_from_usize(column: usize) -> u32 {
    u32::try_from(column).unwrap_or(u32::MAX)
}
