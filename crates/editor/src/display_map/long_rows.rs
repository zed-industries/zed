#[cfg(test)]
use std::sync::atomic::AtomicUsize;
use std::{
    borrow::Cow,
    ops::Range,
    sync::{Arc, OnceLock, Weak, atomic},
};

use collections::{HashMap, HashSet};
use futures::{
    SinkExt,
    channel::{mpsc, oneshot},
};
use futures_lite::future::yield_now;

use gpui::{
    BackgroundExecutor, Bounds, CompactLineLayout, FontId, FontRun, GlyphId, GlyphRasterConfig,
    LineLayout, LineWrapper, LineWrapperHandle, Pixels, RetainedLineLayout, ScaledPixels,
    ShapedGlyph, ShapedRun, SharedString, TextAlign, TextSystem, point, px,
};
use itertools::Itertools as _;
use language::{LanguageAwareStyling, Point};
use multi_buffer::MultiBufferOffset;
use parking_lot::{Mutex, MutexGuard};
use smallvec::SmallVec;
use sum_tree::{Bias, TreeMap};
use text::ToOffset;
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use super::{
    ChunkRendererId, Highlights, LongRowCompletion, LongRowSource, LongRowWidthRequest,
    RendererConstraintJob, RowReadiness,
    invisibles::{is_invisible, replacement},
    tab_map::{TabChunks, TabEdit, TabPoint, TabSnapshot},
    wrap_map::WrapRow,
};
use crate::{
    MAX_LINE_LEN,
    element::{PreparedRendererConstraint, PreparedRowText},
    scroll::ScrollPixelOffset,
};

#[derive(Clone)]
pub struct LongRow {
    geometry: Arc<RowGeometry>,
    #[cfg(test)]
    widths: Arc<CharWidths>,
}

pub(crate) struct PaintTextPiece {
    pub(crate) columns: Range<u32>,
    pub(crate) x: ScrollPixelOffset,
    pub(crate) text: SharedString,
    pub(crate) layout: Arc<LineLayout>,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct RowMetrics {
    pub(crate) ascent: Pixels,
    pub(crate) descent: Pixels,
}

#[derive(Clone, Default)]
pub(crate) struct LongRows {
    widths: Option<Arc<CharWidths>>,
    rows: Arc<RowCache>,
    completed_max_width: Arc<OnceLock<ScrollPixelOffset>>,
    source: Option<Arc<LongRowSource>>,
    identity: Arc<()>,
    version: usize,
    raster_config: GlyphRasterConfig,
}

#[derive(Clone)]
pub(super) struct DetachedLongRows {
    cache: LongRows,
}

struct TextPiece {
    columns: Range<u32>,
    x: ScrollPixelOffset,
    text: SharedString,
    layout: Arc<LineLayout>,
    source_start_column: u32,
    index: LayoutIndex,
}

#[derive(Default)]
pub(super) struct RowCache {
    rows: Mutex<TreeMap<u32, CachedRow>>,
    snapshot: OnceLock<TabSnapshot>,
    drop_executor: OnceLock<BackgroundExecutor>,
}

struct CachedRow {
    generation: Weak<()>,
    width: ScrollPixelOffset,
    geometry: Weak<RowGeometry>,
}

impl Clone for CachedRow {
    fn clone(&self) -> Self {
        #[cfg(test)]
        if let Some(geometry) = self.geometry.upgrade() {
            geometry
                .cached_row_clones
                .fetch_add(1, atomic::Ordering::Relaxed);
        }
        Self {
            generation: self.generation.clone(),
            width: self.width,
            geometry: self.geometry.clone(),
        }
    }
}

struct FontRunsCursor<'a> {
    runs: &'a [FontRun],
    start: u32,
}

struct CharWidths {
    wrapper: LineWrapperHandle,
    text_system: Arc<TextSystem>,
    base_font: FontId,
    em_width: ScrollPixelOffset,
    metrics: RowMetrics,
    #[cfg(test)]
    shaped_rows: AtomicUsize,
    #[cfg(test)]
    highlighted_rows: AtomicUsize,
}

struct RowShaper {
    snapshot: TabSnapshot,
    row: u32,
    source: Option<Arc<LongRowSource>>,
    previous: Option<Arc<RowGeometry>>,
    widths: Arc<CharWidths>,
    drop_executor: BackgroundExecutor,
    raster_config: GlyphRasterConfig,
}

struct RowGeometry {
    len: u32,
    width: ScrollPixelOffset,
    content_end: u32,
    atoms: AtomIndex,
    pieces: Vec<TextPiece>,
    replacement_glyphs: Vec<(u32, f64, FontId, Pixels)>,
    paint: PaintIndex,
    paint_overhang: Range<ScrollPixelOffset>,
    raster_config: GlyphRasterConfig,
    font_runs: Arc<[FontRun]>,
    metrics: RowMetrics,
    compact: Option<(String, CompactLineLayout)>,
    plain: bool,
    drop_executor: Option<BackgroundExecutor>,
    #[cfg(test)]
    cached_row_clones: AtomicUsize,
}

#[derive(Clone)]
struct FontPaintBounds {
    ink: Range<f64>,
    decoration_width: f64,
}

struct RasterPaintBounds<'a> {
    text_system: &'a TextSystem,
    config: GlyphRasterConfig,
    fonts: HashMap<(FontId, u32), FontPaintBounds>,
    glyphs: HashSet<(FontId, u32, GlyphId, bool)>,
}

impl FontPaintBounds {
    fn new(bounds: Bounds<Pixels>) -> Self {
        let left = f64::from(bounds.origin.x);
        let width = f64::from(bounds.size.width);
        Self {
            ink: left..left + width,
            decoration_width: width,
        }
    }
}

impl RasterPaintBounds<'_> {
    fn for_font(&self, font_id: FontId, font_size: Pixels) -> FontPaintBounds {
        self.fonts
            .get(&(font_id, f32::from(font_size).to_bits()))
            .cloned()
            .unwrap_or_else(|| {
                FontPaintBounds::new(self.text_system.bounding_box(font_id, font_size))
            })
    }

    fn add_layout(&mut self, layout: &LineLayout) {
        for run in &layout.runs {
            self.add_glyphs(
                run.font_id,
                layout.font_size,
                run.glyphs.iter().map(|glyph| (glyph.id, glyph.is_emoji)),
            );
        }
    }

    fn add_glyphs(
        &mut self,
        font_id: FontId,
        font_size: Pixels,
        glyphs: impl IntoIterator<Item = (GlyphId, bool)>,
    ) {
        let size_bits = f32::from(font_size).to_bits();
        let bounds = self.fonts.entry((font_id, size_bits)).or_insert_with(|| {
            FontPaintBounds::new(self.text_system.bounding_box(font_id, font_size))
        });
        let mut previous = None;
        for (glyph_id, is_emoji) in glyphs {
            if previous == Some((glyph_id, is_emoji)) {
                continue;
            }
            previous = Some((glyph_id, is_emoji));
            if self.glyphs.insert((font_id, size_bits, glyph_id, is_emoji))
                && let Some(envelope) = self.text_system.glyph_raster_envelope(
                    font_id,
                    glyph_id,
                    font_size,
                    self.config,
                    is_emoji,
                )
            {
                let scale = f64::from(self.config.scale_factor);
                bounds.ink.start = bounds.ink.start.min(envelope.left() / scale);
                bounds.ink.end = bounds.ink.end.max(envelope.right() / scale);
            }
        }
    }
}

#[derive(Clone)]
struct PaintRegion {
    target: PaintTarget,
    bounds: Range<f64>,
}

#[derive(Clone)]
enum PaintTarget {
    Columns(Range<u32>),
    Glyphs {
        piece: usize,
        glyphs: Range<usize>,
        overhang: Range<f64>,
    },
    Advances {
        piece: usize,
        partitions: Range<usize>,
    },
}

#[derive(Default)]
struct PaintIndex {
    regions: Box<[PaintRegion]>,
    ends: Box<[usize]>,
    nodes: Box<[PaintNode]>,
}

struct PaintNode {
    center: f64,
    regions: Range<usize>,
    left: usize,
    right: usize,
}

impl PaintTarget {
    fn columns(&self, pieces: &[TextPiece], bounds: &Range<f64>, columns: &mut Vec<Range<u32>>) {
        let (piece, partitions) = match self {
            Self::Columns(range) => {
                columns.push(range.clone());
                return;
            }
            Self::Glyphs {
                piece,
                glyphs,
                overhang,
            } => {
                let piece = &pieces[*piece];
                let index = &piece.index;
                let offset = glyphs.start;
                let positions = &index.positions[glyphs.clone()];
                let start =
                    positions.partition_point(|x| piece.x + (x + overhang.end) < bounds.start);
                let end =
                    positions.partition_point(|x| piece.x + (x + overhang.start) <= bounds.end);
                if start >= end {
                    return;
                }
                let start = index.partition_for_glyph(offset + start);
                let end = index.partition_for_glyph(offset + end - 1) + 1;
                (piece, start..end)
            }
            Self::Advances { piece, partitions } => {
                let piece = &pieces[*piece];
                let start = piece
                    .index
                    .partition_point(&piece.layout, partitions.clone(), |x| {
                        piece.x + x < bounds.start
                    })
                    .saturating_sub(1)
                    .max(partitions.start);
                let end = piece
                    .index
                    .partition_point(&piece.layout, partitions.clone(), |x| {
                        piece.x + x <= bounds.end
                    });
                (piece, start..end)
            }
        };
        if !partitions.is_empty() {
            columns.push(
                piece.columns.start
                    + piece.index.partition(&piece.layout, partitions.start).0 as u32
                    ..piece.columns.start
                        + piece.index.partition(&piece.layout, partitions.end).0 as u32,
            );
        }
    }
}

impl PaintIndex {
    fn new(mut regions: Vec<PaintRegion>) -> Self {
        for region in &mut regions {
            if region.bounds.start.is_nan() || region.bounds.end.is_nan() {
                region.bounds = f64::NEG_INFINITY..f64::INFINITY;
            } else if region.bounds.start > region.bounds.end {
                std::mem::swap(&mut region.bounds.start, &mut region.bounds.end);
            }
        }
        let mut ends = (0..regions.len()).collect::<Vec<_>>();
        let mut nodes = Vec::new();
        Self::build(&mut regions, &mut ends, 0, &mut nodes);
        Self {
            regions: regions.into_boxed_slice(),
            ends: ends.into_boxed_slice(),
            nodes: nodes.into_boxed_slice(),
        }
    }

    fn build(
        regions: &mut [PaintRegion],
        ends: &mut [usize],
        offset: usize,
        nodes: &mut Vec<PaintNode>,
    ) -> usize {
        if regions.is_empty() {
            return usize::MAX;
        }
        let middle = regions.len() / 2;
        regions.select_nth_unstable_by(middle, |left, right| {
            left.bounds.start.total_cmp(&right.bounds.start)
        });
        let center = regions[middle].bounds.start;
        let mut left = 0;
        let mut right = regions.len();
        let mut cursor = 0;
        while cursor < right {
            if regions[cursor].bounds.end < center {
                regions.swap(left, cursor);
                left += 1;
                cursor += 1;
            } else if regions[cursor].bounds.start > center {
                right -= 1;
                regions.swap(cursor, right);
            } else {
                cursor += 1;
            }
        }
        regions[left..right]
            .sort_unstable_by(|left, right| left.bounds.start.total_cmp(&right.bounds.start));
        ends[left..right].sort_unstable_by(|left, right| {
            regions[*right - offset]
                .bounds
                .end
                .total_cmp(&regions[*left - offset].bounds.end)
        });
        let index = nodes.len();
        nodes.push(PaintNode {
            center,
            regions: offset + left..offset + right,
            left: usize::MAX,
            right: usize::MAX,
        });
        nodes[index].left = Self::build(&mut regions[..left], &mut ends[..left], offset, nodes);
        nodes[index].right = Self::build(
            &mut regions[right..],
            &mut ends[right..],
            offset + right,
            nodes,
        );
        index
    }

    fn columns(&self, pieces: &[TextPiece], bounds: Range<f64>) -> Vec<Range<u32>> {
        let mut ranges = Vec::new();
        self.query(&bounds, |target| {
            target.columns(pieces, &bounds, &mut ranges)
        });
        ranges.sort_unstable_by_key(|range| range.start);
        ranges.dedup_by(|range, previous| {
            if range.start > previous.end {
                return false;
            }
            previous.end = previous.end.max(range.end);
            true
        });
        ranges
    }

    fn query(&self, bounds: &Range<f64>, mut visit: impl FnMut(&PaintTarget)) -> usize {
        self.query_node(0, bounds, &mut visit)
    }

    fn query_node(
        &self,
        index: usize,
        bounds: &Range<f64>,
        visit: &mut impl FnMut(&PaintTarget),
    ) -> usize {
        let Some(node) = self.nodes.get(index) else {
            return 0;
        };
        if bounds.end < node.center {
            for region in self.regions[node.regions.clone()]
                .iter()
                .take_while(|region| region.bounds.start <= bounds.end)
            {
                visit(&region.target);
            }
            1 + self.query_node(node.left, bounds, visit)
        } else if bounds.start > node.center {
            for region in self.ends[node.regions.clone()]
                .iter()
                .map(|index| &self.regions[*index])
                .take_while(|region| region.bounds.end >= bounds.start)
            {
                visit(&region.target);
            }
            1 + self.query_node(node.right, bounds, visit)
        } else {
            for region in &self.regions[node.regions.clone()] {
                visit(&region.target);
            }
            1 + self.query_node(node.left, bounds, visit)
                + self.query_node(node.right, bounds, visit)
        }
    }
}

#[derive(Default)]
struct AtomIndex {
    columns: Arc<[u32]>,
    positions: Arc<[ScrollPixelOffset]>,
}

#[derive(Default)]
struct AtomBuilder {
    columns: Vec<u32>,
    positions: Vec<ScrollPixelOffset>,
}

struct AtomCursor<'a> {
    columns: &'a [u32],
    end: usize,
}

struct Atom {
    start: u32,
    end: u32,
    start_x: ScrollPixelOffset,
    end_x: ScrollPixelOffset,
}

enum InputRun {
    Text {
        start: u32,
        text: String,
    },
    Renderer {
        start: u32,
        id: ChunkRendererId,
        width: Pixels,
    },
}

struct RowInput {
    runs: Vec<InputRun>,
    len: u32,
    content_end: u32,
    plain: bool,
}

impl LongRow {
    pub(crate) fn len(&self) -> u32 {
        self.geometry.len
    }

    pub(crate) fn width(&self) -> ScrollPixelOffset {
        self.geometry().width
    }

    pub(crate) fn trailing_whitespace_start(&self) -> u32 {
        self.geometry().content_end
    }

    pub(crate) fn metrics(&self) -> RowMetrics {
        self.geometry().metrics
    }

    pub(crate) fn x_for_column(&self, column: u32) -> ScrollPixelOffset {
        let columns = self.atom_range(column);
        self.geometry().x_for_boundary(if columns.start == column {
            column
        } else {
            columns.end
        })
    }

    pub(crate) fn closest_column_for_x(&self, x: ScrollPixelOffset) -> u32 {
        if x <= 0. {
            return 0;
        }
        if x >= self.width() {
            return self.len();
        }
        let atom = self.geometry().atom_at_x(x);
        if x - atom.start_x <= atom.end_x - x {
            atom.start
        } else {
            atom.end
        }
    }

    pub(crate) fn column_at_x(&self, x: ScrollPixelOffset) -> u32 {
        if x <= 0. {
            return 0;
        }
        if x >= self.width() {
            return self.len();
        }
        self.geometry().atom_at_x(x).start
    }

    pub(crate) fn visible_window(
        &self,
        scroll_x: ScrollPixelOffset,
        text_align: TextAlign,
        viewport_width: ScrollPixelOffset,
    ) -> Range<u32> {
        let visible = self.visible_x(scroll_x, text_align, viewport_width);
        let geometry = self.geometry();
        let start_x = visible.start - geometry.paint_overhang.end;
        let end_x = visible.end - geometry.paint_overhang.start;
        if let Some((_, compact)) = &geometry.compact {
            let start = compact
                .positions
                .partition_point(|x| x < start_x)
                .saturating_sub(1) as u32;
            let end = compact.positions.partition_point(|x| x <= end_x) as u32;
            return start..end.max(start);
        }
        let start = geometry
            .atoms
            .positions
            .partition_point(|x| *x < start_x)
            .saturating_sub(1);
        let end = geometry.atoms.positions.partition_point(|x| *x <= end_x);
        let start = geometry
            .atoms
            .columns
            .get(start)
            .copied()
            .unwrap_or(geometry.len);
        let end = geometry
            .atoms
            .columns
            .get(end)
            .copied()
            .unwrap_or(geometry.len);
        self.chunk_aligned(start..end.max(start))
    }

    pub(crate) fn paint_window_bounds(
        bounds: Range<ScaledPixels>,
        origin_x: ScrollPixelOffset,
        scale_factor: f32,
    ) -> Range<ScrollPixelOffset> {
        let scale = f64::from(scale_factor);
        f64::from(bounds.start.0) / scale - origin_x..f64::from(bounds.end.0) / scale - origin_x
    }

    pub(crate) fn paint_ranges(&self, bounds: Range<f64>) -> Vec<Range<u32>> {
        if bounds.start >= bounds.end {
            return Vec::new();
        }
        if self.geometry.compact.is_some() {
            let columns =
                self.visible_window(bounds.start, TextAlign::Left, bounds.end - bounds.start);
            return if columns.is_empty() {
                Vec::new()
            } else {
                vec![columns]
            };
        }
        self.geometry.paint.columns(&self.geometry.pieces, bounds)
    }

    pub(crate) fn window_layout(
        &self,
        columns: Range<u32>,
        origin_x: ScrollPixelOffset,
    ) -> Vec<PaintTextPiece> {
        let geometry = self.geometry();
        let columns = columns.start.min(geometry.len)..columns.end.min(geometry.len);
        if let Some((text, compact)) = &geometry.compact {
            if columns.is_empty() {
                return Vec::new();
            }
            let text = &text[columns.start as usize..columns.end as usize];

            let origin_x = origin_x
                .max(geometry.x_for_boundary(columns.start))
                .min(geometry.x_for_boundary(columns.end));
            let layout = materialize_compact(compact, columns.clone(), origin_x);
            return vec![PaintTextPiece {
                columns,
                x: origin_x,
                text: SharedString::new(text),
                layout: Arc::new(layout),
            }];
        }

        let first = geometry
            .pieces
            .partition_point(|piece| piece.columns.end <= columns.start);
        geometry.pieces[first..]
            .iter()
            .take_while(|piece| piece.columns.start < columns.end)
            .filter_map(|piece| piece.window(&columns, origin_x))
            .collect()
    }

    pub(crate) fn decoration_positions(
        &self,
    ) -> impl Iterator<Item = (u32, f64, FontId, Pixels)> + '_ {
        let geometry = self.geometry();
        geometry
            .compact
            .iter()
            .flat_map(|(_, compact)| {
                (0..compact.positions.len()).filter_map(move |column| {
                    compact
                        .positions
                        .get(column)
                        .map(|x| (column as u32, x, compact.font_id, compact.font_size))
                })
            })
            .chain(geometry.pieces.iter().flat_map(|piece| {
                piece
                    .layout
                    .runs
                    .iter()
                    .flat_map(|run| run.glyphs.iter().map(move |glyph| (glyph, run.font_id)))
                    .zip(piece.index.positions.iter())
                    .map(move |((glyph, font), x)| {
                        let (index, x) = if piece.layout.caret_positions.is_empty() {
                            (glyph.index, *x)
                        } else {
                            piece.layout.cluster_start_for_index(glyph.index)
                        };
                        (
                            piece.columns.start + index as u32,
                            piece.x + x,
                            font,
                            piece.layout.font_size,
                        )
                    })
            }))
            .merge_by(
                geometry.replacement_glyphs.iter().copied(),
                |left, right| left.0 <= right.0,
            )
    }

    pub(crate) fn font_id_for_column(&self, column: u32, columns: &Range<u32>) -> Option<FontId> {
        if !columns.contains(&column) || column >= self.len() {
            return None;
        }
        let geometry = self.geometry();
        if let Some((_, compact)) = &geometry.compact {
            return Some(compact.font_id);
        }
        let first = geometry
            .pieces
            .partition_point(|piece| piece.columns.end <= column);
        let piece = geometry.pieces.get(first)?;
        if !piece.columns.contains(&column) {
            return None;
        }
        let column = (column - piece.columns.start) as usize;
        let index = &piece.index;
        let start = index.boundary(&piece.layout, column, Bias::Left).1;
        let end = index
            .boundary(
                &piece.layout,
                (columns.end.min(piece.columns.end) - piece.columns.start) as usize,
                Bias::Right,
            )
            .1;
        let first_run = index
            .runs
            .partition_point(|offset| *offset <= start)
            .saturating_sub(1);
        piece
            .layout
            .runs
            .iter()
            .enumerate()
            .skip(first_run)
            .take_while(|(run, _)| index.runs[*run] < end)
            .find_map(|(run_index, run)| {
                let run_start = index.runs[run_index];
                run.glyphs
                    .iter()
                    .skip(start.saturating_sub(run_start))
                    .take(end - start.max(run_start))
                    .any(|glyph| glyph.index >= column)
                    .then_some(run.font_id)
            })
    }

    pub(crate) fn chunk_aligned(&self, columns: Range<u32>) -> Range<u32> {
        let geometry = self.geometry();
        geometry.boundary(columns.start, Bias::Left)..geometry.boundary(columns.end, Bias::Right)
    }

    pub(crate) fn floor_column(&self, column: u32) -> u32 {
        self.atom_range(column).start
    }

    fn geometry(&self) -> &Arc<RowGeometry> {
        &self.geometry
    }
}

impl RowShaper {
    fn len(&self) -> u32 {
        self.snapshot.line_len(self.row)
    }

    fn shape(&self) -> Option<Arc<RowGeometry>> {
        let previous = self.previous.clone();
        let mut input = None;
        let plain = if let Some(previous) = &previous {
            previous.plain
        } else {
            let row = RowInput::new(&self.snapshot, self.row)?;
            let plain = row.plain;
            input = Some(row);
            plain
        };
        let font_runs = self.resolved_font_runs(plain);
        if let Some(previous) = previous
            && previous.font_runs == font_runs
            && previous.raster_config == self.raster_config
        {
            return Some(previous);
        }
        let input = match input {
            Some(input) => input,
            None => RowInput::new(&self.snapshot, self.row)?,
        };
        #[cfg(test)]
        self.widths
            .shaped_rows
            .fetch_add(1, atomic::Ordering::Relaxed);
        let mut geometry = RowGeometry::shape(
            input,
            font_runs,
            &self.widths.wrapper,
            self.widths.metrics,
            &self.widths.text_system,
            self.raster_config,
        );
        geometry.drop_executor = Some(self.drop_executor.clone());
        Some(Arc::new(geometry))
    }

    fn resolved_font_runs(&self, plain: bool) -> Arc<[FontRun]> {
        let Some(source) = &self.source else {
            return Arc::from([FontRun {
                len: self.len() as usize,
                font_id: self.widths.base_font,
            }]);
        };
        let buffer = self.snapshot.buffer_snapshot();
        if plain
            && source.text_highlights.is_empty()
            && source.inlay_highlights.is_empty()
            && source.semantic_token_highlights.is_empty()
            && !buffer.has_diff_hunks()
            && buffer
                .buffers_with_paths()
                .all(|(buffer, _)| buffer.syntax_layers().next().is_none())
            && !self.has_row_diagnostics()
        {
            #[cfg(test)]
            source
                .resolved_bytes
                .fetch_add(self.len() as usize, atomic::Ordering::Relaxed);
            return Arc::from([FontRun {
                len: self.len() as usize,
                font_id: source.text_system.resolve_font(&source.style.text.font()),
            }]);
        }
        #[cfg(test)]
        self.widths
            .highlighted_rows
            .fetch_add(1, atomic::Ordering::Relaxed);
        let mut runs = SmallVec::<[FontRun; 1]>::new();
        let mut previous_decoration = None;
        for chunk in source.highlighted_chunks(&self.snapshot, self.row, 0..self.len()) {
            let run = chunk.text_run(&source.style);
            #[cfg(test)]
            source
                .resolved_bytes
                .fetch_add(run.len, atomic::Ordering::Relaxed);
            if run.len == 0 {
                continue;
            }
            let font_id = source.text_system.resolve_font(&run.font);
            let decoration = (run.color, run.underline, run.strikethrough);
            if let Some(previous) = runs.last_mut()
                && previous.font_id == font_id
                && previous_decoration == Some(decoration)
            {
                previous.len += run.len;
            } else {
                runs.push(FontRun {
                    len: run.len,
                    font_id,
                });
            }
            previous_decoration = Some(decoration);
        }
        Arc::from(runs.into_vec())
    }

    fn has_row_diagnostics(&self) -> bool {
        let buffer = self.snapshot.buffer_snapshot();
        let start = self
            .snapshot
            .tab_point_to_point(TabPoint::new(self.row, 0), Bias::Left);
        let end = self
            .snapshot
            .tab_point_to_point(TabPoint::new(self.row, self.len()), Bias::Right);
        let start = buffer.point_to_offset(start);
        let start = buffer.clip_offset(MultiBufferOffset(start.0.saturating_sub(1)), Bias::Left);
        buffer
            .diagnostics_in_range(start..buffer.point_to_offset(end))
            .next()
            .is_some()
    }
}

impl LongRow {
    fn atom_range(&self, column: u32) -> Range<u32> {
        let geometry = self.geometry();
        if column >= geometry.len {
            return geometry.len..geometry.len;
        }
        if geometry.compact.is_some() {
            return column..column + 1;
        }
        let index = geometry
            .atoms
            .columns
            .partition_point(|start| *start <= column)
            .saturating_sub(1);
        let start = geometry.atoms.columns.get(index).copied().unwrap_or(0);
        let end = geometry
            .atoms
            .columns
            .get(index + 1)
            .copied()
            .unwrap_or(geometry.len);
        start..end
    }

    fn visible_x(
        &self,
        scroll_x: ScrollPixelOffset,
        text_align: TextAlign,
        viewport_width: ScrollPixelOffset,
    ) -> Range<ScrollPixelOffset> {
        let viewport_width = viewport_width.max(0.);
        let free_width = viewport_width - self.width();
        let alignment_offset = match text_align {
            TextAlign::Left => 0.,
            TextAlign::Center => free_width / 2.,
            TextAlign::Right => free_width,
        };
        let start = scroll_x - alignment_offset;
        start..start + viewport_width
    }
}

impl LongRows {
    pub(crate) fn new(
        mut wrapper: LineWrapperHandle,
        text_system: Arc<TextSystem>,
        version: usize,
    ) -> Self {
        let em_width = f64::from(wrapper.width_for_char('m'));
        let base_font = wrapper.font_id();
        let layout = wrapper.layout_line(
            " ",
            &[FontRun {
                len: 1,
                font_id: base_font,
            }],
        );
        Self {
            widths: Some(Arc::new(CharWidths {
                metrics: RowMetrics {
                    ascent: layout.ascent,
                    descent: layout.descent,
                },
                wrapper,
                text_system,
                base_font,
                em_width,
                #[cfg(test)]
                shaped_rows: AtomicUsize::new(0),
                #[cfg(test)]
                highlighted_rows: AtomicUsize::new(0),
            })),
            version,
            ..Self::default()
        }
    }

    pub(super) fn identity(&self) -> Arc<RowCache> {
        self.rows.clone()
    }

    pub(super) fn detached(&self, executor: BackgroundExecutor) -> DetachedLongRows {
        self.rows.drop_executor.set(executor).ok();
        DetachedLongRows {
            cache: self.clone(),
        }
    }

    pub(crate) fn em_width(&self) -> Option<ScrollPixelOffset> {
        Some(self.widths.as_ref()?.em_width)
    }

    pub(crate) fn row(&self, snapshot: &TabSnapshot, row: u32) -> Option<LongRow> {
        self.widths.as_ref()?;
        if self.version != snapshot.version || snapshot.line_len(row) as usize <= MAX_LINE_LEN {
            return None;
        }
        let rows = self.rows.try_lock()?;
        let cached = rows
            .get(&row)
            .filter(|cached| self.has_current_source(cached))?;
        Some(LongRow {
            geometry: cached.geometry.upgrade()?,
            #[cfg(test)]
            widths: self.widths.clone()?,
        })
    }

    pub(super) fn publish_row(&self, snapshot: &TabSnapshot, row: u32, geometry: LongRow) {
        self.rows.snapshot.set(snapshot.clone()).ok();
        let previous = self.rows.lock().insert_or_replace(
            row,
            CachedRow {
                generation: Arc::downgrade(&self.identity),
                width: geometry.width(),
                geometry: Arc::downgrade(&geometry.geometry),
            },
        );
        drop(previous);
    }

    pub(super) fn publish_width(&self, snapshot: &TabSnapshot, row: u32, width: ScrollPixelOffset) {
        self.rows.snapshot.set(snapshot.clone()).ok();
        let previous = self.rows.lock().insert_or_replace(
            row,
            CachedRow {
                generation: Arc::downgrade(&self.identity),
                width,
                geometry: Weak::new(),
            },
        );
        drop(previous);
    }

    pub(super) fn publish_max_width(&self, width: ScrollPixelOffset) {
        self.completed_max_width.set(width).ok();
    }

    pub(crate) fn interpolate(&self, snapshot: &TabSnapshot, edits: &[TabEdit]) -> Self {
        if edits.is_empty() {
            return Self {
                version: snapshot.version,
                ..self.clone()
            };
        }
        let previous_snapshot = self.rows.snapshot.get();
        let mut completed_max_width = self.completed_max_width.get().copied();
        if edits.len() > 64 {
            completed_max_width = None;
        }
        if completed_max_width.is_some() {
            if let Some(previous_snapshot) = previous_snapshot {
                for (input, old) in [(previous_snapshot, true), (snapshot, false)] {
                    let mut remaining_rows = 64_usize;
                    for edit in edits {
                        let range = if old { &edit.old } else { &edit.new };
                        for row in range.start.row()..=range.end.row() {
                            if remaining_rows == 0 || input.line_len(row) as usize > MAX_LINE_LEN {
                                completed_max_width = None;
                                break;
                            }
                            remaining_rows -= 1;
                        }
                        if completed_max_width.is_none() {
                            break;
                        }
                    }
                    if completed_max_width.is_none() {
                        break;
                    }
                }
            } else {
                completed_max_width = None;
            }
        }
        let preserve_rows = edits.len() <= 64
            && edits.iter().all(|edit| {
                edit.old.start.row() == edit.new.start.row()
                    && edit.old.end.row() == edit.new.end.row()
            });
        let mut rows = if preserve_rows {
            self.rows.lock().clone()
        } else {
            TreeMap::default()
        };
        if preserve_rows {
            for edit in edits {
                let start = edit.old.start.row();
                if rows
                    .iter_from(&start)
                    .next()
                    .is_some_and(|(row, _)| *row <= edit.old.end.row())
                {
                    let Some(end) = edit.old.end.row().checked_add(1) else {
                        rows = TreeMap::default();
                        break;
                    };
                    rows.remove_range(&start, &end);
                }
            }
        }
        Self {
            rows: Arc::new(RowCache {
                rows: Mutex::new(rows),
                snapshot: OnceLock::from(snapshot.clone()),
                drop_executor: self.rows.drop_executor.clone(),
            }),
            completed_max_width: completed_max_width
                .map_or_else(Arc::default, |width| Arc::new(OnceLock::from(width))),
            version: snapshot.version,
            ..self.clone()
        }
    }

    pub(super) fn set_raster_config(&mut self, config: GlyphRasterConfig) {
        if self.raster_config == config {
            return;
        }
        self.raster_config = config;
        self.rows = Arc::new(RowCache {
            rows: Mutex::new(TreeMap::default()),
            snapshot: OnceLock::new(),
            drop_executor: self.rows.drop_executor.clone(),
        });
        self.identity = Arc::default();
    }

    pub(super) fn set_shaping_source(&mut self, source: Arc<LongRowSource>) {
        let rows = self.rows.lock().clone();
        self.rows = Arc::new(RowCache {
            rows: Mutex::new(rows),
            snapshot: OnceLock::new(),
            drop_executor: self.rows.drop_executor.clone(),
        });
        self.completed_max_width = Arc::default();
        self.source = Some(source);
        self.identity = Arc::default();
    }

    pub(super) fn try_max_width(
        &self,
        snapshot: &TabSnapshot,
        excluded: &[Range<WrapRow>],
    ) -> Option<ScrollPixelOffset> {
        if self.widths.is_none() {
            return Some(0.);
        }
        if self.version != snapshot.version {
            return None;
        }
        if excluded.is_empty()
            && let Some(width) = self.completed_max_width.get()
        {
            return Some(*width);
        }

        let row_count = snapshot.max_point().row() + 1;
        let mut start = 0;
        let mut maximum = 0_f64;
        let mut remaining_steps = 64_usize;
        for excluded in excluded
            .iter()
            .cloned()
            .chain([WrapRow(row_count)..WrapRow(row_count)])
        {
            let mut scan = LongRowScan::new(start..excluded.start.0.min(row_count));
            loop {
                remaining_steps = remaining_steps.checked_sub(1)?;
                match scan.step(snapshot) {
                    ScanStep::Row { row } => {
                        let width = self
                            .rows
                            .try_lock()?
                            .get(&row)
                            .filter(|cached| self.has_current_source(cached))?
                            .width;
                        maximum = maximum.max(width);
                    }
                    ScanStep::Continue => {}
                    ScanStep::Done => break,
                }
            }
            start = start.max(excluded.end.0.min(row_count));
        }
        Some(maximum)
    }

    fn has_current_source(&self, row: &CachedRow) -> bool {
        Weak::ptr_eq(&Arc::downgrade(&self.identity), &row.generation)
    }
}

impl LongRowWidthRequest {
    pub(super) async fn prepare(
        &self,
        results: &mut mpsc::Sender<(Arc<RowReadiness>, LongRowCompletion)>,
    ) -> Option<ScrollPixelOffset> {
        let snapshot = &self.snapshot;
        if snapshot.has_unrepresentable_rows() {
            return None;
        }
        let excluded = &self.excluded;
        let row_count = snapshot.max_point().row() + 1;
        let mut start = 0;
        let mut maximum = 0_f64;
        let mut steps = 0;
        for range in excluded
            .iter()
            .cloned()
            .chain([WrapRow(row_count)..WrapRow(row_count)])
        {
            let mut scan = LongRowScan::new(start..range.start.0.min(row_count));
            loop {
                if self.cancelled.load(atomic::Ordering::Acquire) {
                    return None;
                }
                let required = {
                    let mut priority = self.priority.lock();
                    priority
                        .input
                        .swap_remove_index(0)
                        .map(|(request, retained)| (request, Some(retained)))
                        .or_else(|| {
                            priority
                                .visible
                                .swap_remove_index(0)
                                .map(|request| (request, None))
                        })
                };
                let constraint = required
                    .is_none()
                    .then(|| self.priority.lock().constraints.pop_first())
                    .flatten();
                if let Some((id, job)) = constraint {
                    let prepared = self.prepare_renderer_constraint(id, &job);
                    if self.cancelled.load(atomic::Ordering::Acquire)
                        || results
                            .send((
                                self.readiness.clone(),
                                LongRowCompletion::Constraint(id, job, prepared),
                            ))
                            .await
                            .is_err()
                    {
                        return None;
                    }
                    yield_now().await;
                    continue;
                }
                if let Some(((row, text_only), retained)) = required {
                    if row < row_count
                        && !excluded.iter().any(|range| range.contains(&WrapRow(row)))
                    {
                        let text = self.prepare_row_text(row);
                        if self.cancelled.load(atomic::Ordering::Acquire) {
                            return None;
                        }
                        let geometry = (!text_only)
                            .then(|| self.rows.shape_row(snapshot, row))
                            .flatten();
                        if self.cancelled.load(atomic::Ordering::Acquire)
                            || !self
                                .publish_row((row, text_only), geometry, text, results)
                                .await
                        {
                            return None;
                        }
                        drop(retained);
                        yield_now().await;
                    }
                    continue;
                }
                if self.rows.cache.version != snapshot.version
                    || !self.measure_width.load(atomic::Ordering::Acquire)
                {
                    return None;
                }
                match scan.step(snapshot) {
                    ScanStep::Row { row } => {
                        let width = self
                            .rows
                            .cache
                            .rows
                            .lock()
                            .get(&row)
                            .filter(|cached| self.rows.cache.has_current_source(cached))
                            .map(|cached| cached.width);
                        if let Some(width) = width {
                            maximum = maximum.max(width);
                        } else {
                            let geometry = self.rows.shape_row(snapshot, row);
                            if let Some(geometry) = &geometry {
                                maximum = maximum.max(geometry.width());
                            }
                            if self.cancelled.load(atomic::Ordering::Acquire)
                                || !self
                                    .publish_row((row, false), geometry, None, results)
                                    .await
                            {
                                return None;
                            }
                        }
                        yield_now().await;
                    }
                    ScanStep::Continue => {
                        steps += 1;
                        if steps == 64 {
                            steps = 0;
                            yield_now().await;
                        }
                    }
                    ScanStep::Done => break,
                }
            }
            start = start.max(range.end.0.min(row_count));
        }
        (!self.cancelled.load(atomic::Ordering::Acquire)).then_some(maximum)
    }

    fn prepare_renderer_constraint(
        &self,
        id: ChunkRendererId,
        job: &RendererConstraintJob,
    ) -> Option<Arc<PreparedRendererConstraint>> {
        if let Some(input) = &job.input {
            return Some(Arc::new(input.measure(&self.text_system)));
        }
        let (font, font_size) = job.font.as_ref()?;
        let (renderer, text) = self.snapshot.fold_snapshot.renderer_source(id)?;
        let input = PreparedRendererConstraint::new(renderer, text, font.clone(), *font_size);
        if self.cancelled.load(atomic::Ordering::Acquire) {
            return None;
        }
        Some(Arc::new(input.measure(&self.text_system)))
    }

    fn prepare_row_text(&self, row: u32) -> Option<Arc<PreparedRowText>> {
        let source = self.paint_source.as_ref()?;
        if self.snapshot.line_len(row) as usize <= MAX_LINE_LEN {
            return None;
        }
        if let Some(text) = self
            .readiness
            .retained_row(row)
            .and_then(|row| row.text.get().cloned())
        {
            return Some(text);
        }
        Some(Arc::new(
            PreparedRowText::new(source.highlighted_chunks(
                &self.snapshot,
                row,
                0..self.snapshot.line_len(row),
            ))
            .with_drop_executor(self.executor.clone()),
        ))
    }

    async fn publish_row(
        &self,
        request: (u32, bool),
        geometry: Option<LongRow>,
        text: Option<Arc<PreparedRowText>>,
        results: &mut mpsc::Sender<(Arc<RowReadiness>, LongRowCompletion)>,
    ) -> bool {
        let prepared = text.clone().or_else(|| {
            self.readiness
                .retained_row(request.0)
                .and_then(|row| row.text.get().cloned())
        });
        if let Some(prepared) = prepared
            && let Some(source) = &self.paint_source
            && let Some(widths) = &self.rows.cache.widths
            && let Some(row) = geometry.clone().or_else(|| {
                let rows = self.rows.cache.rows.lock();
                let cached = rows
                    .get(&request.0)
                    .filter(|cached| self.rows.cache.has_current_source(cached))?;
                Some(LongRow {
                    geometry: cached.geometry.upgrade()?,
                    #[cfg(test)]
                    widths: widths.clone(),
                })
            })
        {
            prepared.prepare_cursor_widths(
                &row,
                &source.style.text,
                &widths.wrapper,
                &self.text_system,
            );
        }
        if self.cancelled.load(atomic::Ordering::Acquire) {
            return false;
        }
        let (accepted, completion) = oneshot::channel();
        let retained = (geometry.clone(), text.clone());
        if results
            .send((
                self.readiness.clone(),
                LongRowCompletion::Row(request, geometry, text, accepted),
            ))
            .await
            .is_err()
        {
            return false;
        }
        let accepted = completion.await.unwrap_or(false);
        drop(retained);
        accepted
    }
}

impl DetachedLongRows {
    fn shape_row(&self, snapshot: &TabSnapshot, row: u32) -> Option<LongRow> {
        if let Some(geometry) = self.cache.row(snapshot, row) {
            return Some(geometry);
        }
        let widths = self.cache.widths.as_ref()?;
        let previous = self
            .cache
            .rows
            .lock()
            .get(&row)
            .and_then(|cached| cached.geometry.upgrade());
        let shaper = RowShaper {
            snapshot: snapshot.clone(),
            row,
            source: self.cache.source.clone(),
            previous,
            widths: widths.clone(),
            drop_executor: self.cache.rows.drop_executor.get()?.clone(),
            raster_config: self.cache.raster_config,
        };
        Some(LongRow {
            geometry: shaper.shape()?,
            #[cfg(test)]
            widths: widths.clone(),
        })
    }
}

impl RowCache {
    fn lock(&self) -> MutexGuard<'_, TreeMap<u32, CachedRow>> {
        self.rows.lock()
    }

    fn try_lock(&self) -> Option<MutexGuard<'_, TreeMap<u32, CachedRow>>> {
        self.rows.try_lock()
    }
}

impl Drop for RowCache {
    fn drop(&mut self) {
        if let Some(executor) = self.drop_executor.get()
            && executor.is_main_thread()
        {
            let rows = std::mem::take(self.rows.get_mut());
            let snapshot = self.snapshot.take();
            executor
                .spawn(async move {
                    drop((rows, snapshot));
                })
                .detach();
        }
    }
}

impl Drop for RowGeometry {
    fn drop(&mut self) {
        if let Some(executor) = &self.drop_executor
            && executor.is_main_thread()
        {
            let buffers = (
                std::mem::take(&mut self.atoms),
                std::mem::take(&mut self.pieces),
                std::mem::take(&mut self.replacement_glyphs),
                std::mem::take(&mut self.paint),
                std::mem::take(&mut self.font_runs),
                self.compact.take(),
            );
            executor
                .spawn(async move {
                    drop(buffers);
                })
                .detach();
        }
    }
}

impl<'a> FontRunsCursor<'a> {
    fn new(runs: &'a [FontRun]) -> Self {
        Self { runs, start: 0 }
    }

    fn slice(&mut self, columns: Range<u32>) -> SmallVec<[FontRun; 1]> {
        while let Some(run) = self.runs.first()
            && self.start + run.len as u32 <= columns.start
        {
            self.start += run.len as u32;
            self.runs = &self.runs[1..];
        }
        slice_font_runs(self.runs.iter().copied(), self.start, columns)
    }
}

impl<'a> AtomCursor<'a> {
    fn new(columns: &'a [u32]) -> Self {
        Self { columns, end: 0 }
    }

    fn at(&mut self, column: usize) -> Option<usize> {
        while self
            .columns
            .get(self.end)
            .is_some_and(|start| *start as usize <= column)
        {
            self.end += 1;
        }
        let last = self.end.checked_sub(1)?;
        if self.columns[last] as usize <= column {
            Some(last)
        } else {
            self.columns[..self.end]
                .partition_point(|start| *start as usize <= column)
                .checked_sub(1)
        }
    }
}

impl AtomBuilder {
    fn push(&mut self, column: u32, x: ScrollPixelOffset) {
        self.columns.push(column);
        self.positions.push(x);
    }

    fn finish(self, piece: Option<&TextPiece>) -> AtomIndex {
        let columns = match piece {
            Some(piece) if piece.index.partition_columns.as_ref() == self.columns.as_slice() => {
                piece.index.partition_columns.clone()
            }
            _ => Arc::from(self.columns),
        };
        let positions = match piece {
            Some(piece) if piece.index.positions.as_ref() == self.positions.as_slice() => {
                piece.index.positions.clone()
            }
            _ => Arc::from(self.positions),
        };
        AtomIndex { columns, positions }
    }
}

impl RowInput {
    fn new(snapshot: &TabSnapshot, row: u32) -> Option<Self> {
        let len = snapshot.line_len(row);
        let inlay = &snapshot.fold_snapshot.inlay_snapshot;
        if !snapshot.fold_snapshot.has_folds()
            && inlay.len().0 == inlay.buffer.len()
            && !inlay.buffer.has_diff_hunks()
            && let Some(buffer) = inlay.buffer.as_singleton()
            && {
                let mut excerpts = inlay.buffer.excerpts();
                excerpts.next().is_some_and(|excerpt| {
                    excerpt.context.start.to_offset(buffer) == 0
                        && excerpt.context.end.to_offset(buffer) == buffer.len()
                }) && excerpts.next().is_none()
            }
            && buffer.line_len(row) == len
        {
            let mut text = String::with_capacity(len as usize);
            for chunk in buffer.text_for_range(Point::new(row, 0)..Point::new(row, len)) {
                if !chunk.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                    break;
                }
                text.push_str(chunk);
            }
            if text.len() == len as usize {
                return Some(Self {
                    content_end: text.trim_end_matches(' ').len() as u32,
                    runs: vec![InputRun::Text { start: 0, text }],
                    len,
                    plain: true,
                });
            }
        }
        let mut input = Self {
            runs: Vec::new(),
            len,
            content_end: 0,
            plain: true,
        };
        let mut column = 0;
        for chunk in row_chunks(snapshot, row, 0..len).1 {
            let renderer = match chunk.renderer.as_ref() {
                Some(renderer) => Some((renderer.id, renderer.measured_width?)),
                None => None,
            };
            input.plain &= renderer.is_none()
                && !chunk.is_inlay
                && chunk.text.bytes().all(|byte| (b' '..=b'~').contains(&byte));
            if renderer.is_some() || chunk.is_inlay {
                input.content_end = column + chunk.text.len() as u32;
            } else if let Some((offset, character)) = chunk
                .text
                .char_indices()
                .rev()
                .find(|(_, character)| !character.is_whitespace() || is_invisible(*character))
            {
                input.content_end = column + (offset + character.len_utf8()) as u32;
            }
            match (renderer, input.runs.last_mut()) {
                (Some((id, _)), Some(InputRun::Renderer { id: previous, .. }))
                    if *previous == id => {}
                (Some((id, width)), _) => input.runs.push(InputRun::Renderer {
                    start: column,
                    id,
                    width,
                }),
                (None, Some(InputRun::Text { text, .. })) => text.push_str(chunk.text),
                (None, _) => {
                    let mut text = String::with_capacity(if input.runs.is_empty() {
                        len as usize
                    } else {
                        chunk.text.len()
                    });
                    text.push_str(chunk.text);
                    input.runs.push(InputRun::Text {
                        start: column,
                        text,
                    });
                }
            }
            column += chunk.text.len() as u32;
        }
        Some(input)
    }
}

impl RowGeometry {
    fn shape(
        input: RowInput,
        font_runs: Arc<[FontRun]>,
        wrapper: &LineWrapper,
        metrics: RowMetrics,
        text_system: &TextSystem,
        raster_config: GlyphRasterConfig,
    ) -> Self {
        let mut paint_bounds = RasterPaintBounds {
            text_system,
            config: raster_config,
            fonts: HashMap::default(),
            glyphs: HashSet::default(),
        };
        let len = input.len;
        let mut geometry = Self {
            len,
            width: 0.,
            content_end: input.content_end,
            atoms: AtomIndex::default(),
            pieces: Vec::new(),
            replacement_glyphs: Vec::new(),
            paint: PaintIndex::default(),
            paint_overhang: 0.0..0.0,
            raster_config,
            font_runs,
            metrics,
            compact: None,
            plain: input.plain,
            drop_executor: None,
            #[cfg(test)]
            cached_row_clones: AtomicUsize::new(0),
        };
        let mut atoms = AtomBuilder::default();
        let mut paint = Vec::new();

        let font_runs = geometry.font_runs.clone();
        let mut fonts = FontRunsCursor::new(&font_runs);
        let mut runs = input.runs.into_iter().peekable();
        while let Some(run) = runs.next() {
            let (run_start, text) = match run {
                InputRun::Text { start, text } => (start, text),
                InputRun::Renderer { start, width, .. } => {
                    atoms.push(start, geometry.width);
                    let end = runs.peek().map_or(len, |run| match run {
                        InputRun::Text { start, .. } | InputRun::Renderer { start, .. } => *start,
                    });
                    paint.push(PaintRegion {
                        target: PaintTarget::Columns(start..end),
                        bounds: geometry.width..geometry.width + f64::from(width),
                    });
                    geometry.width += f64::from(width);
                    continue;
                }
            };
            if text.bytes().all(|byte| (b' '..=b'~').contains(&byte))
                || !text.chars().any(is_invisible)
            {
                geometry.push_text(
                    run_start,
                    Cow::Owned(text),
                    &mut fonts,
                    wrapper,
                    &mut paint_bounds,
                    &mut atoms,
                );
                continue;
            }
            let mut start = 0;
            for (offset, grapheme) in text.grapheme_indices(true) {
                let Some(shown) = single_invisible(grapheme).and_then(replacement) else {
                    continue;
                };
                geometry.push_text(
                    run_start + start as u32,
                    Cow::Borrowed(&text[start..offset]),
                    &mut fonts,
                    wrapper,
                    &mut paint_bounds,
                    &mut atoms,
                );
                let column = run_start + offset as u32;
                let end = column + grapheme.len() as u32;

                atoms.push(column, geometry.width);
                let font_id = fonts
                    .slice(column..end)
                    .first()
                    .map_or(wrapper.font_id(), |run| run.font_id);
                let mut bytes = [0; 4];
                let shown = shown.encode_utf8(&mut bytes);
                let (layout, positions, width) = wrapper.layout_line_with_precise_positions(
                    shown,
                    &[FontRun {
                        len: shown.len(),
                        font_id,
                    }],
                );
                geometry.replacement_glyphs.extend(
                    layout
                        .runs
                        .iter()
                        .flat_map(|run| run.glyphs.iter().map(move |glyph| (glyph, run.font_id)))
                        .zip(&positions)
                        .map(|((glyph, font), x)| {
                            let x = if layout.caret_positions.is_empty() {
                                *x
                            } else {
                                layout.cluster_start_for_index(glyph.index).1
                            };
                            (column, geometry.width + x, font, layout.font_size)
                        }),
                );
                geometry.metrics.ascent = geometry.metrics.ascent.max(layout.ascent);
                geometry.metrics.descent = geometry.metrics.descent.max(layout.descent);
                paint_bounds.add_layout(&layout);
                let overhang = glyph_overhang(&layout, &positions, &paint_bounds, |_| 0.);
                geometry.paint_overhang.start = geometry.paint_overhang.start.min(overhang.start);
                geometry.paint_overhang.end = geometry.paint_overhang.end.max(overhang.end);
                paint.push(PaintRegion {
                    target: PaintTarget::Columns(column..end),
                    bounds: geometry.width + overhang.start.min(0.)
                        ..geometry.width + overhang.end.max(width),
                });
                geometry.width += width;
                start = offset + grapheme.len();
            }
            geometry.push_text(
                run_start + start as u32,
                Cow::Borrowed(&text[start..]),
                &mut fonts,
                wrapper,
                &mut paint_bounds,
                &mut atoms,
            );
        }
        if geometry.atoms.columns.is_empty() {
            geometry.atoms = atoms.finish(
                geometry
                    .pieces
                    .first()
                    .filter(|piece| geometry.pieces.len() == 1 && piece.columns == (0..len)),
            );
        }
        let mut cursor = AtomCursor::new(&geometry.atoms.columns);
        for piece in &geometry.pieces {
            let overhang = glyph_overhang(
                &piece.layout,
                &piece.index.positions,
                &paint_bounds,
                |column| {
                    let atom = cursor
                        .at(piece.columns.start as usize + column)
                        .unwrap_or(0);
                    geometry
                        .atoms
                        .positions
                        .get(atom)
                        .copied()
                        .unwrap_or(piece.x)
                        - piece.x
                },
            );
            geometry.paint_overhang.start = geometry.paint_overhang.start.min(overhang.start);
            geometry.paint_overhang.end = geometry.paint_overhang.end.max(overhang.end);
        }
        for piece in std::mem::take(&mut geometry.pieces) {
            piece.split_large_text_dependencies(&mut geometry.pieces);
        }
        for (index, piece) in geometry.pieces.iter().enumerate() {
            piece.paint_regions(
                index,
                |font, size| paint_bounds.for_font(font, size),
                &mut paint,
            );
        }
        geometry.paint = PaintIndex::new(paint);
        geometry
    }

    fn push_text(
        &mut self,
        start: u32,
        text: Cow<'_, str>,
        fonts: &mut FontRunsCursor<'_>,
        wrapper: &LineWrapper,
        paint_bounds: &mut RasterPaintBounds<'_>,
        row_atoms: &mut AtomBuilder,
    ) {
        if text.is_empty() {
            return;
        }
        let columns = start..start + text.len() as u32;
        let fonts = fonts.slice(columns.clone());
        let printable_ascii = text.bytes().all(|byte| (b' '..=b'~').contains(&byte));
        let (layout, positions, width) = match wrapper.layout_line_for_retention(&text, &fonts) {
            RetainedLineLayout::Dense(layout, positions, width) => (layout, positions, width),
            RetainedLineLayout::Compact(compact) => {
                paint_bounds.add_glyphs(
                    compact.font_id,
                    compact.font_size,
                    compact
                        .glyphs
                        .iter()
                        .map(|glyph| (GlyphId(u32::from(*glyph)), compact.is_emoji)),
                );
                if start == 0 && text.len() == self.len as usize && printable_ascii {
                    self.width = compact.width;
                    self.metrics.ascent = self.metrics.ascent.max(compact.ascent);
                    self.metrics.descent = self.metrics.descent.max(compact.descent);
                    let bounds = paint_bounds.for_font(compact.font_id, compact.font_size);
                    let min_x = bounds.ink.start;
                    let max_x = bounds.ink.end;
                    self.paint_overhang = min_x.min(0.)..max_x.max(0.);
                    self.compact = Some((text.into_owned(), compact));
                    return;
                }
                let layout = materialize_compact(&compact, 0..text.len() as u32, 0.);
                let positions = (0..text.len())
                    .map(|column| compact.positions.get(column).unwrap_or(compact.width))
                    .collect();
                (layout, positions, compact.width)
            }
        };
        paint_bounds.add_layout(&layout);
        let index = LayoutIndex::new(&layout, positions, width);
        let end_x = self.width + width;
        let shared = start == 0
            && self.width == 0.
            && row_atoms.columns.is_empty()
            && text.len() == self.len as usize
            && index.partition_glyphs.is_none()
            && index.partition_columns.len() == index.positions.len()
            && (layout.caret_positions.is_empty()
                || layout.caret_positions.iter().copied().eq(index
                    .partition_columns
                    .iter()
                    .zip(index.positions.iter())
                    .map(|(column, x)| (*column as usize, *x))
                    .chain([(layout.len, width)])))
            && width.is_finite()
            && (if printable_ascii {
                index
                    .partition_columns
                    .iter()
                    .copied()
                    .eq(0..text.len() as u32)
            } else {
                text.grapheme_indices(true)
                    .map(|(offset, _)| offset as u32)
                    .eq(index.partition_columns.iter().copied())
            })
            && index
                .positions
                .iter()
                .try_fold(0., |previous, position| {
                    (*position >= previous && *position <= width).then_some(*position)
                })
                .is_some();
        if shared {
            self.atoms = AtomIndex {
                columns: index.partition_columns.clone(),
                positions: index.positions.clone(),
            };
        } else {
            let atoms = if printable_ascii {
                (0..text.len() as u32).collect::<Vec<_>>()
            } else {
                text.grapheme_indices(true)
                    .map(|(offset, _)| offset as u32)
                    .collect::<Vec<_>>()
            };
            let positions = atom_offsets_x(&layout, &atoms, &index);
            for (column, position) in atoms.into_iter().zip(positions) {
                let previous = row_atoms.positions.last().copied().unwrap_or(self.width);
                row_atoms.push(
                    start + column,
                    (self.width + position).clamp(previous, end_x.max(previous)),
                );
            }
        }
        self.metrics.ascent = self.metrics.ascent.max(layout.ascent);
        self.metrics.descent = self.metrics.descent.max(layout.descent);
        self.pieces.push(TextPiece {
            source_start_column: columns.start,
            columns,
            x: self.width,
            text: SharedString::new(text.as_ref()),
            layout: Arc::new(layout),
            index,
        });
        self.width = end_x;
    }

    fn x_for_boundary(&self, column: u32) -> ScrollPixelOffset {
        if let Some((_, compact)) = &self.compact {
            return compact
                .positions
                .get(column.min(self.len) as usize)
                .unwrap_or(compact.width);
        }
        if column == 0 {
            return 0.;
        }
        self.atoms
            .positions
            .get(self.atoms.columns.partition_point(|start| *start < column))
            .copied()
            .unwrap_or(self.width)
    }

    fn atom_at_x(&self, x: ScrollPixelOffset) -> Atom {
        if let Some((_, compact)) = &self.compact {
            let start = compact
                .positions
                .partition_point(|position| position <= x)
                .saturating_sub(1) as u32;
            let end = (start + 1).min(self.len);
            return Atom {
                start,
                end,
                start_x: self.x_for_boundary(start),
                end_x: self.x_for_boundary(end),
            };
        }
        let mut index = self
            .atoms
            .positions
            .partition_point(|position| *position <= x)
            .saturating_sub(1);
        if let Some(piece) = self
            .pieces
            .partition_point(|piece| piece.x <= x)
            .checked_sub(1)
            .and_then(|index| self.pieces.get(index))
            && self
                .atoms
                .columns
                .get(index)
                .is_some_and(|column| *column < piece.source_start_column)
        {
            index = self
                .atoms
                .columns
                .partition_point(|column| *column < piece.source_start_column);
        }
        let start = self.atoms.columns.get(index).copied().unwrap_or(self.len);
        let start_x = self
            .atoms
            .positions
            .get(index)
            .copied()
            .unwrap_or(self.width);
        let end = self
            .atoms
            .columns
            .get(index + 1)
            .copied()
            .unwrap_or(self.len);
        let end_x = self
            .atoms
            .positions
            .get(index + 1)
            .copied()
            .unwrap_or(self.width);
        Atom {
            start,
            end,
            start_x,
            end_x,
        }
    }

    fn boundary(&self, column: u32, bias: Bias) -> u32 {
        if self.compact.is_some() {
            return column.min(self.len);
        }
        if column >= self.len {
            return self.len;
        }
        let index = self
            .pieces
            .partition_point(|piece| piece.columns.end <= column);
        if let Some(piece) = self
            .pieces
            .get(index)
            .filter(|piece| piece.columns.start <= column)
        {
            return piece.columns.start
                + piece
                    .index
                    .boundary(&piece.layout, (column - piece.columns.start) as usize, bias)
                    .0 as u32;
        }
        let index = match bias {
            Bias::Left => self
                .atoms
                .columns
                .partition_point(|start| *start <= column)
                .saturating_sub(1),
            Bias::Right => self.atoms.columns.partition_point(|start| *start < column),
        };
        self.atoms.columns.get(index).copied().unwrap_or(self.len)
    }
}

fn materialize_compact(
    compact: &CompactLineLayout,
    columns: Range<u32>,
    origin_x: ScrollPixelOffset,
) -> LineLayout {
    let glyphs = compact.glyphs[columns.start as usize..columns.end as usize]
        .iter()
        .enumerate()
        .map(|(index, glyph)| {
            let x = compact
                .positions
                .get(columns.start as usize + index)
                .unwrap_or(compact.width)
                - origin_x;
            ShapedGlyph {
                id: GlyphId(u32::from(*glyph)),
                position: point(px(x as f32), Pixels::ZERO),
                index,
                is_emoji: compact.is_emoji,
            }
        })
        .collect();
    let width = compact
        .positions
        .get(columns.end as usize)
        .unwrap_or(compact.width)
        - origin_x;
    LineLayout {
        font_size: compact.font_size,
        width: px(width as f32),
        ascent: compact.ascent,
        descent: compact.descent,
        runs: vec![ShapedRun {
            font_id: compact.font_id,
            glyphs,
        }],
        len: columns.len(),
        caret_positions: (columns.start..=columns.end)
            .map(|column| {
                (
                    (column - columns.start) as usize,
                    compact
                        .positions
                        .get(column as usize)
                        .unwrap_or(compact.width)
                        - origin_x,
                )
            })
            .collect(),
    }
}

fn glyph_overhang(
    layout: &LineLayout,
    positions: &[f64],
    paint_bounds: &RasterPaintBounds<'_>,
    mut origin_for_index: impl FnMut(usize) -> f64,
) -> Range<f64> {
    let mut overhang = 0.0_f64..0.0_f64;
    let mut positions = positions.iter();
    for run in &layout.runs {
        let bounds = paint_bounds.for_font(run.font_id, layout.font_size);
        let min_x = bounds.ink.start;
        let max_x = bounds.ink.end;
        for (glyph, x) in run.glyphs.iter().zip(positions.by_ref()) {
            let x = x - origin_for_index(glyph.index);
            overhang.start = overhang.start.min(x + min_x);
            overhang.end = overhang.end.max(x + max_x);
        }
    }
    overhang
}

fn cluster_prefix(snapshot: &TabSnapshot, row: u32, column: u32) -> String {
    let len = snapshot.line_len(row);
    let column = clip_row_columns(snapshot, row, column..column).start;
    if column == 0 || column >= len {
        return String::new();
    }
    let mut context_len = 64;
    loop {
        let (mut start, chunks) = row_chunks(
            snapshot,
            row,
            column.saturating_sub(context_len)..column + 1,
        );
        let mut complete = start == 0;
        let mut position = start;
        let mut text = String::new();
        for chunk in chunks {
            let end = position + chunk.text.len() as u32;
            if chunk.renderer.is_some() {
                if position <= column && column <= end {
                    return String::new();
                }
                if column < position {
                    break;
                }
                text.clear();
                start = end;
                complete = true;
            } else {
                text.push_str(chunk.text);
            }
            position = end;
        }
        let offset = (column - start) as usize;
        if offset >= text.len() {
            return String::new();
        }
        let cluster_start = if complete {
            Some(
                text.grapheme_indices(true)
                    .take_while(|(start, _)| *start <= offset)
                    .last()
                    .map_or(0, |(start, _)| start),
            )
        } else {
            let mut cursor = GraphemeCursor::new(column as usize, len as usize, true);
            match cursor.is_boundary(&text, start as usize) {
                Ok(true) => return String::new(),
                Ok(false) => match cursor.prev_boundary(&text, start as usize) {
                    Ok(Some(boundary)) => Some(boundary - start as usize),
                    Ok(None) | Err(_) => None,
                },
                Err(_) => None,
            }
        };
        if let Some(cluster_start) = cluster_start {
            return text[cluster_start..offset].to_string();
        }
        context_len = context_len.saturating_mul(2);
    }
}

impl TextPiece {
    fn window(&self, columns: &Range<u32>, origin_x: ScrollPixelOffset) -> Option<PaintTextPiece> {
        let start = columns.start.max(self.columns.start);
        let end = columns.end.min(self.columns.end);
        if start >= end {
            return None;
        }
        let index = &self.index;
        let start = index.boundary(
            &self.layout,
            (start - self.columns.start) as usize,
            Bias::Left,
        );
        let end = index.boundary(
            &self.layout,
            (end - self.columns.start) as usize,
            Bias::Right,
        );
        let origin_x = origin_x - self.x;
        let layout = if start.0 == 0 && end.0 == self.layout.len && origin_x == 0. {
            self.layout.clone()
        } else {
            index.slice(&self.layout, start, end, origin_x)
        };
        Some(PaintTextPiece {
            columns: self.columns.start + start.0 as u32..self.columns.start + end.0 as u32,
            x: self.x + origin_x,
            text: if start.0 == 0 && end.0 == self.layout.len {
                self.text.clone()
            } else {
                SharedString::new(&self.text[start.0..end.0])
            },
            layout,
        })
    }

    fn paint_regions(
        &self,
        piece: usize,
        mut font_bounds: impl FnMut(FontId, Pixels) -> FontPaintBounds,
        regions: &mut Vec<PaintRegion>,
    ) {
        let font_bounds = self
            .layout
            .runs
            .iter()
            .map(|run| font_bounds(run.font_id, self.layout.font_size))
            .collect::<Vec<_>>();
        let mut glyphs = self
            .layout
            .runs
            .iter()
            .zip(&font_bounds)
            .flat_map(|(run, bounds)| run.glyphs.iter().map(move |_| bounds))
            .zip(self.index.positions.iter())
            .enumerate()
            .peekable();
        let mut ordered_start = 0;
        for partition in 0..self.index.partition_columns.len() {
            let start = self.index.partition(&self.layout, partition);
            let end = self.index.partition(&self.layout, partition + 1);
            let mut left = start.2.min(end.2);
            let mut right = start.2.max(end.2);
            let mut previous = start.2;
            let mut ordered =
                self.x.is_finite() && start.2.is_finite() && end.2.is_finite() && start.2 <= end.2;
            while let Some((_, (bounds, x))) = glyphs.next_if(|(index, _)| *index < end.1) {
                let font_left = bounds.ink.start;
                let font_width = bounds.decoration_width;
                let font_right = bounds.ink.end;
                ordered &= x.is_finite()
                    && previous <= *x
                    && *x <= end.2
                    && font_left.is_finite()
                    && font_width.is_finite()
                    && font_right.is_finite()
                    && font_width >= 0.;
                previous = *x;
                left = left.min(x + font_left).min(x - font_width / 2.);
                right = right.max(x + font_right);
            }
            if !ordered {
                self.ordered_paint_regions(piece, ordered_start..partition, &font_bounds, regions);
                regions.push(PaintRegion {
                    target: PaintTarget::Columns(
                        self.columns.start + start.0 as u32..self.columns.start + end.0 as u32,
                    ),
                    bounds: self.x + left..self.x + right,
                });
                ordered_start = partition + 1;
            }
        }
        self.ordered_paint_regions(
            piece,
            ordered_start..self.index.partition_columns.len(),
            &font_bounds,
            regions,
        );
    }

    fn ordered_paint_regions(
        &self,
        piece: usize,
        partitions: Range<usize>,
        font_bounds: &[FontPaintBounds],
        regions: &mut Vec<PaintRegion>,
    ) {
        if partitions.is_empty() {
            return;
        }
        let start = self.index.partition(&self.layout, partitions.start);
        let end = self.index.partition(&self.layout, partitions.end);
        regions.push(PaintRegion {
            bounds: self.x + start.2..self.x + end.2,
            target: PaintTarget::Advances { piece, partitions },
        });
        let first_run = self
            .index
            .runs
            .partition_point(|offset| *offset <= start.1)
            .saturating_sub(1);
        let last_run = self.index.runs.partition_point(|offset| *offset < end.1);
        for run in first_run..last_run {
            let glyphs = self.index.runs[run].max(start.1)..self.index.runs[run + 1].min(end.1);
            let positions = &self.index.positions[glyphs.clone()];
            let Some((&first, &last)) = positions.first().zip(positions.last()) else {
                continue;
            };
            let bounds = &font_bounds[run];
            let overhang =
                bounds.ink.start.min(-bounds.decoration_width / 2.).min(0.)..bounds.ink.end.max(0.);
            regions.push(PaintRegion {
                bounds: self.x + (first + overhang.start)..self.x + (last + overhang.end),
                target: PaintTarget::Glyphs {
                    piece,
                    glyphs,
                    overhang,
                },
            });
        }
    }

    fn split_large_text_dependencies(self, pieces: &mut Vec<Self>) {
        let mut previous = 0;
        let large_dependency = self
            .index
            .partition_columns
            .iter()
            .copied()
            .chain([self.layout.len as u32])
            .any(|end| {
                let large = (end - previous) as usize > MAX_LINE_LEN;
                previous = end;
                large
            });
        if !large_dependency || self.index.partition_columns.len() <= 1 {
            pieces.push(self);
            return;
        }
        let mut append_piece = |columns: Range<u32>| {
            let start = self
                .index
                .boundary(&self.layout, columns.start as usize, Bias::Left);
            let end = self
                .index
                .boundary(&self.layout, columns.end as usize, Bias::Right);
            let layout = self.index.slice(&self.layout, start, end, 0.);
            let positions = self.index.positions[start.1..end.1].to_vec();
            let index = LayoutIndex {
                start_x: start.2,
                ..LayoutIndex::new(&layout, positions, end.2)
            };
            pieces.push(Self {
                columns: self.columns.start + columns.start..self.columns.start + columns.end,
                x: self.x,
                source_start_column: self.source_start_column,
                text: SharedString::new(&self.text[start.0..end.0]),
                layout,
                index,
            });
        };
        let mut start = 0;
        let mut end = 0;
        for boundary in self
            .index
            .partition_columns
            .iter()
            .copied()
            .chain([self.layout.len as u32])
        {
            if (boundary - start) as usize > MAX_LINE_LEN && end > start {
                append_piece(start..end);
                start = end;
            }
            end = boundary;
        }
        append_piece(start..end);
    }
}

struct LayoutIndex {
    partition_columns: Arc<[u32]>,
    partition_glyphs: Option<Box<[usize]>>,
    runs: Box<[usize]>,

    positions: Arc<[f64]>,
    len: usize,
    start_x: f64,
    width: f64,
}

impl LayoutIndex {
    fn new(layout: &LineLayout, positions: Vec<f64>, width: f64) -> Self {
        let mut partition_columns = Vec::new();
        let mut partition_glyphs = None::<Vec<usize>>;
        for (column, offset) in [(0, 0)].into_iter().chain(glyph_partitions(layout)) {
            let column = column as u32;
            if partition_columns.last() == Some(&column) {
                continue;
            }
            if let Some(offsets) = &mut partition_glyphs {
                offsets.push(offset);
            } else if offset != partition_columns.len() {
                let mut offsets = (0..partition_columns.len()).collect::<Vec<_>>();
                offsets.push(offset);
                partition_glyphs = Some(offsets);
            }
            partition_columns.push(column);
        }
        let mut offset = 0;
        let runs = layout
            .runs
            .iter()
            .map(|run| {
                let start = offset;
                offset += run.glyphs.len();
                start
            })
            .chain([layout.runs.iter().map(|run| run.glyphs.len()).sum()])
            .collect();

        Self {
            partition_columns: Arc::from(partition_columns),
            partition_glyphs: partition_glyphs.map(Vec::into_boxed_slice),
            runs,
            positions: Arc::from(positions),
            len: layout.len,
            start_x: 0.,
            width,
        }
    }

    fn boundary(
        &self,
        layout: &LineLayout,
        column: usize,
        bias: Bias,
    ) -> (usize, usize, ScrollPixelOffset) {
        if column >= self.len {
            return (self.len, self.positions.len(), self.width);
        }
        let index = match bias {
            Bias::Left => self
                .partition_columns
                .partition_point(|start| *start as usize <= column)
                .saturating_sub(1),
            Bias::Right => self
                .partition_columns
                .partition_point(|start| (*start as usize) < column),
        };
        self.partition(layout, index)
    }

    fn partition(&self, layout: &LineLayout, index: usize) -> (usize, usize, ScrollPixelOffset) {
        let Some(column) = self.partition_columns.get(index) else {
            return (self.len, self.positions.len(), self.width);
        };
        let offset = self
            .partition_glyphs
            .as_ref()
            .map_or(index, |offsets| offsets[index]);
        let x = if !layout.caret_positions.is_empty() {
            layout.caret_x_for_index(*column as usize)
        } else if *column == 0 {
            self.start_x
        } else {
            self.positions.get(offset).copied().unwrap_or(self.width)
        };
        (*column as usize, offset, x)
    }

    fn partition_for_glyph(&self, glyph: usize) -> usize {
        self.partition_glyphs.as_ref().map_or(glyph, |offsets| {
            offsets
                .partition_point(|offset| *offset <= glyph)
                .saturating_sub(1)
        })
    }

    fn partition_point(
        &self,
        layout: &LineLayout,
        range: Range<usize>,
        mut predicate: impl FnMut(f64) -> bool,
    ) -> usize {
        let mut left = range.start;
        let mut right = range.end;
        while left < right {
            let middle = left + (right - left) / 2;
            if predicate(self.partition(layout, middle).2) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        left
    }

    fn slice(
        &self,
        layout: &LineLayout,
        start: (usize, usize, ScrollPixelOffset),
        end: (usize, usize, ScrollPixelOffset),
        origin_x: ScrollPixelOffset,
    ) -> Arc<LineLayout> {
        let first = self
            .runs
            .partition_point(|offset| *offset <= start.1)
            .saturating_sub(1);
        let last = self.runs.partition_point(|offset| *offset < end.1);
        let runs = layout
            .runs
            .iter()
            .enumerate()
            .skip(first)
            .take(last.saturating_sub(first))
            .map(|(index, run)| {
                let run_start = self.runs[index];
                let glyphs = run
                    .glyphs
                    .iter()
                    .zip(&self.positions[run_start..])
                    .skip(start.1.saturating_sub(run_start))
                    .take(end.1 - run_start.max(start.1))
                    .map(|(glyph, x)| ShapedGlyph {
                        index: glyph.index - start.0,
                        position: point(px((x - origin_x) as f32), glyph.position.y),
                        ..glyph.clone()
                    })
                    .collect();
                ShapedRun {
                    font_id: run.font_id,
                    glyphs,
                }
            })
            .collect();
        let mut caret_positions = layout.slice_caret_positions(start.0..end.0, origin_x);
        if let Some((_, x)) = caret_positions.first_mut() {
            *x = start.2 - origin_x;
        }
        Arc::new(LineLayout {
            font_size: layout.font_size,
            width: px((end.2 - origin_x) as f32),
            ascent: layout.ascent,
            descent: layout.descent,
            runs,
            len: end.0 - start.0,
            caret_positions,
        })
    }
}

struct LongRowScan {
    stack: Vec<Range<u32>>,
}

enum ScanStep {
    Row { row: u32 },
    Continue,
    Done,
}

impl LongRowScan {
    fn new(rows: Range<u32>) -> Self {
        Self { stack: vec![rows] }
    }

    fn step(&mut self, snapshot: &TabSnapshot) -> ScanStep {
        let Some(rows) = self.stack.pop() else {
            return ScanStep::Done;
        };
        if rows.is_empty() || snapshot.max_line_len_bound(rows.clone()) as usize <= MAX_LINE_LEN {
            return ScanStep::Continue;
        }
        if rows.len() == 1 {
            let len = snapshot.line_len(rows.start);
            return if len as usize > MAX_LINE_LEN {
                ScanStep::Row { row: rows.start }
            } else {
                ScanStep::Continue
            };
        }
        let middle = rows.start + (rows.end - rows.start) / 2;
        self.stack.push(middle..rows.end);
        self.stack.push(rows.start..middle);
        ScanStep::Continue
    }
}

fn atom_offsets_x(
    layout: &LineLayout,
    atoms: &[u32],
    index: &LayoutIndex,
) -> Vec<ScrollPixelOffset> {
    let mut left_edges = vec![None::<ScrollPixelOffset>; atoms.len()];
    let mut cursor = AtomCursor::new(atoms);
    let mut push = |column, x: f64| {
        let Some(atom) = cursor.at(column) else {
            return;
        };
        let left_edge = &mut left_edges[atom];
        *left_edge = Some(left_edge.map_or(x, |left_edge| left_edge.min(x)));
    };
    if layout.caret_positions.is_empty() {
        for (glyph, x) in layout
            .runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .zip(index.positions.iter())
        {
            push(glyph.index, *x);
        }
    } else {
        for (column, x) in &layout.caret_positions {
            if *column < layout.len {
                push(*column, *x);
            }
        }
    }
    let mut edges = left_edges
        .into_iter()
        .enumerate()
        .filter_map(|(atom, x)| x.map(|x| (atom, x)))
        .peekable();
    let mut positions = Vec::with_capacity(atoms.len());
    let (first, x) = edges.peek().copied().unwrap_or((atoms.len(), index.width));
    positions.resize(first, x);
    while let Some((atom, cluster_x)) = edges.next() {
        let (end, end_x) = edges.peek().copied().unwrap_or((atoms.len(), index.width));
        let parts = (end - atom) as f64;
        for rank in 0..end - atom {
            positions.push(cluster_x + (end_x - cluster_x) * rank as f64 / parts);
        }
    }
    positions
}

fn single_invisible(text: &str) -> Option<char> {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(character), None) if is_invisible(character) => Some(character),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AtomBoundary {
    Cluster,
    Interior,
}

pub(super) struct StandaloneInvisibles {
    atom: String,
    last: AtomBoundary,
}

impl Default for StandaloneInvisibles {
    fn default() -> Self {
        Self {
            atom: String::new(),
            last: AtomBoundary::Cluster,
        }
    }
}

impl StandaloneInvisibles {
    pub(super) fn for_columns(snapshot: &TabSnapshot, row: u32, columns: &Range<u32>) -> Self {
        let atom = cluster_prefix(snapshot, row, columns.start);
        let last = if cluster_prefix(snapshot, row, columns.end).is_empty() {
            AtomBoundary::Cluster
        } else {
            AtomBoundary::Interior
        };
        Self { atom, last }
    }

    pub(super) fn reset(&mut self) {
        self.atom.clear();
    }

    pub(super) fn find(
        &mut self,
        text: &str,
        next: Option<char>,
        is_last: bool,
    ) -> SmallVec<[Range<usize>; 1]> {
        if text.len() > 1 && text.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            self.atom.clear();
            self.atom.push_str(&text[text.len() - 1..]);
            return SmallVec::new();
        }
        let mut found = SmallVec::new();
        let mut candidate = None::<Range<usize>>;
        let mut confirm = |candidate: &mut Option<Range<usize>>, boundary: Option<AtomBoundary>| {
            if let Some(range) = candidate.take()
                && boundary == Some(AtomBoundary::Cluster)
            {
                found.push(range);
            }
        };
        let ascii_suffix = text.bytes().rev().take_while(u8::is_ascii).count();
        let general_end = (text.len() - ascii_suffix + 1).min(text.len());
        for (offset, character) in text[..general_end].char_indices() {
            let boundary = self.push(character);
            confirm(&mut candidate, boundary);
            if boundary == Some(AtomBoundary::Cluster) && is_invisible(character) {
                candidate = Some(offset..offset + character.len_utf8());
            }
        }
        let bytes = text.as_bytes();
        let mut last_atom_start = None;
        for offset in general_end..text.len() {
            let boundary = (!(bytes[offset - 1] == b'\r' && bytes[offset] == b'\n'))
                .then_some(AtomBoundary::Cluster);
            confirm(&mut candidate, boundary);
            if boundary.is_some() {
                last_atom_start = Some(offset);
                if is_invisible(char::from(bytes[offset])) {
                    candidate = Some(offset..offset + 1);
                }
            }
        }
        match last_atom_start {
            Some(start) => {
                self.atom.clear();
                self.atom.push_str(&text[start..]);
            }
            None => self.atom.push_str(&text[general_end..]),
        }
        if candidate.is_some() {
            let boundary = match next {
                Some(next) => self.would_start_atom(next),
                None if is_last => Some(self.last),
                None => Some(AtomBoundary::Cluster),
            };
            confirm(&mut candidate, boundary);
        }
        found
    }

    fn push(&mut self, character: char) -> Option<AtomBoundary> {
        let len = self.atom.len();
        self.atom.push(character);
        if len == 0 {
            return Some(AtomBoundary::Cluster);
        }
        let boundary = is_cluster_boundary(&self.atom, len).then_some(AtomBoundary::Cluster);
        if boundary.is_some() {
            self.atom.clear();
            self.atom.push(character);
        }
        boundary
    }

    fn would_start_atom(&mut self, character: char) -> Option<AtomBoundary> {
        let len = self.atom.len();
        if len == 0 {
            return Some(AtomBoundary::Cluster);
        }
        self.atom.push(character);
        let boundary = is_cluster_boundary(&self.atom, len).then_some(AtomBoundary::Cluster);
        self.atom.truncate(len);
        boundary
    }
}

fn is_cluster_boundary(text: &str, offset: usize) -> bool {
    let bytes = text.as_bytes();
    if let (Some(&previous), Some(&next)) = (bytes.get(offset.wrapping_sub(1)), bytes.get(offset))
        && previous.is_ascii()
        && next.is_ascii()
    {
        return !(previous == b'\r' && next == b'\n');
    }
    GraphemeCursor::new(offset, text.len(), true)
        .is_boundary(text, 0)
        .unwrap_or(false)
}

pub(super) fn row_grapheme(snapshot: &TabSnapshot, row: u32, column: u32) -> String {
    let len = snapshot.line_len(row);
    let column = clip_row_columns(snapshot, row, column..column).start;
    let prefix = cluster_prefix(snapshot, row, column);
    let start = column.saturating_sub(prefix.len() as u32);
    let mut end = column.saturating_add(64).min(len);
    loop {
        let text = row_chunks(snapshot, row, start..end)
            .1
            .map(|chunk| chunk.text)
            .collect::<String>();
        let mut graphemes = text.graphemes(true);
        let Some(grapheme) = graphemes.next() else {
            return String::new();
        };
        if graphemes.next().is_some() || end == len {
            return grapheme.to_string();
        }
        end = start
            .saturating_add((end - start).saturating_mul(2))
            .min(len);
    }
}

pub(super) fn clip_row_columns(
    snapshot: &TabSnapshot,
    row: u32,
    columns: Range<u32>,
) -> Range<u32> {
    let len = snapshot.line_len(row);
    let clip = |column: u32, bias: Bias, fallback: u32| {
        let column = column.min(len);
        let (_, _, to_next_tab_stop) =
            snapshot.tab_point_to_fold_point(TabPoint::new(row, column), Bias::Left);
        if to_next_tab_stop > 0 {
            column
        } else {
            clip_column(snapshot, row, column, bias).unwrap_or(fallback)
        }
    };
    let start = clip(columns.start, Bias::Left, 0);
    let end = clip(columns.end, Bias::Right, len);
    start..end.max(start)
}

fn row_chunks(snapshot: &TabSnapshot, row: u32, columns: Range<u32>) -> (u32, TabChunks<'_>) {
    let columns = clip_row_columns(snapshot, row, columns);
    let chunks = snapshot.chunks(
        TabPoint::new(row, columns.start)..TabPoint::new(row, columns.end),
        LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: false,
        },
        Highlights::default(),
    );
    (columns.start, chunks)
}

fn clip_column(snapshot: &TabSnapshot, row: u32, column: u32, bias: Bias) -> Option<u32> {
    let fold_point = snapshot
        .tab_point_to_fold_point(TabPoint::new(row, column), bias)
        .0;
    let clipped = snapshot.fold_point_to_tab_point(
        snapshot
            .fold_snapshot
            .clip_point_to_char_boundary(fold_point, bias),
    );
    (clipped.row() == row).then_some(clipped.column())
}

fn glyph_partitions(layout: &LineLayout) -> impl Iterator<Item = (usize, usize)> {
    let indices = layout
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .map(|glyph| glyph.index);
    let mut suffix_min = if indices.clone().is_sorted() {
        Vec::new()
    } else {
        indices.clone().collect::<Vec<_>>()
    };
    let mut minimum = usize::MAX;
    for index in suffix_min.iter_mut().rev() {
        minimum = minimum.min(*index);
        *index = minimum;
    }
    let mut suffix_min = suffix_min.into_iter();
    let mut prefix_max = None::<usize>;
    indices.enumerate().filter_map(move |(position, index)| {
        let minimum = suffix_min.next().unwrap_or(index);
        let partition = prefix_max
            .is_none_or(|max| max < minimum)
            .then_some((minimum, position));
        prefix_max = Some(prefix_max.map_or(index, |max| max.max(index)));
        partition
    })
}

fn slice_font_runs(
    runs: impl IntoIterator<Item = FontRun>,
    start: u32,
    columns: Range<u32>,
) -> SmallVec<[FontRun; 1]> {
    let mut sliced = SmallVec::new();
    let mut run_start = start;
    for run in runs {
        let run_end = run_start + run.len as u32;
        let sliced_start = run_start.max(columns.start);
        let sliced_end = run_end.min(columns.end);
        if sliced_start < sliced_end {
            sliced.push(FontRun {
                len: (sliced_end - sliced_start) as usize,
                font_id: run.font_id,
            });
        }
        run_start = run_end;
        if run_start >= columns.end {
            break;
        }
    }
    sliced
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Editor, EditorMode, EditorStyle,
        display_map::{
            ChunkRenderer, ChunkReplacement, Crease, DisplayMap, DisplayRow, DisplaySnapshot,
            FoldPlaceholder, HighlightKey, HighlightedChunk, RendererWidthsKey, SyntaxStyling,
            fold_map::FoldMap, inlay_map::InlayMap, tab_map::TabMap,
        },
        element::{EditorElement, LineChunk},
        inlays::Inlay,
        test::{ShapingTextSystem, test_font},
    };
    use buffer_diff::BufferDiff;
    use clock::{Lamport, ReplicaId};
    use futures::{FutureExt, StreamExt};
    use gpui::{
        AbsoluteLength, App, AppContext as _, BorrowAppContext as _, Bounds, CompactGlyphPositions,
        Context, DevicePixels, Entity, Font, FontMetrics, FontStyle, FontWeight, GlyphId,
        HighlightStyle, Hsla, IntoElement, PlatformTextSystem, Render, RenderGlyphParams,
        ShapedLine, Size, StrikethroughStyle, Styled, TestAppContext, TextRenderingMode, TextRun,
        TextStyle, UnderlineStyle, Window, canvas, rgba, size,
    };
    use language::{
        Buffer, CursorShape, Diagnostic, DiagnosticEntry, DiagnosticSet, Operation, Point,
    };
    use lsp::LanguageServerId;
    use multi_buffer::{MultiBuffer, MultiBufferOffset, PathKey};
    use project::{InlayId, project_settings::DiagnosticSeverity};
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{
        borrow::Cow,
        num::NonZeroU32,
        ops::{ControlFlow, RangeInclusive},
        rc::Rc,
        slice,
    };
    use text::{PointUtf16, Selection, SelectionGoal};
    use theme::LoadThemes;

    const FONT_SIZE: gpui::Pixels = px(14.);

    #[gpui::test(iterations = 10)]
    async fn test_random_long_row_edits(cx: &mut TestAppContext, mut rng: StdRng) {
        cx.background_executor.set_block_on_ticks(0..=50);
        let text = (0..4)
            .map(|_| random_text(&mut rng, 200..1_200))
            .collect::<Vec<_>>()
            .join("\n");
        let (buffer, map) = build_display_map(&text, 4, cx);
        let mut wrapper = cx.update(|cx| cx.text_system().line_wrapper(test_font(), FONT_SIZE));

        for _ in 0..10 {
            for _ in 0..rng.random_range(1..=3) {
                randomly_mutate(&buffer, &map, &mut rng, cx);
                let snapshot = ready_snapshot(&map, cx);
                assert_round_trips(&snapshot);
            }
            cx.run_until_parked();
            let snapshot = ready_snapshot(&map, cx);
            assert_matches_char_widths(&snapshot, &mut wrapper);
        }
    }

    #[gpui::test]
    async fn test_long_inlays_at_buffer_edges(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map("x", 4, cx);
        let mut wrapper = cx.update(|cx| cx.text_system().line_wrapper(test_font(), FONT_SIZE));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let inlays = [
            Inlay::mock_hint(
                0,
                buffer_snapshot.anchor_before(MultiBufferOffset(0)),
                "漢".repeat(200),
            ),
            Inlay::mock_hint(
                1,
                buffer_snapshot.anchor_after(MultiBufferOffset(1)),
                "é".repeat(1_000),
            ),
        ];
        for inlay in inlays {
            map.update(cx, |map, cx| map.splice_inlays(&[], vec![inlay], cx));
            cx.run_until_parked();
            let snapshot = ready_snapshot(&map, cx);
            assert_round_trips(&snapshot);
            assert_matches_char_widths(&snapshot, &mut wrapper);
        }
    }

    #[gpui::test]
    async fn test_tab_expanded_long_rows(cx: &mut TestAppContext) {
        let text = format!("{}{}", "\t".repeat(129), "é".repeat(50));
        let (_, map) = build_display_map(&text, 8, cx);
        let mut wrapper = cx.update(|cx| cx.text_system().line_wrapper(test_font(), FONT_SIZE));
        cx.run_until_parked();
        let snapshot = ready_snapshot(&map, cx);
        assert_eq!(snapshot.line_len(DisplayRow(0)), 129 * 8 + 50 * 2);
        assert_matches_char_widths(&snapshot, &mut wrapper);
    }

    #[gpui::test]
    async fn test_long_rows_keep_grapheme_clusters_across_splices(cx: &mut TestAppContext) {
        let joined = (0..150)
            .map(|index| {
                let letter = char::from(b'a' + (index % 26) as u8);
                format!("{letter}\u{200d}\u{1f600}b\u{1f1fa}\u{1f1f8}e\u{301}")
            })
            .collect::<String>();
        let flags =
            |pairs: usize| format!("{}{}", "\u{1f1e8}\u{1f1e6}".repeat(pairs), "ab".repeat(600));
        for (text, joins_seam) in [(joined, true), (flags(40), false), (flags(300), false)] {
            let (buffer, map) = build_display_map(&text, 4, cx);
            cx.run_until_parked();
            let snapshot = ready_snapshot(&map, cx);
            assert_cluster_atoms(&snapshot);

            let (edited, inserted) = if joins_seam {
                let seam = text.find('\u{1f600}').expect("an emoji");
                let joiner_start = seam - "a\u{200d}".len();
                (joiner_start..joiner_start + 1, "\u{1f600}")
            } else {
                (0..0, "\u{1f1e6}")
            };
            buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(
                        MultiBufferOffset(edited.start)..MultiBufferOffset(edited.end),
                        inserted,
                    )],
                    None,
                    cx,
                )
            });
            let snapshot = ready_snapshot(&map, cx);
            let long_row = snapshot
                .long_row(DisplayRow(0))
                .expect("row geometry ready")
                .expect("a long row");
            assert_eq!(
                long_row.len() as usize,
                text.len() - edited.len() + inserted.len()
            );
            assert_cluster_atoms(&snapshot);
            cx.run_until_parked();
            assert_cluster_atoms(&ready_snapshot(&map, cx));
        }
    }

    #[gpui::test]
    async fn test_long_rows_use_measured_inlay_widths(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map(&"x".repeat(2_000), 4, cx);
        let position = buffer.read_with(cx, |buffer, cx| {
            buffer.snapshot(cx).anchor_after(MultiBufferOffset(10))
        });
        let inlay = Inlay::color(0, position, rgba(0xff0000ff));
        map.update(cx, |map, cx| map.splice_inlays(&[], vec![inlay], cx));
        cx.run_until_parked();
        let inlay_width = |map: &Entity<DisplayMap>, cx: &mut TestAppContext| {
            let snapshot = ready_snapshot(&map, cx);
            let long_row = snapshot
                .long_row(DisplayRow(0))
                .expect("row geometry ready")
                .expect("a long row");
            long_row.x_for_column(10 + "\u{25fc}".len() as u32) - long_row.x_for_column(10)
        };
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let pending = snapshot
            .long_row(DisplayRow(0))
            .err()
            .expect("unmeasured renderer");
        map.update(cx, |map, cx| {
            map.request_row_geometry(pending.clone(), cx);
            assert_eq!(map.request_max_long_row_width(&snapshot, cx), None);
        });
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_none());
        assert_eq!(
            snapshot
                .wrap_snapshot()
                .long_rows
                .rows
                .lock()
                .iter()
                .count(),
            0
        );
        assert_eq!(
            snapshot.wrap_snapshot().long_rows.completed_max_width.get(),
            None
        );
        for _ in 0..2 {
            map.update(cx, |map, cx| {
                map.request_row_geometry(pending.clone(), cx);
                map.request_viewport_row_geometry([pending.clone()], cx);
                assert_eq!(map.request_max_long_row_width(&snapshot, cx), None);
                assert!(!map.long_row_widths.requested);
            });
            cx.run_until_parked();
        }

        let key = |font_size: f32| RendererWidthsKey {
            rem_size: px(16.),
            font: test_font(),
            font_size: px(font_size),
            buffer_font: test_font(),
            editor_width: px(500.),
            line_height: px(20.),
        };
        let renderer_widths = [(ChunkRendererId::Inlay(InlayId::Color(0)), px(37.))];
        assert!(map.update(cx, |map, cx| {
            map.update_renderer_widths(key(14.), renderer_widths, cx)
        }));
        assert!(pending.wait().now_or_never().is_some());
        assert_eq!(inlay_width(&map, cx), 37.);
        assert!(!map.update(cx, |map, cx| {
            map.update_renderer_widths(key(14.), renderer_widths, cx)
        }));

        assert!(map.update(cx, |map, cx| {
            map.update_renderer_widths(key(28.), [], cx)
        }));
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let pending = snapshot
            .long_row(DisplayRow(0))
            .err()
            .expect("invalidated measurement");
        map.update(cx, |map, cx| map.request_row_geometry(pending.clone(), cx));
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_none());
        assert_eq!(
            snapshot.wrap_snapshot().long_rows.completed_max_width.get(),
            None
        );
        assert!(map.update(cx, |map, cx| {
            map.update_renderer_widths(
                key(28.),
                [(ChunkRendererId::Inlay(InlayId::Color(0)), px(51.))],
                cx,
            )
        }));
        assert!(pending.wait().now_or_never().is_some());
        assert_eq!(inlay_width(&map, cx), 51.);
    }

    #[gpui::test]
    async fn test_soft_wrap_counts_renderer_width_once(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map("a", 4, cx);
        let position = buffer.read_with(cx, |buffer, cx| {
            buffer.snapshot(cx).anchor_after(MultiBufferOffset(1))
        });
        let inlay = Inlay::mock_repl_result(0, position, &"r".repeat(300));
        map.update(cx, |map, cx| {
            map.splice_inlays(&[], vec![inlay], cx);
            map.set_wrap_width(Some(px(150.)), cx);
        });
        cx.run_until_parked();
        map.update(cx, |map, cx| {
            map.update_renderer_widths(
                RendererWidthsKey {
                    rem_size: px(16.),
                    font: test_font(),
                    font_size: FONT_SIZE,
                    buffer_font: test_font(),
                    editor_width: px(500.),
                    line_height: px(20.),
                },
                [(ChunkRendererId::Inlay(InlayId::ReplResult(0)), px(100.))],
                cx,
            )
        });
        cx.run_until_parked();
        let snapshot = ready_snapshot(&map, cx);
        assert_eq!(snapshot.max_point().row(), DisplayRow(0));
    }

    #[gpui::test]
    async fn test_long_rows_replace_only_standalone_invisibles(cx: &mut TestAppContext) {
        let mut scanner = StandaloneInvisibles {
            atom: "\u{d4e}".to_string(),
            last: AtomBoundary::Interior,
        };
        for (text, atom) in [
            ("", "\u{d4e}"),
            ("a", "\u{d4e}a"),
            ("\u{301}", "\u{d4e}a\u{301}"),
            ("\u{d4e}", "\u{d4e}"),
            (" ~", "~"),
            ("\u{301}", "~\u{301}"),
        ] {
            assert!(scanner.find(text, Some('\u{301}'), false).is_empty());
            assert_eq!(scanner.atom, atom);
        }
        assert!(scanner.find("\u{200b}", None, true).is_empty());
        scanner.last = AtomBoundary::Cluster;
        assert_eq!(scanner.find("\u{200b}", None, true).as_slice(), &[0..3]);
        assert_eq!(
            scanner.find("\x1f\x7f", None, true).as_slice(),
            &[0..1, 1..2]
        );

        let text = format!(
            "a{}\u{200c}b\u{200b}{}ab\u{200c}c a\u{200b}b",
            "\u{301}".repeat(15),
            "x".repeat(1_100)
        );
        let (buffer, map) = build_display_map(&text, 4, cx);
        let capped_joiner = text.find('\u{200c}').unwrap_or_default() as u32;
        let attached_joiner = text.rfind('\u{200c}').unwrap_or_default();
        let highlighted = text.rfind("ab").unwrap_or_default()..attached_joiner;
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        map.update(cx, |map, cx| {
            map.highlight_text(
                HighlightKey::Editor,
                vec![
                    buffer_snapshot.anchor_before(MultiBufferOffset(highlighted.start))
                        ..buffer_snapshot.anchor_after(MultiBufferOffset(highlighted.end)),
                ],
                HighlightStyle::color(gpui::red()),
                false,
                cx,
            )
        });
        cx.run_until_parked();
        let snapshot = ready_snapshot(&map, cx);
        let style = EditorStyle::default();
        let replaced = snapshot
            .highlighted_chunks(
                DisplayRow(0)..DisplayRow(1),
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                &style,
            )
            .filter(|chunk| matches!(chunk.replacement, Some(ChunkReplacement::Str(_))))
            .map(|chunk| chunk.text.to_string())
            .collect::<Vec<_>>();
        assert_eq!(replaced, ["\u{200b}", "\u{200b}"]);

        let long_row = snapshot
            .long_row(DisplayRow(0))
            .expect("row geometry ready")
            .expect("a long row");
        let attached_joiner = attached_joiner as u32;
        assert_eq!(
            long_row.atom_range(attached_joiner).start,
            attached_joiner - 1
        );
        assert_eq!(long_row.atom_range(capped_joiner), 0..capped_joiner + 3);
        for (column, expected) in [
            (1, "a"),
            (capped_joiner, &text[..capped_joiner as usize]),
            (capped_joiner + 3, ""),
            (attached_joiner, "b"),
        ] {
            assert_eq!(cluster_prefix(snapshot.tab_snapshot(), 0, column), expected);
        }
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: false,
        };
        for columns in [0..long_row.len(), capped_joiner..long_row.len()] {
            let replaced = snapshot
                .highlighted_chunks_in_row(DisplayRow(0), columns.clone(), language_aware, &style)
                .filter(|chunk| matches!(chunk.replacement, Some(ChunkReplacement::Str(_))))
                .map(|chunk| chunk.text.to_string())
                .collect::<Vec<_>>();
            assert_eq!(replaced, ["\u{200b}", "\u{200b}"], "{columns:?}");
        }
    }

    #[gpui::test]
    fn test_cluster_prefix_recovers_unbounded_context_without_shaping(cx: &mut TestAppContext) {
        for clusters in [
            vec![format!("a{}", "\u{301}".repeat(256))],
            vec!["🇺🇸".to_string(); 34],
            vec![format!("{}🇺🇸", "\u{d4e}".repeat(256)), "🇺🇸".to_string()],
            vec![format!("👩{}\u{200d}💻", "\u{301}".repeat(256))],
            vec![format!("{}क", "क्".repeat(128))],
        ] {
            let text = format!(
                "{}{}{}",
                "x".repeat(128),
                clusters.concat(),
                "z".repeat(MAX_LINE_LEN)
            );
            let (_, map) = build_display_map(&text, 4, cx);
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            let widths = snapshot
                .wrap_snapshot()
                .long_rows
                .widths
                .as_ref()
                .expect("metrics");
            assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
            let mut start = 128;
            for cluster in clusters {
                for offset in [
                    0,
                    1,
                    4,
                    63,
                    64,
                    65,
                    cluster.len() / 2,
                    cluster.len() - 1,
                    cluster.len(),
                ] {
                    let offset = offset.min(cluster.len());
                    let expected = if offset == cluster.len() {
                        ""
                    } else {
                        &cluster[..cluster.floor_char_boundary(offset)]
                    };
                    assert_eq!(
                        cluster_prefix(snapshot.tab_snapshot(), 0, (start + offset) as u32),
                        expected,
                        "cluster {cluster:?}, offset {offset}"
                    );
                    if offset < cluster.len() {
                        assert_eq!(
                            row_grapheme(snapshot.tab_snapshot(), 0, (start + offset) as u32),
                            cluster
                        );
                    }
                }
                start += cluster.len();
            }
            for column in [0, 63, 64, 65, start, text.len() - 1, text.len()] {
                assert_eq!(
                    cluster_prefix(snapshot.tab_snapshot(), 0, column as u32),
                    ""
                );
            }
            assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        }
    }

    #[test]
    fn test_measured_dense_unicode_paints_visible_glyphs() {
        let mut cx = ignored_marks_context();
        crate::editor_tests::init_test(&mut cx, |_| {});
        for (character, marks, count) in [('a', 524_288, 1), ('!', 8_192, 65)] {
            let text = format!(
                "{}END",
                format!("{character}{}", "\u{34f}".repeat(marks)).repeat(count)
            );
            let expected = (0..count)
                .map(|index| (index * (2 * marks + 1), character))
                .chain(
                    "END"
                        .char_indices()
                        .map(|(index, character)| (text.len() - 3 + index, character)),
                )
                .collect::<Vec<_>>();
            let cx = cx.add_empty_window();
            let editor = cx.update(|window, cx| {
                let buffer = MultiBuffer::build_simple(&text, cx);
                cx.new(|cx| Editor::new(EditorMode::full(), buffer, None, window, cx))
            });
            editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
            });
            let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
            style.text.font_size = gpui::AbsoluteLength::from(px(10.));
            crate::element::tests::assert_long_row_paints(&editor, &style, &expected, cx);
        }
    }

    #[gpui::test]
    async fn test_long_rows_keep_flag_pairs_after_long_prepend_clusters(cx: &mut TestAppContext) {
        let flags = "\u{1f1fa}\u{1f1f8}".repeat(4);
        for prepend_count in [339, 340, 341] {
            let text = format!("{}{flags}{}", "x".repeat(1_100), "z".repeat(200));
            let (buffer, map) = build_display_map(&text, 4, cx);
            cx.run_until_parked();

            let prepend = "\u{d4e}".repeat(prepend_count);
            buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(1_100)..MultiBufferOffset(1_100), prepend)],
                    None,
                    cx,
                )
            });
            cx.run_until_parked();
            let snapshot = ready_snapshot(&map, cx);
            let long_row = snapshot
                .long_row(DisplayRow(0))
                .expect("row geometry ready")
                .expect("a long row");
            let flags_start = 1_100 + prepend_count as u32 * "\u{d4e}".len() as u32;
            let atom_starts = long_row
                .geometry()
                .atoms
                .columns
                .iter()
                .copied()
                .filter(|start| flags_start <= *start && *start <= flags_start + flags.len() as u32)
                .map(|start| start - flags_start)
                .collect::<Vec<_>>();
            assert_eq!(atom_starts, [8, 16, 24, 32], "{prepend_count} prepends");
        }
    }

    #[gpui::test]
    async fn test_long_rows_after_whole_buffer_edits(cx: &mut TestAppContext) {
        let text = format!(
            "{}{}\n{}",
            "\ta\n".repeat(50),
            "\tx".repeat(800),
            "\tb\n".repeat(50)
        );
        let (_, map) = build_display_map(&text, 4, cx);
        let mut wrapper = cx.update(|cx| cx.text_system().line_wrapper(test_font(), FONT_SIZE));
        cx.run_until_parked();
        assert_matches_char_widths(&ready_snapshot(&map, cx), &mut wrapper);

        set_tab_size(8, cx);
        let snapshot = ready_snapshot(&map, cx);
        assert_round_trips(&snapshot);
        cx.run_until_parked();
        let snapshot = ready_snapshot(&map, cx);
        assert!(
            snapshot
                .long_row(DisplayRow(50))
                .expect("row geometry ready")
                .is_some()
        );
        assert_matches_char_widths(&snapshot, &mut wrapper);
    }

    #[gpui::test]
    fn test_row_input_uses_shifted_singleton_excerpt(cx: &mut TestAppContext) {
        let expected = "b".repeat(2_048);
        let (buffer, map) = build_display_map(&format!("{}\n{expected}", "a".repeat(2_048)), 4, cx);
        buffer.update(cx, |multi_buffer, cx| {
            let buffer = multi_buffer.as_singleton().expect("singleton buffer");
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(0),
                buffer,
                [Point::new(1, 0)..Point::new(1, 2_048)],
                0,
                cx,
            );
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let input = RowInput::new(snapshot.tab_snapshot(), 0).expect("measured row input");
        let [InputRun::Text { start, text }] = input.runs.as_slice() else {
            panic!("expected one text run");
        };
        assert_eq!((*start, text.as_str()), (0, expected.as_str()));
    }

    #[test]
    fn test_folded_buffers_do_not_widen_long_rows() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        init_settings(4, cx);
        let long_text = "x".repeat(2_000);
        let buffer = cx.update(|cx| {
            MultiBuffer::build_multi(
                [
                    (
                        long_text.as_str(),
                        vec![Point::new(0, 0)..Point::new(0, 2_000)],
                    ),
                    ("short", vec![Point::new(0, 0)..Point::new(0, 5)]),
                ],
                cx,
            )
        });
        let map = new_display_map(&buffer, cx);
        cx.run_until_parked();

        let long_buffer_id = buffer.read_with(cx, |buffer, cx| {
            buffer
                .snapshot(cx)
                .excerpts()
                .next()
                .map(|excerpt| excerpt.context.start.buffer_id)
                .expect("an excerpt")
        });
        map.update(cx, |map, cx| map.fold_buffers([long_buffer_id], cx));
        let hidden = map.update(cx, |map, cx| map.snapshot(cx));
        let long_rows = &hidden.wrap_snapshot().long_rows;
        assert_eq!(long_rows.rows.lock().iter().count(), 0);
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&hidden, cx)),
            Some(0.)
        );
        cx.run_until_parked();
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&hidden, cx)),
            Some(0.)
        );
        assert_eq!(long_rows.rows.lock().iter().count(), 0);

        map.update(cx, |map, cx| map.unfold_buffers([long_buffer_id], cx));
        let shown = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(shown.tab_snapshot().version, hidden.tab_snapshot().version);
        assert!(shown.long_row_width_key() != hidden.long_row_width_key());
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&shown, cx)),
            None
        );
        cx.run_until_parked();
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&shown, cx)),
            Some(2_000. * 7.)
        );
        assert_eq!(
            long_rows.try_max_width(
                hidden.tab_snapshot(),
                &hidden.block_snapshot.replaced_wrap_rows(),
            ),
            Some(0.)
        );
        map.update(cx, |map, cx| {
            map.fold_buffers([long_buffer_id], cx);
            let hidden = map.snapshot(cx);
            assert_eq!(map.request_max_long_row_width(&hidden, cx), Some(0.));
        });
    }

    #[test]
    fn test_unrepresentable_rows_cancel_geometry_work_and_recover() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let text = format!("{}{}", "\t".repeat(240), "x".repeat(2_047));
        let (buffer, map) = build_display_map(&text, 128, cx);
        map.update(cx, |map, cx| {
            let mut previous = map.snapshot(cx);
            for limit in [32_767, 32_768, 32_767] {
                let pending = previous.pending_row(DisplayRow(0), false);
                map.set_tab_column_limit(limit, cx);
                assert!(pending.wait().now_or_never().is_some());
                let current = map.snapshot(cx);
                assert_eq!(
                    current.tab_snapshot().version,
                    previous.tab_snapshot().version + 1
                );
                assert!(!pending.is_current(&current));
                previous = current;
            }
        });
        set_test_shaping_style(&map, cx);
        let original = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(!original.tab_snapshot().has_unrepresentable_rows());
        assert_eq!(original.tab_snapshot().line_len(0), 32_767);
        let widths = original
            .wrap_snapshot()
            .long_rows
            .widths
            .as_ref()
            .expect("metrics");
        let lease = map.read_with(cx, |map, _| map.retain_row_geometry());
        let geometry = original.pending_row(DisplayRow(0), false);
        let paint = original.pending_row(DisplayRow(0), true);
        let renderer = ChunkRenderer {
            id: ChunkRendererId::Inlay(InlayId::Hint(0)),
            render: Arc::new(|_| panic!("constraint preparation must not render")),
            constrain_width: true,
            measured_width: None,
            width_class: None,
        };
        let constraint = original.pending_renderer_constraint(DisplayRow(0), renderer.id);
        let (resume, pause) = oneshot::channel();
        let (cancelled, priority) = map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            map.request_row_geometry(geometry.clone(), cx);
            map.request_row_geometry(paint.clone(), cx);
            assert!(
                map.request_renderer_constraint(
                    renderer.clone(),
                    SharedString::from("xy"),
                    test_font(),
                    FONT_SIZE,
                    cx,
                )
                .is_none()
            );
            assert_eq!(map.request_max_long_row_width(&original, cx), None);
            (
                map.long_row_widths.cancelled.clone(),
                map.long_row_widths.priority.clone(),
            )
        });
        cx.run_until_parked();
        assert_eq!(priority.lock().input.len(), 2);
        assert_eq!(lease.rows.lock().len(), 1);
        for pending in [&geometry, &paint, &constraint] {
            assert!(pending.wait().now_or_never().is_none());
        }
        buffer.update(cx, |buffer, cx| {
            let end = Point::new(0, text.len() as u32);
            buffer.edit([(end..end, "x")], None, cx);
        });
        let invalid = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(invalid.tab_snapshot().has_unrepresentable_rows());
        assert!(cancelled.load(atomic::Ordering::Acquire));
        assert!(priority.lock().input.is_empty());
        assert!(lease.rows.lock().is_empty());
        for pending in [&geometry, &paint, &constraint] {
            assert!(!pending.is_current(&invalid));
            assert!(pending.wait().now_or_never().is_some());
        }
        let pending = [
            invalid.pending_row(DisplayRow(0), false),
            invalid.pending_row(DisplayRow(0), true),
            invalid.pending_renderer_constraint(DisplayRow(0), renderer.id),
        ];
        map.update(cx, |map, cx| {
            for _ in 0..2 {
                assert!(Arc::ptr_eq(
                    &invalid.row_readiness,
                    &map.snapshot(cx).row_readiness
                ));
                for pending in [&geometry, &paint, &constraint].into_iter().chain(&pending) {
                    map.request_row_geometry(pending.clone(), cx);
                    assert!(pending.wait().now_or_never().is_some());
                }
                map.request_viewport_row_geometry(pending.clone(), cx);
                assert!(
                    map.request_renderer_constraint(
                        renderer.clone(),
                        SharedString::from("xy"),
                        test_font(),
                        FONT_SIZE,
                        cx,
                    )
                    .is_none()
                );
                assert_eq!(map.request_max_long_row_width(&invalid, cx), None);
                assert_eq!(map.request_max_long_row_width(&original, cx), None);
                map.start_long_row_worker(&invalid, cx);
                map.start_long_row_worker(&original, cx);
                assert!(!map.long_row_widths.requested);
                assert!(
                    !map.long_row_widths
                        .measure_width
                        .load(atomic::Ordering::Acquire)
                );
                assert!(map.long_row_widths.pending.lock().is_none());
                assert!(map.long_row_widths.viewport.is_empty());
                assert!(map.long_row_widths.readiness.constraints.lock().is_empty());
                let priority = map.long_row_widths.priority.lock();
                assert!(priority.input.is_empty());
                assert!(priority.visible.is_empty());
                assert!(priority.constraints.is_empty());
            }
        });
        resume.send(()).expect("cancelled worker");
        cx.run_until_parked();
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        assert_eq!(widths.highlighted_rows.load(atomic::Ordering::Relaxed), 0);
        set_tab_size(1, cx);
        let restored = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(!restored.tab_snapshot().has_unrepresentable_rows());
        assert!(!pending[0].is_current(&restored));
        let geometry = restored
            .long_row(DisplayRow(0))
            .err()
            .expect("pending geometry");
        map.update(cx, |map, cx| map.request_row_geometry(geometry.clone(), cx));
        cx.run_until_parked();
        assert!(geometry.wait().now_or_never().is_some());
        assert_eq!(
            restored
                .long_row(DisplayRow(0))
                .expect("ready geometry")
                .expect("long row")
                .width(),
            2_288. * 7.,
        );
    }

    #[test]
    fn test_async_width_pending_supersession_and_teardown() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let (buffer, map) = build_display_map(
            &format!("{}\n{}", "x".repeat(2_048), "x".repeat(1_536)),
            4,
            cx,
        );
        set_test_shaping_style(&map, cx);
        let original = map.update(cx, |map, cx| map.snapshot(cx));
        let widths = original
            .wrap_snapshot()
            .long_rows
            .widths
            .as_ref()
            .expect("metrics");
        let original_pending = original.long_row(DisplayRow(0)).err().expect("pending row");
        let (resume, pause) = futures::channel::oneshot::channel();
        let original_cancelled = map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            map.request_row_geometry(original_pending.clone(), cx);
            assert_eq!(map.request_max_long_row_width(&original, cx), None);
            map.long_row_widths.cancelled.clone()
        });
        cx.run_until_parked();
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        assert!(original_pending.wait().now_or_never().is_none());
        assert!(map.read_with(cx, |map, _| map.long_row_widths.pending.lock().is_none()));
        let (next_resume, next_pause) = futures::channel::oneshot::channel();
        map.update(cx, |map, cx| {
            map.masked = true;
            map.snapshot(cx);
            map.masked = false;
            let restored = map.snapshot(cx);
            assert!(restored.long_row_width_key() == original.long_row_width_key());
            assert!(!original_pending.is_current(&restored));
            map.long_row_widths.pause = Some(next_pause);
            assert_eq!(map.request_max_long_row_width(&original, cx), None);
            assert!(
                map.long_row_widths
                    .pending
                    .lock()
                    .as_ref()
                    .is_some_and(|request| Arc::ptr_eq(
                        &request.readiness,
                        &restored.row_readiness
                    ))
            );
        });
        resume.send(()).expect("superseded worker");
        cx.run_until_parked();
        assert!(map.read_with(cx, |map, _| map.long_row_widths.requested));
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        let resume = next_resume;

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(Point::new(0, 0)..Point::new(0, 2_048), "x".repeat(1_025))],
                None,
                cx,
            );
        });
        let superseded = map.update(cx, |map, cx| map.snapshot(cx));
        let superseded_cancelled = map.update(cx, |map, cx| {
            assert_eq!(map.request_max_long_row_width(&superseded, cx), None);
            map.long_row_widths.cancelled.clone()
        });
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(Point::new(1, 0)..Point::new(1, 1_536), "x".repeat(1_280))],
                None,
                cx,
            );
        });
        let latest = map.update(cx, |map, cx| map.snapshot(cx));
        let unready = latest.long_row(DisplayRow(1)).err().expect("pending row");
        assert!(unready.wait().now_or_never().is_none());
        map.update(cx, |map, cx| map.request_row_geometry(unready.clone(), cx));
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&latest, cx)),
            None
        );
        assert!(original_cancelled.load(atomic::Ordering::Acquire));
        assert!(original_pending.wait().now_or_never().is_some());
        assert!(superseded_cancelled.load(atomic::Ordering::Acquire));
        map.read_with(cx, |map, _| {
            assert!(
                map.long_row_widths
                    .pending
                    .lock()
                    .as_ref()
                    .is_some_and(|request| Arc::ptr_eq(&request.readiness, &latest.row_readiness))
            );
        });
        resume.send(()).expect("paused worker");
        cx.run_until_parked();
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&latest, cx)),
            Some(1_280. * 7.)
        );
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);
        assert_eq!(
            original
                .wrap_snapshot()
                .long_rows
                .rows
                .lock()
                .iter()
                .count(),
            0
        );
        assert_eq!(
            superseded
                .wrap_snapshot()
                .long_rows
                .rows
                .lock()
                .iter()
                .count(),
            0
        );
        assert!(unready.wait().now_or_never().is_some());
        assert!(
            latest
                .long_row(DisplayRow(1))
                .expect("row geometry ready")
                .is_some()
        );
        let masked = map.update(cx, |map, cx| {
            map.masked = true;
            let snapshot = map.snapshot(cx);
            assert_eq!(map.request_max_long_row_width(&snapshot, cx), Some(0.));
            snapshot
        });
        assert!(masked.long_row_width_key() != latest.long_row_width_key());
        map.update(cx, |map, cx| {
            map.masked = false;
            let snapshot = map.snapshot(cx);
            assert_eq!(
                map.request_max_long_row_width(&snapshot, cx),
                Some(1_280. * 7.)
            );
        });
        cx.run_until_parked();
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&latest, cx)),
            Some(1_280. * 7.)
        );
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(Point::new(1, 0)..Point::new(1, 1_280), "x".repeat(1_300))],
                None,
                cx,
            );
        });
        let stale = map.update(cx, |map, cx| map.snapshot(cx));
        let (resume, pause) = futures::channel::oneshot::channel();
        map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            assert_eq!(map.request_max_long_row_width(&stale, cx), None);
        });
        cx.run_until_parked();
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(Point::new(1, 0)..Point::new(1, 1_300), "x".repeat(1_400))],
                None,
                cx,
            );
        });
        resume.send(()).expect("paused worker");
        cx.run_until_parked();
        assert_eq!(map.read_with(cx, |map, _| map.long_row_widths.width), None);
        let current = map.update(cx, |map, cx| map.snapshot(cx));
        let pending_drop = current.long_row(DisplayRow(1)).err().expect("pending row");
        let (resume, pause) = futures::channel::oneshot::channel();
        let cancelled = map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            map.request_row_geometry(pending_drop.clone(), cx);
            assert_eq!(map.request_max_long_row_width(&current, cx), None);
            map.long_row_widths.cancelled.clone()
        });
        cx.run_until_parked();
        let shaped_rows = widths.shaped_rows.load(atomic::Ordering::Relaxed);
        let weak_map = map.downgrade();
        cx.update(|_| drop(map));
        cx.run_until_parked();
        assert!(weak_map.upgrade().is_none());
        assert!(pending_drop.wait().now_or_never().is_some());
        assert!(cancelled.load(atomic::Ordering::Acquire));
        assert!(resume.send(()).is_err());
        assert_eq!(
            widths.shaped_rows.load(atomic::Ordering::Relaxed),
            shaped_rows
        );
    }

    #[test]
    fn test_geometry_worker_shares_readiness_and_replaces_viewport() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let (_, map) = build_display_map(&format!("{}\n", "x".repeat(1_025)).repeat(34), 4, cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let original = (0..32)
            .map(|row| {
                snapshot
                    .long_row(DisplayRow(row))
                    .err()
                    .expect("pending row")
            })
            .collect::<Vec<_>>();
        let (resume, pause) = oneshot::channel();
        map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            map.request_viewport_row_geometry(original.iter().cloned(), cx);
        });
        cx.run_until_parked();
        for _ in 0..3_600 {
            map.update(cx, |map, cx| {
                let current = map.snapshot(cx);
                let pending = (0..32).map(|row| {
                    let pending = current
                        .long_row(DisplayRow(row))
                        .err()
                        .expect("pending row");
                    assert!(Arc::ptr_eq(
                        &pending.completion,
                        &original[row as usize].completion
                    ));
                    pending
                });
                map.request_viewport_row_geometry(pending, cx);
            });
        }
        assert_eq!(snapshot.row_readiness.retained.lock().len(), 32);
        let latest = snapshot
            .long_row(DisplayRow(33))
            .err()
            .expect("pending row");
        map.update(cx, |map, cx| {
            map.request_row_geometry(original[1].clone(), cx);
            let previous = snapshot
                .long_row(DisplayRow(32))
                .err()
                .expect("pending row");
            map.request_viewport_row_geometry([previous], cx);
            map.request_viewport_row_geometry([], cx);
            assert!(map.long_row_widths.priority.lock().visible.is_empty());
            map.request_viewport_row_geometry([latest.clone()], cx);
            let priority = map.long_row_widths.priority.lock();
            assert_eq!(
                priority.input.keys().copied().collect::<Vec<_>>(),
                [(1, false)]
            );
            assert_eq!(
                priority.visible.iter().copied().collect::<Vec<_>>(),
                [(33, false)]
            );
        });
        let widths = snapshot
            .wrap_snapshot()
            .long_rows
            .widths
            .as_ref()
            .expect("metrics");
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        assert_eq!(snapshot.row_readiness.retained.lock().len(), 34);
        resume.send(()).expect("paused worker");
        cx.run_until_parked();
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);
        assert_eq!(
            snapshot
                .wrap_snapshot()
                .long_rows
                .rows
                .lock()
                .iter()
                .map(|(row, _)| *row)
                .collect::<Vec<_>>(),
            [1, 33],
        );
        assert_eq!(snapshot.row_readiness.retained.lock().len(), 34);
        assert_eq!(
            original
                .iter()
                .filter(|pending| pending.wait().now_or_never().is_none())
                .count(),
            31
        );
        assert!(original[1].wait().now_or_never().is_some());
        assert!(latest.wait().now_or_never().is_some());
        assert!(original[0].wait().now_or_never().is_none());
        cx.update(|_| drop(map));
        cx.run_until_parked();
        assert!(original[0].wait().now_or_never().is_some());
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);
    }

    #[test]
    fn test_input_row_request_owns_retention_until_publication() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        for text_only in [false, true] {
            let (_, map) = build_display_map(&"x".repeat(2_048), 4, cx);
            set_test_shaping_style(&map, cx);
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            let pending = snapshot.pending_row(DisplayRow(0), text_only);
            let retained = Arc::downgrade(pending.retained.as_ref().expect("retained row"));
            let completion = pending.completion.receiver.clone();
            let (resume, pause) = oneshot::channel();
            map.update(cx, |map, cx| {
                map.long_row_widths.pause = Some(pause);
                map.request_row_geometry(pending, cx);
            });
            drop(snapshot);
            cx.run_until_parked();
            assert_eq!(retained.strong_count(), 1);
            assert!(completion.clone().now_or_never().is_none());
            resume.send(()).expect("paused worker");
            cx.run_until_parked();
            completion
                .now_or_never()
                .expect("row publication completed")
                .expect("row readiness was published");
            assert_eq!(retained.strong_count(), 0);
            map.read_with(cx, |map, _| {
                assert!(map.long_row_widths.priority.lock().input.is_empty());
                assert!(map.long_row_widths.readiness.retained.lock().is_empty());
            });
        }
    }

    #[test]
    fn test_worker_prepares_paint_text_and_short_row_constraints() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let (buffer, map) = build_display_map(&"x".repeat(2_048), 4, cx);
        set_test_shaping_style(&map, cx);
        let original = map.update(cx, |map, cx| map.snapshot(cx));
        let pending = original
            .prepared_row_text(DisplayRow(0))
            .err()
            .expect("pending text");
        map.update(cx, |map, cx| map.request_row_geometry(pending.clone(), cx));
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_some());
        let widths = original
            .wrap_snapshot()
            .long_rows
            .widths
            .as_ref()
            .expect("metrics");
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        let text = original
            .prepared_row_text(DisplayRow(0))
            .expect("ready text")
            .expect("long row");
        assert_eq!(
            text.chunks(0..usize::MAX)
                .map(|chunk| chunk.len())
                .sum::<usize>(),
            2_048
        );
        let geometry = original
            .long_row(DisplayRow(0))
            .err()
            .expect("pending geometry");
        map.update(cx, |map, cx| map.request_row_geometry(geometry, cx));
        cx.run_until_parked();
        let original_geometry = original
            .long_row(DisplayRow(0))
            .expect("ready")
            .expect("long row");
        let buffer = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        map.update(cx, |map, cx| {
            map.highlight_text(
                HighlightKey::Editor,
                vec![
                    buffer.anchor_after(MultiBufferOffset(0))
                        ..buffer.anchor_before(MultiBufferOffset(2_048)),
                ],
                HighlightStyle {
                    background_color: Some(gpui::blue()),
                    ..HighlightStyle::default()
                },
                false,
                cx,
            );
        });
        let painted = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(!pending.is_current(&painted));
        let ready = painted
            .long_row(DisplayRow(0))
            .expect("unchanged geometry")
            .expect("long row");
        assert!(Arc::ptr_eq(&ready.geometry, &original_geometry.geometry));
        let pending = painted
            .prepared_row_text(DisplayRow(0))
            .err()
            .expect("new paint");
        map.update(cx, |map, cx| map.request_row_geometry(pending, cx));
        cx.run_until_parked();
        let painted_text = painted
            .prepared_row_text(DisplayRow(0))
            .expect("ready paint")
            .expect("long row");
        for (prepared, background) in [(&painted_text, Some(gpui::blue())), (&text, None)] {
            for chunk in prepared.chunks(0..usize::MAX) {
                let LineChunk::Text(chunk) = chunk else {
                    panic!("expected text without replacements");
                };
                assert_eq!(
                    chunk.style.and_then(|style| style.background_color),
                    background
                );
            }
        }
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 1);

        let (short_buffer, map) = build_display_map("short", 4, cx);
        map.update(cx, |map, cx| map.set_wrap_width(Some(px(70.)), cx));
        cx.run_until_parked();
        let renderer = ChunkRenderer {
            id: ChunkRendererId::Inlay(InlayId::Hint(0)),
            render: Arc::new(|_| panic!("constraint preparation must not render")),
            constrain_width: true,
            measured_width: None,
            width_class: None,
        };
        let old_text = SharedString::from("x".repeat(2_048));
        let latest_text = SharedString::from("x".repeat(3_072));
        let (resume, pause) = oneshot::channel();
        map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            assert!(
                map.request_renderer_constraint(
                    renderer.clone(),
                    old_text,
                    test_font(),
                    FONT_SIZE,
                    cx
                )
                .is_none()
            );
        });
        cx.run_until_parked();
        map.update(cx, |map, cx| {
            for _ in 0..2 {
                assert!(
                    map.request_renderer_constraint(
                        renderer.clone(),
                        latest_text.clone(),
                        test_font(),
                        FONT_SIZE,
                        cx
                    )
                    .is_none()
                );
            }
            assert_eq!(map.long_row_widths.priority.lock().constraints.len(), 1);
        });
        resume.send(()).expect("paused worker");
        cx.run_until_parked();
        let constraint = map.update(cx, |map, cx| {
            map.request_renderer_constraint(
                renderer.clone(),
                latest_text.clone(),
                test_font(),
                FONT_SIZE,
                cx,
            )
            .expect("ready constraint")
        });
        assert_eq!(constraint.width(), Some(px(3_072. * 7.)));
        assert!(constraint.matches(&renderer, &latest_text, &test_font(), FONT_SIZE));
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(!snapshot.is_long_row(DisplayRow(0)));
        assert!(Arc::ptr_eq(
            &constraint,
            &snapshot
                .prepared_renderer_constraint(renderer.id)
                .expect("published constraint")
        ));

        set_test_shaping_style(&map, cx);
        let key = RendererWidthsKey {
            rem_size: px(16.),
            font: test_font(),
            font_size: FONT_SIZE,
            buffer_font: test_font(),
            editor_width: px(70.),
            line_height: px(20.),
        };
        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    Point::new(0, 0)..Point::new(0, 2),
                    FoldPlaceholder {
                        collapsed_text: Some(SharedString::from("xy")),
                        ..FoldPlaceholder::test()
                    },
                )],
                cx,
            );
            map.update_renderer_widths(key.clone(), [], cx);
        });
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut renderers = Vec::new();
        snapshot.for_each_unmeasured_renderer(DisplayRow(0)..DisplayRow(1), |renderer, text| {
            renderers.push((renderer, text));
            ControlFlow::Continue(())
        });
        assert_eq!(renderers.len(), 1);
        let (renderer, text) = renderers.pop().expect("fold renderer");
        assert_eq!(text.materialize().as_str(), "xy");
        map.update(cx, |map, cx| {
            for _ in 0..32 {
                let (_, current) = map
                    .snapshot(cx)
                    .fold_snapshot()
                    .renderer_source(renderer.id)
                    .expect("fold source");
                assert!(text.same_source(&current));
                assert!(!text.same_source(&SharedString::from("xz").into()));
                assert!(
                    map.request_renderer_constraint(
                        renderer.clone(),
                        current,
                        test_font(),
                        FONT_SIZE,
                        cx,
                    )
                    .is_none()
                );
            }
            assert_eq!(map.long_row_widths.priority.lock().constraints.len(), 1);
        });
        cx.run_until_parked();
        map.update(cx, |map, cx| {
            let before_width_update = map.snapshot(cx);
            let prepared = before_width_update
                .prepared_renderer_constraint(renderer.id)
                .expect("prepared fold constraint");
            assert_eq!(prepared.width(), Some(px(14.)));
            short_buffer
                .read(cx)
                .as_singleton()
                .expect("singleton buffer")
                .update(cx, |buffer, cx| {
                    let anchor = buffer.snapshot().anchor_after(Point::new(0, 3));
                    buffer.set_active_selections(
                        Arc::from([Selection {
                            id: 0,
                            start: anchor,
                            end: anchor,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }]),
                        false,
                        CursorShape::Bar,
                        cx,
                    );
                });
            let selected = map.snapshot(cx);
            assert!(Arc::ptr_eq(
                &before_width_update.row_readiness,
                &selected.row_readiness,
            ));
            for width in [
                Some(-14.826),
                Some(0.),
                Some(70.),
                Some(140.),
                Some(70.),
                None,
                Some(70.),
            ] {
                map.set_wrap_width(width.map(px), cx);
                let snapshot = map.snapshot(cx);
                let constraint = snapshot
                    .prepared_renderer_constraint(renderer.id)
                    .expect("wrap-only changes retain renderer constraints");
                assert!(Arc::ptr_eq(&prepared, &constraint));
                assert_eq!(constraint.width(), Some(px(14.)));
                assert_eq!(map.long_row_widths.priority.lock().constraints.len(), 0);
            }
            let before_width_update = map.snapshot(cx);
            map.update_renderer_widths(key.clone(), [(renderer.id, px(14.))], cx);
            let snapshot = map.snapshot(cx);
            assert!(!Arc::ptr_eq(
                &before_width_update.row_readiness,
                &snapshot.row_readiness,
            ));
            let (current, text) = snapshot
                .fold_snapshot()
                .renderer_source(renderer.id)
                .expect("measured fold source");
            assert!(
                prepared.text().same_source(&text),
                "width-only update replaced renderer text source"
            );
            let constraint = snapshot
                .prepared_renderer_constraint(renderer.id)
                .expect("retained fold constraint");
            assert!(Arc::ptr_eq(&prepared, &constraint));
            assert!(constraint.matches(&current, &text, &test_font(), FONT_SIZE));
        });
        cx.run_until_parked();
        short_buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 5)..Point::new(0, 5), "!")], None, cx);
        });
        map.update(cx, |map, cx| map.snapshot(cx));
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(!snapshot.fold_snapshot().has_unmeasured_renderers());
        assert!(snapshot.prepared_renderer_constraint(renderer.id).is_none());
        let pending = snapshot.pending_renderer_constraint(DisplayRow(0), renderer.id);
        map.update(cx, |map, cx| map.request_row_geometry(pending.clone(), cx));
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_some());
        assert_eq!(
            snapshot
                .prepared_renderer_constraint(renderer.id)
                .expect("restored constraint")
                .width(),
            Some(px(14.))
        );

        let (resume, pause) = oneshot::channel();
        let (previous, pending) = map.update(cx, |map, cx| {
            let mut style = map
                .paint_source
                .as_ref()
                .expect("paint source")
                .style
                .clone();
            style.text.font_size = AbsoluteLength::from(gpui::rems(1.));
            map.set_shaping_style(&style, SyntaxStyling::default(), cx);
            map.long_row_widths.pause = Some(pause);
            let snapshot = map.snapshot(cx);
            assert_eq!(
                snapshot.renderer_constraint_font(),
                Some((test_font(), px(16.)))
            );
            let pending = snapshot.pending_renderer_constraint(DisplayRow(0), renderer.id);
            map.request_row_geometry(pending.clone(), cx);
            (snapshot, pending)
        });
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_none());
        let (resized, next_pending) = map.update(cx, |map, cx| {
            let mut key = key.clone();
            key.rem_size = px(24.);
            assert!(!map.update_renderer_widths(key, [(renderer.id, px(14.))], cx));
            let snapshot = map.snapshot(cx);
            assert_eq!(
                snapshot.tab_snapshot().version,
                previous.tab_snapshot().version
            );
            assert!(!pending.is_current(&snapshot));
            assert!(snapshot.prepared_renderer_constraint(renderer.id).is_none());
            let pending = snapshot.pending_renderer_constraint(DisplayRow(0), renderer.id);
            map.request_row_geometry(pending.clone(), cx);
            (snapshot, pending)
        });
        assert!(pending.wait().now_or_never().is_some());
        assert!(next_pending.wait().now_or_never().is_none());
        resume.send(()).expect("paused renderer worker");
        cx.run_until_parked();
        assert!(next_pending.wait().now_or_never().is_some());
        assert_eq!(
            resized
                .prepared_renderer_constraint(renderer.id)
                .expect("resized constraint")
                .width(),
            Some(px(24.))
        );

        let (buffer, map) = build_display_map("short", 4, cx);
        set_test_shaping_style(&map, cx);
        let position = buffer.read_with(cx, |buffer, cx| {
            buffer.snapshot(cx).anchor_after(MultiBufferOffset(1))
        });
        let inlay_text = format!("{}fi\n{}é", "x".repeat(127), "x".repeat(999_868));
        assert_eq!(inlay_text.len(), 1_000_000);
        let inlay = Inlay::mock_repl_result(0, position, &inlay_text);
        let key = RendererWidthsKey {
            rem_size: px(16.),
            font: test_font(),
            font_size: FONT_SIZE,
            buffer_font: test_font(),
            editor_width: px(70.),
            line_height: px(20.),
        };
        map.update(cx, |map, cx| {
            map.splice_inlays(&[], vec![inlay], cx);
            map.update_renderer_widths(key.clone(), [], cx);
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let id = ChunkRendererId::Inlay(InlayId::ReplResult(0));
        let (renderer, source) = snapshot
            .fold_snapshot()
            .renderer_source(id)
            .expect("complete inlay source");
        let pending = snapshot.pending_renderer_constraint(DisplayRow(0), id);
        let (resume, pause) = oneshot::channel();
        map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            map.request_row_geometry(pending.clone(), cx);
        });
        cx.run_until_parked();
        for _ in 0..32 {
            let mut visits = 0;
            snapshot.for_each_unmeasured_renderer(
                DisplayRow(0)..snapshot.max_point().row() + DisplayRow(1),
                |current, text| {
                    assert!(source.same_source(&text));
                    assert!(Arc::ptr_eq(&renderer.render, &current.render));
                    visits += 1;
                    ControlFlow::Continue(())
                },
            );
            assert_eq!(visits, 1);
            map.update(cx, |map, cx| map.request_row_geometry(pending.clone(), cx));
        }
        assert_eq!(
            map.read_with(cx, |map, _| map
                .long_row_widths
                .priority
                .lock()
                .constraints
                .len()),
            1
        );
        resume.send(()).expect("paused worker");
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_some());
        let constraint = snapshot
            .prepared_renderer_constraint(id)
            .expect("complete constraint");
        assert_eq!(constraint.width(), Some(px(6_999_989.5)));
        assert!(constraint.text().same_source(&source));
        map.update(cx, |map, cx| {
            let ready = map
                .request_renderer_constraint(
                    renderer.clone(),
                    source.clone(),
                    test_font(),
                    FONT_SIZE,
                    cx,
                )
                .expect("reuse deferred constraint");
            assert!(Arc::ptr_eq(&constraint, &ready));
            map.update_renderer_widths(key, [(id, px(20.))], cx);
            let snapshot = map.snapshot(cx);
            let (renderer, text) = snapshot
                .fold_snapshot()
                .renderer_source(id)
                .expect("measured source");
            assert_eq!(renderer.measured_width, Some(px(20.)));
            assert!(text.same_source(&source));
            assert!(Arc::ptr_eq(
                &constraint,
                &snapshot
                    .prepared_renderer_constraint(id)
                    .expect("retained constraint")
            ));
            map.splice_inlays(
                &[InlayId::ReplResult(0)],
                vec![Inlay::mock_repl_result(0, position, "fi")],
                cx,
            );
            let snapshot = map.snapshot(cx);
            let (renderer, text) = snapshot
                .fold_snapshot()
                .renderer_source(id)
                .expect("replacement source");
            assert!(!constraint.matches(&renderer, &text, &test_font(), FONT_SIZE));
            assert!(snapshot.prepared_renderer_constraint(id).is_none());
        });
    }

    #[test]
    fn test_geometry_worker_prioritizes_rows_and_defers_drop() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let (_, map) = build_display_map(
            &format!(
                "{}\n{}\n{}",
                "x".repeat(2_048),
                "x".repeat(1_536),
                "x".repeat(1_280)
            ),
            4,
            cx,
        );
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let request = LongRowWidthRequest {
            paint_source: snapshot.paint_source.clone(),
            snapshot: snapshot.tab_snapshot().clone(),
            rows: snapshot
                .wrap_snapshot()
                .long_rows
                .detached(cx.background_executor.clone()),
            readiness: snapshot.row_readiness.clone(),
            text_system: cx.update(|cx| cx.text_system().clone()),
            executor: cx.background_executor.clone(),
            excluded: snapshot.block_snapshot.replaced_wrap_rows(),
            measure_width: Arc::new(atomic::AtomicBool::new(true)),
            priority: Arc::default(),
            cancelled: Arc::default(),
            pause: None,
        };
        let priority = request.priority.clone();
        let cancelled = request.cancelled.clone();
        let (mut results, mut completed) = mpsc::channel(0);
        let worker = cx
            .background_executor
            .spawn(async move { request.prepare(&mut results).await });
        for expected in [0, 2, 1] {
            cx.run_until_parked();
            let (readiness, completion) = completed
                .next()
                .now_or_never()
                .flatten()
                .expect("completed row");
            let LongRowCompletion::Row((row, false), Some(geometry), _, accepted) = completion
            else {
                panic!("expected measured geometry");
            };
            assert!(Arc::ptr_eq(&readiness, &snapshot.row_readiness));
            assert!(!cancelled.load(atomic::Ordering::Acquire));
            assert_eq!(row, expected);
            if row == 0 {
                let retained = snapshot.row_readiness.retain_row(2, &snapshot.row_geometry);
                priority.lock().input.insert((2, false), retained);
            }
            let retained = snapshot
                .row_readiness
                .retain_row(row, &snapshot.row_geometry);
            retained.geometry.set(geometry.clone()).ok();
            snapshot
                .wrap_snapshot()
                .long_rows
                .publish_row(snapshot.tab_snapshot(), row, geometry);
            accepted.send(true).expect("waiting worker");
        }
        cx.run_until_parked();
        assert_eq!(worker.now_or_never(), Some(Some(2_048. * 7.)));
        let cache = snapshot.wrap_snapshot().long_rows.rows.clone();
        let cached = Arc::downgrade(
            &snapshot
                .long_row(DisplayRow(0))
                .expect("ready row")
                .expect("long row")
                .geometry,
        );
        let retained = snapshot
            .long_row(DisplayRow(2))
            .expect("ready row")
            .expect("long row");
        let geometry = Arc::downgrade(&retained.geometry);
        let columns = Arc::downgrade(&retained.geometry.atoms.columns);
        drop(snapshot);
        cx.update(|_| drop(map));
        cx.run_until_parked();
        drop(cache);
        assert!(cached.upgrade().is_none());
        assert!(geometry.upgrade().is_some());
        drop(retained);
        assert!(geometry.upgrade().is_none());
        assert!(columns.upgrade().is_some());
        cx.run_until_parked();
        assert!(columns.upgrade().is_none());
    }

    #[test]
    fn test_width_scan_does_not_own_geometry() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let (_, map) = build_display_map(&format!("{}\n", "x".repeat(2_048)).repeat(128), 4, cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        map.update(cx, |map, cx| {
            assert_eq!(map.request_max_long_row_width(&snapshot, cx), None);
        });
        cx.run_until_parked();
        let rows = &snapshot.wrap_snapshot().long_rows;
        assert_eq!(
            rows.try_max_width(snapshot.tab_snapshot(), &[]),
            Some(2_048. * 7.)
        );
        assert_eq!(rows.rows.lock().iter().count(), 128);
        assert_eq!(
            rows.rows
                .lock()
                .iter()
                .filter(|(_, row)| row.geometry.strong_count() > 0)
                .count(),
            0
        );
        assert_eq!(snapshot.row_readiness.retained.lock().len(), 0);
        let shaped = rows
            .widths
            .as_ref()
            .expect("metrics")
            .shaped_rows
            .load(atomic::Ordering::Relaxed);
        map.update(cx, |map, cx| {
            assert_eq!(
                map.request_max_long_row_width(&snapshot, cx),
                Some(2_048. * 7.)
            );
        });
        cx.run_until_parked();
        assert_eq!(
            rows.widths
                .as_ref()
                .expect("metrics")
                .shaped_rows
                .load(atomic::Ordering::Relaxed),
            shaped
        );
    }

    #[test]
    fn test_long_row_readiness_tracks_render_dependencies() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let line = format!("\"{}\"", "x".repeat(2_048));
        let (buffer, map) = build_display_map(&format!("{line}\n{line}"), 4, cx);
        set_test_shaping_style(&map, cx);
        let language = languages::rust_lang();
        map.update(cx, |map, cx| {
            let mut style = map
                .long_row_source
                .as_ref()
                .expect("shaping source")
                .style
                .clone();
            style.syntax = Arc::new(theme::SyntaxTheme::new_test_styles([(
                "string",
                HighlightStyle {
                    font_weight: Some(FontWeight::BOLD),
                    ..HighlightStyle::default()
                },
            )]));
            language.set_theme(&style.syntax);
            map.set_shaping_style(&style, SyntaxStyling::default(), cx);
        });
        let source_buffer = buffer.read_with(cx, |buffer, _| {
            buffer.as_singleton().expect("singleton buffer")
        });
        source_buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx));
        cx.run_until_parked();
        let source_before = source_buffer.read_with(cx, |buffer, _| buffer.snapshot());
        assert!(source_before.syntax_layers().next().is_some());
        let original = map.update(cx, |map, cx| {
            let snapshot = map.snapshot(cx);
            map.request_row_geometry(snapshot.pending_row(DisplayRow(0), false), cx);
            map.request_max_long_row_width(&snapshot, cx);
            snapshot
        });
        cx.run_until_parked();
        let geometry = original
            .long_row(DisplayRow(0))
            .expect("ready geometry")
            .expect("long row");
        let paint = original
            .prepared_row_text(DisplayRow(0))
            .expect("ready paint")
            .expect("long row");
        let rows = &original.wrap_snapshot().long_rows;
        let widths = rows.widths.as_ref().expect("shaping metrics");
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);
        assert_eq!(
            rows.rows
                .lock()
                .get(&1)
                .expect("offscreen width")
                .geometry
                .strong_count(),
            0
        );
        let expected_width = Some(line.len() as f64 * 8.);
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&original, cx)),
            expected_width
        );
        let pending = original.long_row(DisplayRow(1)).err().expect("pending row");
        let (resume, pause) = oneshot::channel();
        map.update(cx, |map, cx| {
            map.long_row_widths.pause = Some(pause);
            map.request_row_geometry(pending.clone(), cx);
        });
        cx.run_until_parked();

        for remote in [false, true] {
            source_buffer.update(cx, |buffer, cx| {
                let anchor = buffer
                    .snapshot()
                    .anchor_after(Point::new(0, 1 + u32::from(remote)));
                let selections = Arc::from([Selection {
                    id: 0,
                    start: anchor,
                    end: anchor,
                    reversed: false,
                    goal: SelectionGoal::None,
                }]);
                if remote {
                    buffer.apply_ops(
                        [Operation::UpdateSelections {
                            selections,
                            lamport_timestamp: Lamport::new(ReplicaId::FIRST_COLLAB_ID),
                            line_mode: false,
                            cursor_shape: CursorShape::Bar,
                        }],
                        cx,
                    );
                } else {
                    buffer.set_active_selections(selections, false, CursorShape::Bar, cx);
                }
            });
            let source_after = source_buffer.read_with(cx, |buffer, _| buffer.snapshot());
            assert_eq!(source_after.version(), source_before.version());
            assert_eq!(
                source_after.syntax_update_count(),
                source_before.syntax_update_count()
            );
            let current = map.update(cx, |map, cx| map.snapshot(cx));
            assert!(
                pending.is_current(&current),
                "selection update: remote={remote}"
            );
            assert!(pending.wait().now_or_never().is_none());
            assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);
            assert_eq!(
                map.update(cx, |map, cx| map.request_max_long_row_width(&current, cx)),
                expected_width
            );
            assert!(Arc::ptr_eq(
                &current
                    .long_row(DisplayRow(0))
                    .expect("ready geometry")
                    .expect("long row")
                    .geometry,
                &geometry.geometry
            ));
            assert!(Arc::ptr_eq(
                &current
                    .prepared_row_text(DisplayRow(0))
                    .expect("ready paint")
                    .expect("long row"),
                &paint
            ));
        }
        resume.send(()).expect("pending worker");
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_some());
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 3);

        source_buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 1)..Point::new(0, 2), "yy")], None, cx);
        });
        let edited = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(!pending.is_current(&edited));
        assert!(edited.long_row(DisplayRow(0)).is_err());
        assert!(edited.prepared_row_text(DisplayRow(0)).is_err());
        let edited = ready_snapshot(&map, cx);
        let edited_len = line.len() + 1;
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&edited, cx)),
            Some(edited_len as f64 * 8.)
        );
        assert_eq!(
            rows.try_max_width(original.tab_snapshot(), &[]),
            expected_width
        );

        source_buffer.update(cx, |buffer, cx| buffer.set_language(None, cx));
        let unstyled = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(
            !edited
                .pending_row(DisplayRow(0), false)
                .is_current(&unstyled)
        );
        assert!(unstyled.long_row(DisplayRow(0)).is_err());
        assert!(unstyled.prepared_row_text(DisplayRow(0)).is_err());
        let unstyled = ready_snapshot(&map, cx);
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&unstyled, cx)),
            Some(edited_len as f64 * 7.)
        );

        for present in [true, false] {
            let previous = ready_snapshot(&map, cx);
            source_buffer.update(cx, |buffer, cx| {
                let diagnostics = DiagnosticSet::new(
                    present.then(|| {
                        DiagnosticEntry::new(
                            PointUtf16::new(0, 0)..PointUtf16::new(0, edited_len as u32),
                            Diagnostic::default(),
                        )
                    }),
                    buffer,
                );
                buffer.update_diagnostics(LanguageServerId(0), diagnostics, cx);
            });
            let updated = map.update(cx, |map, cx| map.snapshot(cx));
            assert!(
                !previous
                    .pending_row(DisplayRow(0), false)
                    .is_current(&updated)
            );
            assert!(updated.prepared_row_text(DisplayRow(0)).is_err());
            let updated = ready_snapshot(&map, cx);
            let paint = updated
                .prepared_row_text(DisplayRow(0))
                .expect("ready diagnostic paint")
                .expect("long row");
            assert_eq!(
                paint
                    .chunks(0..usize::MAX)
                    .map(|chunk| {
                        let LineChunk::Text(chunk) = chunk else {
                            panic!("expected text");
                        };
                        (chunk.text.len(), chunk.diagnostic_underline_severity)
                    })
                    .collect::<Vec<_>>(),
                [(
                    edited_len,
                    present.then_some(lsp::DiagnosticSeverity::ERROR)
                )]
            );
            assert_eq!(
                map.update(cx, |map, cx| map.request_max_long_row_width(&updated, cx)),
                Some(edited_len as f64 * 7.)
            );
        }
    }

    #[test]
    fn test_geometry_lease_preserves_multirow_retry_progress_and_releases_rows() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let (buffer, map) =
            build_display_map(&format!("{}\n", "x".repeat(1_025)).repeat(128), 4, cx);
        set_test_shaping_style(&map, cx);
        let lease = map.read_with(cx, |map, _| map.retain_row_geometry());
        let mut geometries = Vec::new();
        let mut texts = Vec::new();
        for required in 0..128 {
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            for row in 0..required {
                assert!(
                    snapshot
                        .long_row(DisplayRow(row))
                        .expect("retained row")
                        .is_some()
                );
            }
            let pending = snapshot
                .long_row(DisplayRow(required))
                .err()
                .expect("new row");
            map.update(cx, |map, cx| map.request_row_geometry(pending.clone(), cx));
            drop(snapshot);
            cx.run_until_parked();
            assert!(pending.wait().now_or_never().is_some());
            drop(pending);
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            let row = snapshot
                .long_row(DisplayRow(required))
                .expect("ready row")
                .expect("long row");
            geometries.push(Arc::downgrade(&row.geometry));
            texts.push(Arc::downgrade(
                &snapshot
                    .prepared_row_text(DisplayRow(required))
                    .expect("ready text")
                    .expect("prepared text"),
            ));
        }
        assert_eq!(
            geometries
                .iter()
                .filter(|row| row.strong_count() > 0)
                .count(),
            128
        );
        assert_eq!(
            texts.iter().filter(|text| text.strong_count() > 0).count(),
            128
        );
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let cold_clone = snapshot.clone();
        assert!(snapshot.row_geometry.get().is_none());
        assert_eq!(
            map.read_with(cx, |map, _| map.retained_row_counts(Some(&cold_clone))),
            (128, 128, 128)
        );
        assert!(cold_clone.row_geometry.get().is_none());
        let retained = snapshot
            .long_row(DisplayRow(0))
            .expect("ready row")
            .expect("long row");
        let initialized_clone = snapshot.clone();
        assert!(Arc::ptr_eq(
            &snapshot.row_geometry.get().expect("initialized lease").rows,
            &initialized_clone
                .row_geometry
                .get()
                .expect("cloned lease")
                .rows,
        ));
        drop(lease);
        cx.run_until_parked();
        assert_eq!(
            geometries
                .iter()
                .filter(|row| row.strong_count() > 0)
                .count(),
            1
        );
        assert_eq!(
            texts.iter().filter(|text| text.strong_count() > 0).count(),
            1
        );
        assert_eq!(snapshot.row_readiness.retained.lock().len(), 1);
        drop(snapshot);
        assert_eq!(
            texts.iter().filter(|text| text.strong_count() > 0).count(),
            1
        );
        drop(initialized_clone);
        assert_eq!(
            texts.iter().filter(|text| text.strong_count() > 0).count(),
            0
        );
        drop(retained);
        cx.run_until_parked();
        assert_eq!(
            geometries
                .iter()
                .filter(|row| row.strong_count() > 0)
                .count(),
            0
        );
        assert_eq!(cold_clone.row_readiness.retained.lock().len(), 0);
        drop(cold_clone);
        let old = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(old.row_readiness.retained.lock().len(), 0);
        let lease = map.read_with(cx, |map, _| map.retain_row_geometry());
        let pending = old
            .long_row(DisplayRow(0))
            .err()
            .expect("released geometry");
        assert_eq!(lease.rows.lock().len(), 1);
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 0)..Point::new(0, 1), "y")], None, cx);
        });
        let current = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(pending.wait().now_or_never().is_some());
        assert_eq!(lease.rows.lock().len(), 0);
        let current_pending = current
            .long_row(DisplayRow(0))
            .err()
            .expect("edited geometry");
        assert!(old.long_row(DisplayRow(0)).is_err());
        assert!(Arc::ptr_eq(
            lease.rows.lock().get(&0).expect("current demand"),
            current_pending.retained.as_ref().expect("retained demand")
        ));
    }

    #[test]
    fn test_width_index_reuses_scalars_and_invalidates() {
        let cx = &mut shaping_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let short_rows = format!("{}\n", "x".repeat(257)).repeat(1_024);
        let text = format!("{short_rows}{}\n{}", "x".repeat(2_048), "x".repeat(1_536));
        let (buffer, map) = build_display_map(&text, 4, cx);
        set_test_shaping_style(&map, cx);
        let original = map.update(cx, |map, cx| map.snapshot(cx));
        let long_rows = &original.wrap_snapshot().long_rows;
        assert!(original.long_row_width_key() == original.clone().long_row_width_key());
        let widths = long_rows.widths.as_ref().expect("shaping metrics");
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        assert!(Arc::ptr_eq(
            &long_rows.rows,
            &long_rows.interpolate(original.tab_snapshot(), &[]).rows,
        ));
        assert_eq!(
            long_rows.try_max_width(original.tab_snapshot(), &[WrapRow(0)..WrapRow(1_026)]),
            Some(0.)
        );
        assert_eq!(
            long_rows.try_max_width(original.tab_snapshot(), &[WrapRow(0)..WrapRow(1_025)]),
            None
        );
        assert_eq!(long_rows.rows.lock().iter().count(), 0);
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 0);
        let pending = original
            .long_row(DisplayRow(1_025))
            .err()
            .expect("pending row");
        map.update(cx, |map, cx| map.request_row_geometry(pending.clone(), cx));
        cx.run_until_parked();
        assert!(pending.wait().now_or_never().is_some());
        assert_eq!(
            original
                .long_row(DisplayRow(1_025))
                .expect("row geometry ready")
                .expect("visible row")
                .width(),
            1_536. * 7.
        );
        for _ in 0..2 {
            assert_eq!(
                long_rows.try_max_width(original.tab_snapshot(), &[WrapRow(0)..WrapRow(1_025)]),
                Some(1_536. * 7.)
            );
            assert_eq!(
                long_rows
                    .rows
                    .lock()
                    .iter()
                    .map(|(row, _)| *row)
                    .collect::<Vec<_>>(),
                [1_025]
            );
            assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 1);
        }
        let notifications = Arc::new(AtomicUsize::new(0));
        let subscription = cx.update(|cx| {
            let notifications = notifications.clone();
            cx.observe(&map, move |_, _| {
                notifications.fetch_add(1, atomic::Ordering::Relaxed);
            })
        });
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&original, cx)),
            None
        );
        cx.run_until_parked();
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&original, cx)),
            Some(2_048. * 7.)
        );
        assert_eq!(
            long_rows.try_max_width(original.tab_snapshot(), &[]),
            Some(2_048. * 7.)
        );
        assert_eq!(
            long_rows.try_max_width(original.tab_snapshot(), &[WrapRow(0)..WrapRow(1_025)]),
            Some(1_536. * 7.)
        );
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);
        assert_eq!(notifications.load(atomic::Ordering::Relaxed), 1);
        drop(subscription);

        let mut tab_snapshot = original.tab_snapshot().clone();
        tab_snapshot.version += 1;
        assert_eq!(
            long_rows.try_max_width(&tab_snapshot, &[WrapRow(0)..WrapRow(1_024)]),
            None
        );
        let unchanged = long_rows.interpolate(&tab_snapshot, &[]);
        tab_snapshot.version += 1;
        let unchanged = unchanged.interpolate(&tab_snapshot, &[]);
        assert_eq!(
            unchanged.try_max_width(&tab_snapshot, &[]),
            Some(2_048. * 7.)
        );
        for (end_row, expected) in [(63, Some(2_048. * 7.)), (64, None)] {
            let range = TabPoint::new(0, 0)..TabPoint::new(end_row, 257);
            let changed = unchanged.interpolate(
                &tab_snapshot,
                &[TabEdit {
                    old: range.clone(),
                    new: range,
                }],
            );
            assert_eq!(changed.try_max_width(&tab_snapshot, &[]), expected);
        }
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 0)..Point::new(0, 1), "y")], None, cx);
        });
        let short_edit = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            map.update(cx, |map, cx| map
                .request_max_long_row_width(&short_edit, cx)),
            Some(2_048. * 7.)
        );
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 2);
        assert!(long_rows.row(original.tab_snapshot(), 1_024).is_none());
        let previous = long_rows
            .row(original.tab_snapshot(), 1_025)
            .expect("original row");
        let current = unchanged.row(&tab_snapshot, 1_025).expect("unchanged row");
        assert!(Arc::ptr_eq(previous.geometry(), current.geometry()));

        let source = long_rows.source.as_ref().expect("shaping source");
        let resolved_bytes = source.resolved_bytes.load(atomic::Ordering::Relaxed);
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(
                    MultiBufferOffset(short_rows.len())
                        ..MultiBufferOffset(short_rows.len() + 2_048),
                    "x".repeat(1_025),
                )],
                None,
                cx,
            );
        });
        let edited = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(edited.long_row_width_key() != original.long_row_width_key());
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&edited, cx)),
            None
        );
        cx.run_until_parked();
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&edited, cx)),
            Some(1_536. * 7.)
        );
        assert_eq!(
            long_rows.try_max_width(original.tab_snapshot(), &[WrapRow(0)..WrapRow(1_024)]),
            Some(2_048. * 7.)
        );
        assert_eq!(widths.shaped_rows.load(atomic::Ordering::Relaxed), 3);
        assert_eq!(
            source.resolved_bytes.load(atomic::Ordering::Relaxed) - resolved_bytes,
            1_025,
        );

        let snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let mut previous_source = None::<Weak<LongRowSource>>;
        for (index, color) in [None, Some(gpui::blue())].into_iter().enumerate() {
            map.update(cx, |map, cx| {
                map.highlight_text(
                    HighlightKey::Editor,
                    vec![
                        snapshot.anchor_after(Point::new(1_025, 0))
                            ..snapshot.anchor_before(Point::new(1_025, 1_536)),
                    ],
                    HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        color,
                        ..HighlightStyle::default()
                    },
                    false,
                    cx,
                );
            });
            let restyled = map.update(cx, |map, cx| map.snapshot(cx));
            let pending = restyled
                .long_row(DisplayRow(1_025))
                .err()
                .expect("restyled row");
            map.update(cx, |map, cx| map.request_row_geometry(pending, cx));
            if let Some(previous_source) = previous_source.take() {
                assert!(previous_source.upgrade().is_none());
            }
            previous_source = restyled
                .wrap_snapshot()
                .long_rows
                .source
                .as_ref()
                .map(Arc::downgrade);
            assert_eq!(
                restyled.tab_snapshot().version,
                edited.tab_snapshot().version
            );
            assert!(restyled.long_row_width_key() != edited.long_row_width_key());
            assert_eq!(
                restyled
                    .wrap_snapshot()
                    .long_rows
                    .try_max_width(restyled.tab_snapshot(), &[]),
                None
            );
            assert_eq!(
                map.update(cx, |map, cx| map.request_max_long_row_width(&restyled, cx)),
                None
            );
            cx.run_until_parked();
            assert_eq!(
                map.update(cx, |map, cx| map.request_max_long_row_width(&restyled, cx)),
                Some(1_536. * 8.)
            );
            assert_eq!(
                edited
                    .wrap_snapshot()
                    .long_rows
                    .try_max_width(edited.tab_snapshot(), &[WrapRow(0)..WrapRow(1_024)],),
                Some(1_536. * 7.)
            );
            assert_eq!(
                edited
                    .long_row(DisplayRow(1_025))
                    .expect("row geometry ready")
                    .expect("original source")
                    .width(),
                1_536. * 7.
            );
            assert_eq!(
                long_rows.try_max_width(original.tab_snapshot(), &[WrapRow(0)..WrapRow(1_024)]),
                Some(2_048. * 7.)
            );
            assert_eq!(
                widths.shaped_rows.load(atomic::Ordering::Relaxed),
                5 + index * 2
            );
            assert!(
                restyled
                    .long_row(DisplayRow(1_025))
                    .expect("row geometry ready")
                    .is_some()
            );
        }
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                (0..9).map(|row| (Point::new(row, 257)..Point::new(row + 1, 0), "")),
                None,
                cx,
            );
        });
        let joined = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(joined.tab_snapshot().line_len(0), 2_570);
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&joined, cx)),
            None
        );
        cx.run_until_parked();
        assert_eq!(
            map.update(cx, |map, cx| map.request_max_long_row_width(&joined, cx)),
            Some(2_570. * 7.)
        );

        let text = format!("s\n{}", format!("{}\n", "x".repeat(1_025)).repeat(20_000));
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));
        let original = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut inlay_map, original) = InlayMap::new(original);
        let (mut fold_map, original) = FoldMap::new(original);
        let tab_size = NonZeroU32::new(4).expect("nonzero tab size");
        let (mut tab_map, original) = TabMap::new(original, tab_size);
        let mut cache = cx.update(|cx| {
            LongRows::new(
                cx.text_system().line_wrapper(test_font(), FONT_SIZE),
                cx.text_system().clone(),
                original.version,
            )
        });
        cache.set_shaping_source(source.clone());
        let detached = cache.detached(cx.background_executor.clone());
        let task = cx.background_executor.spawn({
            let original = original.clone();
            async move { detached.shape_row(&original, 1) }
        });
        cx.run_until_parked();
        let row = task.now_or_never().flatten().expect("ready row");
        assert_eq!(row.width(), 1_025. * 7.);
        cache.publish_row(&original, 1, row.clone());
        let mut sync = |cx: &mut TestAppContext| {
            let snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
            let edits = subscription.consume().into_inner();
            let (snapshot, edits) = inlay_map.sync(snapshot, edits);
            let (snapshot, edits) = fold_map.read(snapshot, edits);
            tab_map.sync(snapshot, edits, tab_size)
        };
        *cache.rows.lock() = TreeMap::from_ordered_entries((1..=20_000).map(|index| {
            (
                index,
                CachedRow {
                    generation: Arc::downgrade(&cache.identity),
                    width: row.width(),
                    geometry: Arc::downgrade(&row.geometry),
                },
            )
        }));
        cache.publish_max_width(1_025. * 7.);
        row.geometry
            .cached_row_clones
            .store(0, atomic::Ordering::Relaxed);
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 0)..Point::new(0, 1), "t")], None, cx);
        });
        let (edited, edits) = sync(cx);
        let retained = cache.interpolate(&edited, &edits);
        assert_eq!(
            row.geometry
                .cached_row_clones
                .load(atomic::Ordering::Relaxed),
            0
        );
        assert_eq!(retained.rows.lock().iter().count(), 20_000);
        assert_eq!(retained.try_max_width(&edited, &[]), Some(1_025. * 7.));
        for index in [1, 20_000] {
            let ready = retained.row(&edited, index).expect("retained row");
            assert!(Arc::ptr_eq(&ready.geometry, &row.geometry));
        }
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 0)..Point::new(0, 0), "\n")], None, cx);
        });
        let (shifted, edits) = sync(cx);
        let cleared = retained.interpolate(&shifted, &edits);
        assert_eq!(
            row.geometry
                .cached_row_clones
                .load(atomic::Ordering::Relaxed),
            0
        );
        assert!(cleared.rows.lock().is_empty());
        assert_eq!(cleared.try_max_width(&shifted, &[]), Some(1_025. * 7.));
        assert_eq!(row.widths.shaped_rows.load(atomic::Ordering::Relaxed), 1);
        assert_eq!(cache.rows.lock().iter().count(), 20_000);
    }

    #[gpui::test]
    async fn test_long_rows_track_trailing_whitespace(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map(&format!("x{}", " ".repeat(10_000)), 4, cx);
        let trailing_whitespace_start = |map: &Entity<DisplayMap>, cx: &mut TestAppContext| {
            cx.run_until_parked();
            ready_snapshot(&map, cx)
                .long_row(DisplayRow(0))
                .expect("row geometry ready")
                .map(|long_row| long_row.trailing_whitespace_start())
        };
        assert_eq!(trailing_whitespace_start(&map, cx), Some(1));

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(MultiBufferOffset(5_000)..MultiBufferOffset(5_000), "y")],
                None,
                cx,
            )
        });
        assert_eq!(trailing_whitespace_start(&map, cx), Some(5_001));

        let row_end = buffer.read_with(cx, |buffer, cx| {
            buffer.snapshot(cx).anchor_after(MultiBufferOffset(10_002))
        });
        let hint = Inlay::mock_hint(0, row_end, "hint ".to_string());
        map.update(cx, |map, cx| map.splice_inlays(&[], vec![hint], cx));
        assert_eq!(trailing_whitespace_start(&map, cx), Some(10_007));
    }

    #[gpui::test]
    async fn test_row_windows_keep_styles_from_before_their_start(cx: &mut TestAppContext) {
        init_settings(128, cx);
        let text = format!("abc\n{}\t{}", "x".repeat(79), "y".repeat(3_000));
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        buffer.update(cx, |buffer, cx| {
            let diagnostics = DiagnosticSet::new(
                [DiagnosticEntry::new(
                    PointUtf16::new(0, 0)..PointUtf16::new(0, 3),
                    Diagnostic {
                        severity: lsp::DiagnosticSeverity::ERROR,
                        group_id: 1,
                        message: "error".into(),
                        ..Diagnostic::default()
                    },
                )],
                buffer,
            );
            buffer.update_diagnostics(LanguageServerId(0), diagnostics, cx)
        });
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let map = new_display_map(&buffer, cx);
        let hint = Inlay::mock_hint(
            0,
            buffer_snapshot.anchor_after(MultiBufferOffset(1)),
            "h".repeat(4_096),
        );
        map.update(cx, |map, cx| {
            map.splice_inlays(&[], vec![hint], cx);
            map.highlight_text(
                HighlightKey::Editor,
                vec![
                    buffer_snapshot.anchor_before(Point::new(1, 79))
                        ..buffer_snapshot.anchor_after(Point::new(1, 80)),
                ],
                HighlightStyle::color(gpui::red()),
                false,
                cx,
            );
        });
        cx.run_until_parked();

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let style = EditorStyle::default();
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: true,
        };
        let inlay_window = merge_chunks(
            snapshot
                .highlighted_chunks_in_row(DisplayRow(0), 100..350, language_aware, &style)
                .map(|chunk| (chunk.text, chunk.diagnostic_underline_severity)),
        );
        assert_eq!(
            inlay_window,
            [("h".repeat(250), Some(lsp::DiagnosticSeverity::ERROR))]
        );
        let tab_window = merge_chunks(
            snapshot
                .highlighted_chunks_in_row(DisplayRow(1), 90..110, language_aware, &style)
                .map(|chunk| (chunk.text, chunk.style.and_then(|style| style.color))),
        );
        assert_eq!(tab_window, [(" ".repeat(20), Some(gpui::red()))]);
    }

    #[test]
    fn test_long_row_positions_match_whole_line_shaping_after_edits() {
        let cx = &mut shaping_context();
        let mut state = 7_u64;
        let random_text = (0..1_500)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                ['A', 'V', 'f', 'i', 'x'][(state >> 33) as usize % 5]
            })
            .collect::<String>();
        let ligature_runs = (1..30)
            .map(|run| format!("x{}", "w".repeat(run * 7 % 101 + 20)))
            .collect::<String>();
        let forced_runs = [0, 1].map(|lead| format!("{}{}", "x".repeat(lead), "w".repeat(1_501)));
        let reordered = format!("a\u{915}{}b", ShapingTextSystem::PRE_BASE_VOWEL).repeat(150);
        for text in [random_text, ligature_runs, reordered]
            .into_iter()
            .chain(forced_runs)
        {
            let (buffer, map) = build_display_map(&text, 4, cx);
            cx.run_until_parked();
            assert_matches_whole_line_shaping(&map, None, cx);
            for (offset, inserted) in [(0, "w"), (700, "w"), (3, "\u{301}w")] {
                buffer.update(cx, |buffer, cx| {
                    let offset = buffer.snapshot(cx).text().floor_char_boundary(offset);
                    buffer.edit(
                        [(
                            MultiBufferOffset(offset)..MultiBufferOffset(offset),
                            inserted,
                        )],
                        None,
                        cx,
                    )
                });
                cx.run_until_parked();
                assert_matches_whole_line_shaping(&map, None, cx);
            }
        }
    }

    #[test]
    fn test_visible_window_keeps_paintable_glyphs_and_shares_geometry() {
        let cx = &mut shaping_context();
        for (text, shared) in [
            ("x".repeat(2_048), true),
            (
                (b' '..=b'~').map(char::from).collect::<String>().repeat(16),
                true,
            ),
            ("AV".repeat(1_024), true),
            ("漢".repeat(1_024), true),
            ("e\u{301}".repeat(512), false),
            ("a\u{93f}".repeat(512), false),
            ("fi".repeat(1_024), false),
        ] {
            let row = shaped_long_row(&text, cx);
            let expected = row.widths.wrapper.layout_line(
                &text,
                &[FontRun {
                    len: text.len(),
                    font_id: row.widths.base_font,
                }],
            );
            let expected_glyphs = positioned_glyphs(&expected, 0, 0.);
            let geometry = row.geometry();
            let piece = &geometry.pieces[0];
            if shared {
                assert!(Arc::ptr_eq(
                    &geometry.atoms.columns,
                    &piece.index.partition_columns
                ));
                assert!(Arc::ptr_eq(
                    &geometry.atoms.positions,
                    &piece.index.positions
                ));
                assert!(piece.index.partition_glyphs.is_none());
            } else {
                assert!(!Arc::ptr_eq(
                    &geometry.atoms.positions,
                    &piece.index.positions
                ));
            }
            for alignment in [TextAlign::Left, TextAlign::Center, TextAlign::Right] {
                for scroll in [
                    0.,
                    8.,
                    f64::from(expected.width) / 2.,
                    f64::from(expected.width) - 1.,
                ] {
                    let viewport = 47.5;
                    let free = viewport - f64::from(expected.width);
                    let left = scroll
                        - match alignment {
                            TextAlign::Left => 0.,
                            TextAlign::Center => free / 2.,
                            TextAlign::Right => free,
                        };
                    let visible = |glyph: &(FontId, GlyphId, usize, f64, Pixels)| {
                        let bounds = row
                            .widths
                            .text_system
                            .bounding_box(glyph.0, expected.font_size);
                        let min_x = glyph.3 + f64::from(bounds.origin.x);
                        let max_x = min_x + f64::from(bounds.size.width);
                        min_x < left + viewport && max_x > left
                    };
                    let window = row.visible_window(scroll, alignment, viewport);
                    let actual = row
                        .window_layout(window, left)
                        .into_iter()
                        .flat_map(|piece| {
                            positioned_glyphs(&piece.layout, piece.columns.start as usize, piece.x)
                        })
                        .filter(&visible)
                        .collect::<Vec<_>>();
                    let expected = expected_glyphs
                        .iter()
                        .filter(|glyph| visible(glyph))
                        .cloned()
                        .collect::<Vec<_>>();
                    if text.is_ascii()
                        && text.starts_with('x')
                        && alignment == TextAlign::Left
                        && scroll == 8.
                    {
                        assert_eq!(expected.first().map(|glyph| glyph.2), Some(0));
                    }
                    assert_eq!(actual, expected, "{alignment:?}, scroll {scroll}");
                }
            }
        }
    }

    #[test]
    fn test_long_row_ink_overhang() {
        let cx = &mut ignored_marks_context();
        for (font, advance, ink) in [
            (
                Font {
                    style: FontStyle::Italic,
                    ..test_font()
                },
                7.,
                -28_f64..28.,
            ),
            (
                Font {
                    weight: FontWeight::BOLD,
                    ..test_font()
                },
                8.,
                28_f64..84.,
            ),
        ] {
            for compact in [false, true] {
                let text = format!("{}{}", if compact { "x" } else { "!" }, "x".repeat(2_047));
                let (_, map) = build_display_map(&text, 4, cx);
                map.update(cx, |map, cx| map.set_font(font.clone(), FONT_SIZE, cx));
                let row = test_long_row(&map, cx);
                assert_eq!(row.geometry().compact.is_some(), compact);
                assert_eq!(row.paint_ranges(700.0..700.0), Vec::new());
                assert_eq!(
                    row.geometry().paint_overhang,
                    (ink.start - 1.875).min(0.)..ink.end + 1.875
                );
                for left in [0., 700., row.width() + 14.] {
                    let visible =
                        |(_, x): &(u32, f64)| x + ink.start < left + 1. && x + ink.end > left;
                    let expected = (0..row.len())
                        .map(|column| (column, f64::from(column) * advance))
                        .filter(visible)
                        .collect::<Vec<_>>();
                    let window = row.visible_window(left, TextAlign::Left, 1.);
                    let actual = row
                        .window_layout(window, left)
                        .into_iter()
                        .flat_map(|piece| {
                            positioned_glyphs(&piece.layout, piece.columns.start as usize, piece.x)
                        })
                        .map(|(_, _, column, x, _)| (column as u32, x))
                        .filter(visible)
                        .collect::<Vec<_>>();
                    assert_eq!(actual, expected, "{ink:?}, compact {compact}, left {left}");
                }
                for scale in [1_f64, 1.25, 1.5, 2., 3., 4.] {
                    for left in [
                        100. * advance + ink.end + 0.25 / scale,
                        100. * advance + ink.start - 0.25 / scale - 10.,
                    ] {
                        let mask_left = 100. + 0.5 / scale;
                        let origin = mask_left - left;
                        let snapped_left = (mask_left * scale).floor();
                        let snapped_right = ((mask_left + 10.) * scale).ceil();
                        let visible = |(_, x): &(u32, f64)| {
                            (origin + x + ink.start) * scale < snapped_right
                                && (origin + x + ink.end) * scale > snapped_left
                        };
                        let expected = (0..row.len())
                            .map(|column| (column, f64::from(column) * advance))
                            .filter(visible)
                            .collect::<Vec<_>>();
                        let actual = row
                            .paint_ranges(LongRow::paint_window_bounds(
                                ScaledPixels(snapped_left as f32)
                                    ..ScaledPixels(snapped_right as f32),
                                origin,
                                scale as f32,
                            ))
                            .into_iter()
                            .flat_map(|columns| row.window_layout(columns, left))
                            .flat_map(|piece| {
                                positioned_glyphs(
                                    &piece.layout,
                                    piece.columns.start as usize,
                                    piece.x,
                                )
                            })
                            .map(|(_, _, column, x, _)| (column as u32, x))
                            .filter(visible)
                            .collect::<Vec<_>>();
                        assert_eq!(
                            actual.iter().map(|glyph| glyph.0).collect::<Vec<_>>(),
                            expected.iter().map(|glyph| glyph.0).collect::<Vec<_>>(),
                            "{ink:?}, compact {compact}, scale {scale}, left {left}"
                        );
                    }
                }
                for scale in [1_f32, 1.25, 1.5, 2., 3., 4.] {
                    for rendering_mode in
                        [TextRenderingMode::Grayscale, TextRenderingMode::Subpixel]
                    {
                        let config = GlyphRasterConfig {
                            scale_factor: scale,
                            rendering_mode,
                            max_dilation: 4,
                        };
                        map.update(cx, |map, cx| map.set_raster_config(config, cx));
                        let raster_row = test_long_row(&map, cx);
                        assert_eq!(raster_row.geometry.raster_config, config);
                        assert_eq!(raster_row.width(), row.width());
                        assert!(!Arc::ptr_eq(&raster_row.geometry, &row.geometry));
                        map.update(cx, |map, cx| map.set_raster_config(config, cx));
                        assert!(Arc::ptr_eq(
                            &raster_row.geometry,
                            &test_long_row(&map, cx).geometry
                        ));
                        let scale = f64::from(scale);
                        for dilation in [0, 4] {
                            let padding = 1.
                                + f64::from(dilation)
                                + f64::from(rendering_mode == TextRenderingMode::Subpixel);
                            for mask_left in [-100.5, 100.5] {
                                for edge in
                                    [ink.start - padding / scale - 10., ink.end + padding / scale]
                                {
                                    for phase in [0.0625, 0.3125, 0.8125] {
                                        let left = 100. * advance + edge + phase / scale;
                                        let origin = mask_left / scale - left;
                                        let mask =
                                            mask_left.floor()..(mask_left + 10. * scale).ceil();
                                        let visible = |column: u32| {
                                            let position =
                                                (origin + f64::from(column) * advance) * scale;
                                            let quantized = ((position.abs() * 4. - 0.5).ceil()
                                                / 4.)
                                                .copysign(position);
                                            let phase = if quantized >= 0. {
                                                (quantized.fract() * 4.) as u8
                                            } else {
                                                0
                                            };
                                            let start = quantized.trunc()
                                                + (ink.start * scale).floor()
                                                - padding;
                                            let end = quantized.trunc()
                                                + (ink.end * scale).ceil()
                                                + padding
                                                + f64::from(phase > 0);
                                            start < mask.end && end > mask.start
                                        };
                                        let expected = (0..raster_row.len())
                                            .filter(|column| visible(*column))
                                            .collect::<Vec<_>>();
                                        let actual = raster_row
                                            .paint_ranges(LongRow::paint_window_bounds(
                                                ScaledPixels(mask.start as f32)
                                                    ..ScaledPixels(mask.end as f32),
                                                origin,
                                                scale as f32,
                                            ))
                                            .into_iter()
                                            .flat_map(|columns| {
                                                raster_row.window_layout(columns, left)
                                            })
                                            .flat_map(|piece| {
                                                positioned_glyphs(
                                                    &piece.layout,
                                                    piece.columns.start as usize,
                                                    piece.x,
                                                )
                                            })
                                            .map(|glyph| glyph.2 as u32)
                                            .filter(|column| visible(*column))
                                            .collect::<Vec<_>>();
                                        assert_eq!(
                                            actual, expected,
                                            "{ink:?}, compact {compact}, {config:?}, dilation {dilation}, origin {origin}"
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_paint_index_matches_exhaustive_intersection() {
        let regions = (0..4_096_u32)
            .map(|index| {
                let start = f64::from((index * 2_651) % 4_099) - 2_050.;
                let width = if index % 17 == 0 {
                    8_192.
                } else {
                    f64::from(index % 7)
                };
                PaintRegion {
                    target: PaintTarget::Columns(index..index + 1),
                    bounds: start..start + width,
                }
            })
            .collect::<Vec<_>>();
        let paint = PaintIndex::new(regions.clone());
        let mut stack = vec![(0, 1)];
        let mut entries = 0;
        let mut nodes = 0;
        while let Some((index, depth)) = stack.pop() {
            if let Some(node) = paint.nodes.get(index) {
                entries += node.regions.len();
                nodes += 1;
                assert!(depth <= 1 + regions.len().ilog2() as usize);
                stack.extend([(node.left, depth + 1), (node.right, depth + 1)]);
            }
        }
        assert_eq!(entries, regions.len());
        assert_eq!(paint.ends.len(), entries);
        assert_eq!(paint.nodes.len(), nodes);
        assert!(nodes <= regions.len());
        for bounds in [
            -10_000.0..-9_999.0,
            -2_050.0..-2_050.0,
            -100.0..100.0,
            0.0..0.0,
            2_049.0..2_050.0,
            10_000.0..10_001.0,
        ] {
            let mut actual = Vec::new();
            let visits = paint.query(&bounds, |target| target.columns(&[], &bounds, &mut actual));
            actual.sort_unstable_by_key(|range| range.start);
            let expected = regions
                .iter()
                .filter(|region| {
                    region.bounds.start <= bounds.end && region.bounds.end >= bounds.start
                })
                .map(|region| match &region.target {
                    PaintTarget::Columns(columns) => columns.clone(),
                    _ => panic!("dependency region"),
                })
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
            assert!(visits <= 2 * (1 + regions.len().ilog2() as usize) + 2 * actual.len());
        }
    }

    #[test]
    fn test_ordered_paint_storage_depends_on_runs_not_glyphs() {
        for count in [4_096, 32_768] {
            for text in [
                "é".repeat(count),
                format!("a{}", "\u{301}".repeat(count - 1)),
            ] {
                for backstep in [false, true] {
                    let position = |index: usize| {
                        index as f64 * 8.
                            - if backstep && index == count / 2 {
                                12.
                            } else {
                                0.
                            }
                    };
                    let piece = retained_piece(
                        &text,
                        count as f64 * 8.,
                        text.char_indices().enumerate().map(|(index, (column, _))| {
                            (FontId(0), GlyphId(1), column, position(index), Pixels::ZERO)
                        }),
                    );
                    let mut regions = Vec::new();
                    piece.paint_regions(
                        0,
                        |_, _| {
                            FontPaintBounds::new(Bounds::new(
                                point(px(-2.), Pixels::ZERO),
                                size(px(8.), px(16.)),
                            ))
                        },
                        &mut regions,
                    );
                    let paint = PaintIndex::new(regions);
                    let expected_regions = if backstep { 5 } else { 2 };
                    assert_eq!(
                        (paint.regions.len(), paint.ends.len()),
                        (expected_regions, expected_regions)
                    );
                    assert!(paint.nodes.len() <= expected_regions);
                    assert_eq!(
                        paint
                            .regions
                            .iter()
                            .filter(|region| matches!(region.target, PaintTarget::Columns(_)))
                            .count(),
                        usize::from(backstep)
                    );
                    let origin = count as f64 * 4. + 0.25;
                    let actual = paint
                        .columns(slice::from_ref(&piece), origin..origin + 100.)
                        .into_iter()
                        .flat_map(|columns| {
                            let window = piece
                                .window(&columns, origin)
                                .expect("selected native glyphs");
                            positioned_glyphs(
                                &window.layout,
                                window.columns.start as usize,
                                window.x,
                            )
                            .into_iter()
                            .map(|glyph| (glyph.2, glyph.3))
                        })
                        .collect::<Vec<_>>();
                    let expected = text
                        .char_indices()
                        .enumerate()
                        .filter_map(|(index, (column, _))| {
                            let x = position(index);
                            let end = position(index + 1);
                            let left = x.min(end).min(x - 4.);
                            let right = x.max(end).max(x + 6.);
                            (left <= origin + 100. && right >= origin).then_some((column, x))
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    #[test]
    fn test_paint_dependencies_preserve_reordering_and_share_large_text() {
        for positions in [[-100., -50., 105., 200., 300.], [0., 80., 105., 200., 300.]] {
            let piece =
                retained_piece(
                    "abcde",
                    400.,
                    [0, 2, 1, 3, 4].into_iter().zip(positions).enumerate().map(
                        |(id, (column, x))| {
                            (FontId(id % 2), GlyphId(id as u32), column, x, px(id as f32))
                        },
                    ),
                );
            let mut regions = Vec::new();
            piece.paint_regions(
                0,
                |font, _| {
                    FontPaintBounds::new(Bounds::new(
                        point(px(-10.), Pixels::ZERO),
                        size(px(20. + font.0 as f32), px(16.)),
                    ))
                },
                &mut regions,
            );
            let paint = PaintIndex::new(regions);
            let columns = paint.columns(slice::from_ref(&piece), 100.0..110.0);
            assert_eq!(columns, [1..3]);
            let window = piece
                .window(&columns[0], 100.)
                .expect("selected dependency");
            assert_eq!(
                positioned_glyphs(&window.layout, 1, window.x),
                [
                    (FontId(1), GlyphId(1), 2, positions[1], px(1.)),
                    (FontId(0), GlyphId(2), 1, positions[2], px(2.)),
                ]
            );
        }
        let text = format!("a{}bc", "\u{301}".repeat(32 * MAX_LINE_LEN));
        let end = text.len() - 2;
        let piece = retained_piece(
            &text,
            48.,
            [(0, 0.), (end, 16.), (end + 1, 32.)]
                .map(|(column, x)| (FontId(0), GlyphId(1), column, x, Pixels::ZERO)),
        );
        let mut pieces = Vec::new();
        piece.split_large_text_dependencies(&mut pieces);
        let retained = pieces[0]
            .window(&(0..end as u32), 8.)
            .expect("large dependency");
        assert_eq!(retained.text.as_ptr(), pieces[0].text.as_ptr());
        assert_eq!(retained.layout.runs[0].glyphs.len(), 1);
        assert_eq!(retained.columns, 0..end as u32);
    }

    #[test]
    fn test_atom_cursor_preserves_reordered_indices_without_rewinding() {
        for columns in [&[][..], &[2][..], &[0, 4, 8, 12][..], &[2, 4, 8, 12][..]] {
            let mut cursor = AtomCursor::new(columns);
            let mut furthest = 0;
            for column in [1, 0, 9, 4, 12, 8, 12, 0, u32::MAX as usize, 0, 12] {
                let expected = columns.iter().rposition(|start| *start as usize <= column);
                assert_eq!(cursor.at(column), expected, "{columns:?}, {column}");
                furthest = furthest.max(expected.map_or(0, |index| index + 1));
                assert_eq!(cursor.end, furthest);
            }
            assert_eq!(cursor.end, columns.len());
        }
    }

    #[test]
    fn test_native_carets_preserve_independent_glyph_ink() {
        let cx = &mut shaping_context();
        let mut row = shaped_long_row(&"x".repeat(2 * MAX_LINE_LEN), cx);
        let mut piece = retained_piece(
            "a\u{301}fié",
            40.,
            [(0, -2., 0.), (1, 8., -3.), (3, 14., 1.), (5, 31., 0.)]
                .into_iter()
                .enumerate()
                .map(|(id, (column, x, y))| (FontId(0), GlyphId(id as u32), column, x, px(y))),
        );
        Arc::get_mut(&mut piece.layout)
            .expect("unique layout")
            .caret_positions = vec![(0, 0.), (3, 12.), (4, 17.), (5, 26.), (7, 40.)];
        let columns = vec![0, 3, 4, 5];
        let positions = atom_offsets_x(&piece.layout, &columns, &piece.index);
        assert_eq!(positions, [0., 12., 17., 26.]);
        assert_eq!(piece.index.partition_columns.as_ref(), [0, 1, 3, 5]);
        let atoms = AtomBuilder { columns, positions }.finish(Some(&piece));
        assert!(!Arc::ptr_eq(&atoms.positions, &piece.index.positions));
        let mut regions = Vec::new();
        piece.paint_regions(
            0,
            |_, _| FontPaintBounds {
                ink: 0.0..1.0,
                decoration_width: 1.,
            },
            &mut regions,
        );
        let paint = PaintIndex::new(regions);
        assert_eq!(paint.columns(slice::from_ref(&piece), -2.0..-1.5), [0..1]);
        assert_eq!(paint.columns(slice::from_ref(&piece), 27.0..28.0), [5..7]);
        let window = piece.window(&(4..5), 15.).expect("ligature dependency");
        assert_eq!(window.columns, 3..5);
        assert_eq!(window.text.as_str(), "fi");
        assert_eq!(window.layout.caret_positions, [(0, -3.), (1, 2.), (2, 11.)]);
        assert_eq!(window.layout.width, px(11.));
        assert_eq!(
            positioned_glyphs(&window.layout, 0, 0.),
            [(FontId(0), GlyphId(2), 0, -1., px(1.))],
        );
        let window = piece.window(&(0..3), 0.).expect("combining glyphs");
        assert_eq!(
            positioned_glyphs(&window.layout, 0, 0.),
            [
                (FontId(0), GlyphId(0), 0, -2., Pixels::ZERO),
                (FontId(0), GlyphId(1), 1, 8., px(-3.)),
            ],
        );
        let window = piece.window(&(1..3), 5.).expect("missing native boundary");
        assert_eq!(window.layout.caret_positions, [(0, 7.), (2, 7.)]);
        assert_eq!(
            positioned_glyphs(&window.layout, 0, 0.),
            [(FontId(0), GlyphId(1), 0, 3., px(-3.))],
        );
        row.geometry = Arc::new(RowGeometry {
            len: 7,
            width: 40.,
            content_end: 7,
            atoms,
            pieces: vec![piece],
            replacement_glyphs: Vec::new(),
            paint,
            paint_overhang: -2.0..9.0,
            raster_config: row.geometry.raster_config,
            font_runs: row.geometry.font_runs.clone(),
            metrics: row.widths.metrics,
            compact: None,
            plain: false,
            drop_executor: None,
            cached_row_clones: AtomicUsize::new(0),
        });
        for (column, x) in [(0, 0.), (1, 12.), (3, 12.), (4, 17.), (5, 26.), (7, 40.)] {
            assert_eq!(row.x_for_column(column), x);
        }
        for (x, column, closest) in [
            (-3., 0, 0),
            (6., 0, 0),
            (7., 0, 3),
            (12., 3, 3),
            (16., 3, 4),
            (17., 4, 4),
            (22., 4, 5),
            (26., 5, 5),
            (39., 5, 7),
            (40., 7, 7),
        ] {
            assert_eq!(row.column_at_x(x), column);
            assert_eq!(row.closest_column_for_x(x), closest);
        }
    }

    #[test]
    fn test_retained_slice_preserves_precise_positions_and_glyph_dependencies() {
        for origin in [16_777_216., 1_099_511_627_776.] {
            let dependency_text = format!("a{}bc", "\u{301}".repeat(MAX_LINE_LEN));
            let dependency_end = dependency_text.len() - 2;
            let positions = [0.125, origin + 0.125, origin + 0.5];
            let mut piece = retained_piece(
                &dependency_text,
                origin + 1.625,
                [0, dependency_end, dependency_end + 1]
                    .into_iter()
                    .zip(positions)
                    .map(|(index, x)| (FontId(0), GlyphId(1), index, x, Pixels::ZERO)),
            );
            Arc::get_mut(&mut piece.layout)
                .expect("unique layout")
                .caret_positions = vec![
                (0, 0.),
                (dependency_end, origin + 0.25),
                (dependency_end + 1, origin + 1.),
                (dependency_text.len(), origin + 1.625),
            ];
            let mut pieces = Vec::new();
            piece.split_large_text_dependencies(&mut pieces);
            assert_eq!(pieces.len(), 2);
            assert_eq!(
                pieces[0].layout.caret_positions,
                [(0, 0.), (dependency_end, origin + 0.25)]
            );
            assert_eq!(
                pieces[1].layout.caret_positions,
                [(0, origin + 0.25), (1, origin + 1.), (2, origin + 1.625)]
            );
            for (piece, expected) in pieces.iter().zip([&positions[..1], &positions[1..]]) {
                let left = piece.window(&piece.columns, 0.).expect("piece origin");
                assert_eq!(left.x, 0.);
                let window = piece
                    .window(&piece.columns, origin + 0.5)
                    .expect("retained text");
                assert_eq!(window.text, piece.text);
                if piece.columns.start == 0 {
                    assert_eq!(window.text.as_ptr(), piece.text.as_ptr());
                } else {
                    assert_eq!(
                        window.layout.caret_positions,
                        [(0, -0.25), (1, 0.5), (2, 1.125)]
                    );
                }
                assert_eq!(
                    window
                        .layout
                        .runs
                        .iter()
                        .flat_map(|run| &run.glyphs)
                        .map(|glyph| glyph.position.x)
                        .collect::<Vec<_>>(),
                    expected
                        .iter()
                        .map(|x| Pixels::from(x - window.x))
                        .collect::<Vec<_>>(),
                );
            }
            let compact_positions = [0., origin + 0.125, origin + 0.5, origin + 1.375];
            let compact = CompactLineLayout {
                font_id: FontId(2),
                font_size: FONT_SIZE,
                ascent: px(12.),
                descent: px(3.),
                width: origin + 1.625,
                is_emoji: true,
                glyphs: vec![1, 7, 19, u16::MAX],
                positions: CompactGlyphPositions::new(compact_positions.len(), |range, output| {
                    output.copy_from_slice(&compact_positions[range]);
                    Some(())
                })
                .expect("native positions"),
            };
            let layout = materialize_compact(&compact, 1..4, origin + 0.5);
            assert_eq!((layout.len, layout.width), (3, px(1.125)));
            assert_eq!(
                layout.caret_positions,
                [(0, -0.375), (1, 0.), (2, 0.875), (3, 1.125)]
            );
            assert_eq!((layout.ascent, layout.descent), (px(12.), px(3.)));
            assert_eq!(
                positioned_glyphs(&layout, 0, 0.),
                [
                    (FontId(2), GlyphId(7), 0, -0.375, Pixels::ZERO),
                    (FontId(2), GlyphId(19), 1, 0., Pixels::ZERO),
                    (
                        FontId(2),
                        GlyphId(u32::from(u16::MAX)),
                        2,
                        0.875,
                        Pixels::ZERO
                    ),
                ]
            );
            assert!(
                layout
                    .runs
                    .iter()
                    .flat_map(|run| &run.glyphs)
                    .all(|glyph| glyph.is_emoji)
            );
            let positions = [
                0.125,
                origin + 0.125,
                origin + 0.5,
                origin - 0.125,
                origin + 0.875,
                origin + 1.375,
            ];
            let width = origin + 1.625;

            let piece_for_indices = |text: &str, indices: [usize; 6]| {
                retained_piece(
                    text,
                    width,
                    indices
                        .into_iter()
                        .zip(positions)
                        .enumerate()
                        .map(|(id, (index, x))| {
                            (FontId(id / 3), GlyphId(id as u32), index, x, px(id as f32))
                        }),
                )
            };
            let mut piece = piece_for_indices("éabçdefg", [0, 4, 2, 4, 6, 8]);
            piece.columns = 4_096..4_106;
            piece.source_start_column = 4_096;
            piece.x = 123.5;
            let index = &piece.index;
            assert_eq!(
                index
                    .partition_columns
                    .iter()
                    .map(|column| *column as usize)
                    .chain([index.len])
                    .collect::<Vec<_>>(),
                [0, 2, 6, 8, 10]
            );
            assert_eq!(
                atom_offsets_x(&piece.layout, &[0, 2, 6, 7, 8, 9], index),
                [
                    0.125,
                    origin - 0.125,
                    origin + 0.875,
                    origin + 1.125,
                    origin + 1.375,
                    origin + 1.5,
                ]
            );

            let sliced = piece
                .window(&(4_099..4_101), 123.5 + origin + 0.5)
                .expect("visible dependency");
            assert_eq!(sliced.columns, 4_098..4_102);
            assert_eq!(sliced.text.as_str(), "abç");
            assert_eq!(sliced.x, 123.5 + origin + 0.5);
            assert_eq!(sliced.layout.width, px(0.375));
            assert_eq!(sliced.layout.len, 4);

            assert_eq!(
                positioned_glyphs(&sliced.layout, 0, 0.),
                [
                    (FontId(0), GlyphId(1), 2, -0.375, px(1.)),
                    (FontId(0), GlyphId(2), 0, 0., px(2.)),
                    (FontId(1), GlyphId(3), 2, -0.625, px(3.)),
                ]
            );
            let end = piece.window(&(4_104..4_105), 0.).expect("last glyph");
            assert_eq!(
                (end.x, end.layout.width),
                (0., Pixels::from(123.5 + origin + 1.625))
            );
            assert_eq!(end.columns, 4_104..4_106);
            assert!(piece.window(&(4_106..4_107), 0.).is_none());

            let text = "éabçdefg".repeat(16);
            let piece = piece_for_indices(&text, [0; 6]);
            let sliced = piece.window(&(3..5), 0.).expect("indivisible dependency");
            assert_eq!(sliced.columns, 0..text.len() as u32);
            assert!(Arc::ptr_eq(&piece.layout, &sliced.layout));
            let rebased = piece
                .window(&(3..5), origin + 0.125)
                .expect("visible glyphs");
            assert_eq!(rebased.columns, 0..text.len() as u32);
            assert_eq!(rebased.text.as_str().as_ptr(), piece.text.as_str().as_ptr());
            assert_eq!(rebased.x, origin + 0.125);
            assert_eq!(rebased.layout.width, px(1.5));
            assert_eq!(
                positioned_glyphs(&rebased.layout, 0, 0.)
                    .into_iter()
                    .skip(1)
                    .map(|glyph| (glyph.2, glyph.3))
                    .collect::<Vec<_>>(),
                [(0, 0.), (0, 0.375), (0, -0.25), (0, 0.75), (0, 1.25)]
            );
            let end = piece
                .window(&(3..5), width + 10.)
                .expect("offscreen dependency");
            assert_eq!((end.x, end.layout.width), (width + 10., px(-10.)));
            assert_eq!(end.x + f64::from(end.layout.width), width);
            assert_eq!(
                end.layout
                    .runs
                    .iter()
                    .flat_map(|run| &run.glyphs)
                    .map(|glyph| glyph.position.x)
                    .collect::<Vec<_>>(),
                positions.map(|position| Pixels::from(position - (width + 10.))),
            );
        }
    }

    #[test]
    fn test_long_rows_keep_same_font_run_boundaries() {
        let padding = "x".repeat(10_000);
        let text = format!("{padding}fi{padding}");
        let split = padding.len() + 1;
        for highlight in [
            HighlightStyle::color(gpui::red()),
            HighlightStyle {
                underline: Some(UnderlineStyle {
                    thickness: px(1.),
                    color: None,
                    wavy: false,
                }),
                ..HighlightStyle::default()
            },
            HighlightStyle {
                strikethrough: Some(StrikethroughStyle {
                    thickness: px(1.),
                    color: None,
                }),
                ..HighlightStyle::default()
            },
        ] {
            let cx = &mut shaping_context();
            let (buffer, map) = build_display_map(&text, 4, cx);
            set_test_shaping_style(&map, cx);
            let snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
            map.update(cx, |map, cx| {
                map.highlight_text(
                    HighlightKey::Editor,
                    vec![
                        snapshot.anchor_after(MultiBufferOffset(0))
                            ..snapshot.anchor_before(MultiBufferOffset(split)),
                    ],
                    highlight,
                    false,
                    cx,
                );
            });
            assert_matches_whole_line_shaping(
                &map,
                Some(&[0..split, split..text.len()].map(|columns| FontRun {
                    len: columns.len(),
                    font_id: ShapingTextSystem::REGULAR,
                })),
                cx,
            );
        }
    }

    #[test]
    fn test_font_run_cursor_preserves_boundaries_across_skipped_spans() {
        let runs = [FontId(0), FontId(0), FontId(1)].map(|font_id| FontRun { len: 4, font_id });
        let mut fonts = FontRunsCursor::new(&runs);
        for (columns, expected) in [
            (0..0, Vec::new()),
            (0..3, vec![(FontId(0), 3)]),
            (3..5, vec![(FontId(0), 1), (FontId(0), 1)]),
            (9..12, vec![(FontId(1), 3)]),
            (12..12, Vec::new()),
        ] {
            assert_eq!(
                fonts
                    .slice(columns)
                    .into_iter()
                    .map(|run| (run.font_id, run.len))
                    .collect::<Vec<_>>(),
                expected,
            );
        }
    }

    #[test]
    fn test_chunk_seams_preserve_contextual_substitutions_beyond_lookahead() {
        for (seam, context) in [(64, 9), (8_192, 65)] {
            let cx = &mut ignored_marks_context();
            let text = format!(
                "{}Q{}Z{}",
                "x".repeat(seam - 1),
                "x".repeat(context),
                "x".repeat(MAX_LINE_LEN)
            );
            let long_row = shaped_long_row(&text, cx);
            let wrapper = &long_row.widths.wrapper;
            let advance = f64::from(FONT_SIZE * ShapingTextSystem::ADVANCE);
            let expected_width = advance * (text.len() + 1) as f64;
            let expected = wrapper.layout_line(
                &text,
                &[FontRun {
                    len: text.len(),
                    font_id: wrapper.font_id(),
                }],
            );
            assert_eq!(f64::from(expected.width), expected_width);
            assert_eq!(
                positioned_glyphs(&expected, 0, 0.),
                (0..text.len())
                    .map(|column| (
                        wrapper.font_id(),
                        GlyphId(if column == seam - 1 { 9 } else { 1 }),
                        column,
                        advance * (column + usize::from(column >= seam)) as f64,
                        Pixels::ZERO,
                    ))
                    .collect::<Vec<_>>(),
            );
            assert_eq!(
                (long_row.width(), long_row.x_for_column(seam as u32)),
                (expected_width, advance * (seam + 1) as f64),
            );
            assert_window_matches_layout(&long_row, 0..long_row.len(), 0., &expected);
            let window = long_row.chunk_aligned(seam as u32 - 1..seam as u32 + 2);
            assert_window_matches_layout(&long_row, window, advance * seam as f64, &expected);
        }
    }

    #[test]
    fn test_ignored_marks_keep_complete_run_shaping() {
        let cx = &mut ignored_marks_context();
        for marks in [16, 512, 16_385] {
            let text = format!(
                "{}f{}i{}",
                "x".repeat(63),
                "\u{34f}".repeat(marks),
                "x".repeat(1_024)
            );
            let row = shaped_long_row(&text, cx);
            let wrapper = &row.widths.wrapper;
            let expected = wrapper.layout_line(
                &text,
                &[FontRun {
                    len: text.len(),
                    font_id: wrapper.font_id(),
                }],
            );
            assert_eq!(
                expected.width,
                FONT_SIZE * ShapingTextSystem::ADVANCE * (63. + 1.5 + 1_024.)
            );
            assert_eq!(
                expected
                    .runs
                    .iter()
                    .flat_map(|run| &run.glyphs)
                    .filter(|glyph| glyph.index == 63)
                    .map(|glyph| glyph.id)
                    .collect::<Vec<_>>(),
                [GlyphId(2)],
            );
            assert_eq!(row.width(), f64::from(expected.width));
            assert_window_matches_layout(&row, 0..row.len(), 0., &expected);
            if marks > MAX_LINE_LEN {
                let piece = row
                    .geometry
                    .pieces
                    .iter()
                    .find(|piece| piece.text.len() > MAX_LINE_LEN)
                    .expect("retained dependency");
                let columns = piece.columns.start + 1..piece.columns.end - 1;
                let window = row.window_layout(columns, 123.);
                assert_eq!(window.len(), 1);
                assert_eq!(window[0].text.as_ptr(), piece.text.as_ptr());
                assert_eq!(window[0].columns, piece.columns);
            }
        }
        let text = format!("\u{34f}{}", "fi".repeat(1_024));
        let row = shaped_long_row(&text, cx);
        assert_long_row_matches_whole_line_shaping(&row, &text, None, cx);
        assert_eq!(
            row.window_layout(0..row.len(), 0.)
                .iter()
                .map(|piece| (piece.columns.clone(), piece.text.as_str()))
                .collect::<Vec<_>>(),
            [(0..row.len(), text.as_str())],
        );
        let row = shaped_long_row(&"\u{34f}".repeat(2_048), cx);
        assert_eq!(row.width(), 0.);
        assert_eq!(row.x_for_column(row.len() / 2), 0.);
        assert_eq!(row.column_at_x(0.), 0);
        assert_eq!(row.column_at_x(1.), row.len());
        let window = row.visible_window(0., TextAlign::Left, 100.);
        assert_eq!(window, 0..row.len());
        let layout = row.window_layout(0..row.len(), 0.);
        assert_eq!(
            layout
                .iter()
                .flat_map(|piece| piece.layout.runs.iter())
                .count(),
            0
        );
    }

    #[test]
    fn test_retained_underline_origin_across_ligatures_and_source_fragments() {
        let cx = &mut ignored_marks_context();
        let text = format!("fia{}bcd", "\u{034f}".repeat(1_024));
        let row = shaped_long_row(&text, cx);
        assert_eq!(
            row.decoration_positions()
                .map(|(column, x, _, _)| (column, x))
                .collect::<Vec<_>>(),
            [(0, 0.), (2, 10.5), (2051, 17.5), (2052, 24.5), (2053, 31.5)]
        );
        assert_eq!(
            row.geometry
                .pieces
                .iter()
                .map(|piece| piece.columns.clone())
                .collect::<Vec<_>>(),
            [0..2, 2..2051, 2051..2054]
        );
        let style = TextStyle {
            font_family: test_font().family,
            font_size: AbsoluteLength::from(FONT_SIZE),
            color: Hsla::from(rgba(0x112233ff)),
            ..TextStyle::default()
        };
        let primary = UnderlineStyle {
            thickness: px(1.),
            color: Some(Hsla::from(rgba(0xff0000ff))),
            wavy: true,
        };
        let ignored = UnderlineStyle {
            color: Some(Hsla::from(rgba(0x00ff00ff))),
            ..primary
        };
        let spans = [
            (0..1, None),
            (1..5, Some(primary)),
            (5..7, Some(ignored)),
            (7..text.len(), Some(primary)),
        ];
        let prepared =
            PreparedRowText::new(spans.iter().map(|(columns, underline)| HighlightedChunk {
                text: &text[columns.clone()],
                style: underline.map(|underline| HighlightStyle {
                    underline: Some(underline),
                    ..HighlightStyle::default()
                }),
                diagnostic_underline_severity: None,
                is_tab: false,
                is_inlay: false,
                replacement: None,
            }));
        prepared.prepare_cursor_widths(&row, &style, &row.widths.wrapper, &row.widths.text_system);
        let layout = row.widths.wrapper.layout_line(
            &text,
            &[FontRun {
                len: text.len(),
                font_id: row.widths.wrapper.font_id(),
            }],
        );
        let runs = spans
            .iter()
            .map(|(columns, underline)| TextRun {
                len: columns.len(),
                font: style.font(),
                color: style.color,
                underline: *underline,
                ..TextRun::default()
            })
            .collect::<Vec<_>>();
        let full = ShapedLine::new(Arc::new(layout), SharedString::new(&text), &runs);
        for scroll in [0., 8., 16., 24.] {
            let full = full.clone();
            let pieces = row
                .paint_ranges(scroll..scroll + 14.)
                .into_iter()
                .flat_map(|columns| row.window_layout(columns, scroll))
                .map(|piece| {
                    let (runs, origins) = prepared
                        .styles(
                            piece.columns.start as usize..piece.columns.end as usize,
                            &piece.layout,
                        )
                        .expect("prepared paint runs");
                    (
                        piece.x,
                        ShapedLine::new(piece.layout, piece.text, &runs)
                            .with_underline_origins(piece.x, origins),
                    )
                })
                .collect::<Vec<_>>();
            let window = cx.open_window(size(px(14.), px(40.)), move |_, _| {
                UnderlinePaintView(Rc::new(move |window, cx| {
                    let mut full_origins = Vec::new();
                    full.paint_with_underline_handler(
                        point(px(-scroll as f32), px(0.)),
                        px(20.),
                        TextAlign::Left,
                        None,
                        window,
                        cx,
                        |_, underline, _| full_origins.push(underline.origin.x),
                    )
                    .unwrap();
                    assert_eq!(full_origins, [px(10.5 - scroll as f32)]);
                    for (x, piece) in &pieces {
                        piece
                            .paint(
                                point(px((x - scroll) as f32), px(0.)),
                                px(20.),
                                TextAlign::Left,
                                None,
                                window,
                                cx,
                            )
                            .unwrap();
                    }
                }))
            });
            for scale in [1., 1.25, 2.] {
                cx.simulate_window_scale_factor_change(window.into(), scale);
                let underlines = cx
                    .update_window(window.into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        window.painted_underlines()
                    })
                    .unwrap();
                assert!(!underlines.is_empty());
                let scale_quarters = (scale * 4.) as i64;
                let numerator = (21 - 2 * scroll as i64) * scale_quarters;
                let expected_origin = (numerator.abs() + 3) / 8 * numerator.signum();
                let period = 9 * ((scale_quarters + 1) / 4).max(1);
                for underline in underlines {
                    assert_eq!(
                        (underline.bounds.left().0 as i64 - expected_origin).rem_euclid(period),
                        0,
                        "scroll={scroll}, scale={scale}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_retained_underline_origins_match_reordered_cluster_painting() {
        let cx = &mut shaping_context();
        let mut row = shaped_long_row(&"x".repeat(2 * MAX_LINE_LEN), cx);
        let text = "किx";
        let font_id = row.widths.wrapper.font_id();
        let mut piece = retained_piece(
            text,
            24.,
            [(3, 0.), (0, 8.), (6, 16.)]
                .map(|(index, x)| (font_id, GlyphId(1), index, x, Pixels::ZERO)),
        );
        let layout = Arc::get_mut(&mut piece.layout).expect("unique layout");
        layout.font_size = FONT_SIZE;
        layout.caret_positions = vec![(0, 0.), (6, 16.), (7, 24.)];
        let layout = piece.layout.clone();
        row.geometry = Arc::new(RowGeometry {
            len: 7,
            width: 24.,
            content_end: 7,
            atoms: AtomBuilder {
                columns: vec![0, 6],
                positions: vec![0., 16.],
            }
            .finish(Some(&piece)),
            pieces: vec![piece],
            replacement_glyphs: Vec::new(),
            paint: PaintIndex::default(),
            paint_overhang: 0.0..0.0,
            raster_config: row.geometry.raster_config,
            font_runs: Arc::from([FontRun { len: 7, font_id }]),
            metrics: row.widths.metrics,
            compact: None,
            plain: false,
            drop_executor: None,
            cached_row_clones: AtomicUsize::new(0),
        });
        let style = TextStyle {
            font_family: test_font().family,
            font_size: AbsoluteLength::from(FONT_SIZE),
            color: Hsla::red(),
            underline: Some(UnderlineStyle {
                thickness: px(1.),
                color: None,
                wavy: true,
            }),
            ..TextStyle::default()
        };
        let spans = [(0..3, Hsla::green()), (3..7, Hsla::blue())];
        let prepared =
            PreparedRowText::new(spans.iter().map(|(columns, background)| HighlightedChunk {
                text: &text[columns.clone()],
                style: Some(HighlightStyle {
                    background_color: Some(*background),
                    ..HighlightStyle::default()
                }),
                diagnostic_underline_severity: None,
                is_tab: false,
                is_inlay: false,
                replacement: None,
            }));
        prepared.prepare_cursor_widths(&row, &style, &row.widths.wrapper, &row.widths.text_system);
        let (_, origins) = prepared
            .styles(0..text.len(), &layout)
            .expect("prepared paint runs");
        let runs = spans
            .iter()
            .map(|(columns, background)| TextRun {
                len: columns.len(),
                font: style.font(),
                color: style.color,
                background_color: Some(*background),
                underline: style.underline,
                ..TextRun::default()
            })
            .collect::<Vec<_>>();
        let full = ShapedLine::new(layout, SharedString::new_static(text), &runs);
        let window = cx.open_window(size(px(24.), px(40.)), move |_, _| {
            UnderlinePaintView(Rc::new(move |window, cx| {
                let mut painted_origins = Vec::new();
                full.paint_with_underline_handler(
                    point(px(0.), px(0.)),
                    px(20.),
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                    |columns, underline, _| {
                        painted_origins.push((columns, f64::from(underline.origin.x)));
                    },
                )
                .expect("paint underlines");
                assert_eq!(painted_origins, [(0..3, 0.), (3..7, 16.)]);
                assert_eq!(origins, painted_origins);
            }))
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("draw underline comparison");
    }

    #[test]
    fn test_compact_row_geometry_matches_native_output() {
        let cx = &mut ignored_marks_context();
        let text = format!(
            "{}Q{}Z{}",
            "x".repeat(1_023),
            "x".repeat(9),
            "x".repeat(1_025)
        );
        for font_size in [FONT_SIZE, px(14.000001)] {
            let (_, map) = build_display_map(&text, 4, cx);
            map.update(cx, |map, cx| map.set_font(test_font(), font_size, cx));
            let row = test_long_row(&map, cx);
            let snapshot = ready_snapshot(&map, cx);
            let input = RowInput::new(snapshot.tab_snapshot(), 0).expect("measured row input");
            let input_pointer = match input.runs.as_slice() {
                [InputRun::Text { text, .. }] => text.as_ptr(),
                _ => panic!("expected one text run"),
            };
            let geometry = RowGeometry::shape(
                input,
                row.geometry.font_runs.clone(),
                &row.widths.wrapper,
                row.widths.metrics,
                &row.widths.text_system,
                row.geometry.raster_config,
            );
            let (retained_text, _) = geometry.compact.as_ref().expect("compact layout");
            assert_eq!(retained_text.as_ptr(), input_pointer);
            assert!(row.geometry.compact.is_some());
            assert!(row.geometry.atoms.columns.is_empty());
            assert!(row.geometry.pieces.is_empty());
            let (native, positions, width) = row
                .widths
                .wrapper
                .layout_line_with_precise_positions(&text, &row.geometry.font_runs);
            assert_eq!(row.width().to_bits(), width.to_bits());
            for column in [
                0,
                1,
                1_023,
                1_024,
                1_025,
                1_033,
                2_048,
                row.len() - 1,
                row.len(),
            ] {
                let x = positions.get(column as usize).copied().unwrap_or(width);
                assert_eq!(row.x_for_column(column).to_bits(), x.to_bits());
                assert_eq!(row.column_at_x(x), column);
                assert_eq!(row.closest_column_for_x(x), column);
                assert_eq!(
                    row.font_id_for_column(column, &(0..row.len())),
                    native.font_id_for_index(column as usize),
                );
            }
            let expected = positioned_glyphs(&native, 0, 0.);
            for alignment in [TextAlign::Left, TextAlign::Center, TextAlign::Right] {
                for scroll in [0., positions[1_023] + 0.5, width - 1., width + 10.] {
                    let viewport = 70.;
                    let left = scroll
                        - match alignment {
                            TextAlign::Left => 0.,
                            TextAlign::Center => (viewport - width) / 2.,
                            TextAlign::Right => viewport - width,
                        };
                    let visible = |glyph: &(FontId, GlyphId, usize, f64, Pixels)| {
                        let bounds = row.widths.text_system.bounding_box(glyph.0, font_size);
                        let min_x = glyph.3 + f64::from(bounds.origin.x);
                        min_x < left + viewport && min_x + f64::from(bounds.size.width) > left
                    };
                    let window = row.visible_window(scroll, alignment, viewport);
                    let actual = row
                        .window_layout(window, left)
                        .into_iter()
                        .flat_map(|piece| {
                            assert_eq!(
                                piece
                                    .layout
                                    .runs
                                    .iter()
                                    .flat_map(|run| &run.glyphs)
                                    .map(|glyph| glyph.position.x)
                                    .collect::<Vec<_>>(),
                                positions[piece.columns.start as usize..piece.columns.end as usize]
                                    .iter()
                                    .map(|x| Pixels::from(x - piece.x))
                                    .collect::<Vec<_>>(),
                            );
                            positioned_glyphs(&piece.layout, piece.columns.start as usize, piece.x)
                        })
                        .filter(&visible)
                        .collect::<Vec<_>>();
                    assert_eq!(
                        actual,
                        expected
                            .iter()
                            .filter(|glyph| visible(glyph))
                            .cloned()
                            .collect::<Vec<_>>()
                    );
                }
            }
        }
    }

    #[test]
    fn test_hit_testing_before_first_glyph() {
        let cx = &mut shaping_context();
        let mut row = shaped_long_row(&"x".repeat(2 * MAX_LINE_LEN), cx);
        row.geometry = Arc::new(RowGeometry {
            len: row.len(),
            width: 16.,
            compact: None,
            content_end: row.len(),
            atoms: AtomIndex {
                columns: Arc::from([0, 1]),
                positions: Arc::from([1., 8.]),
            },
            pieces: Vec::new(),
            replacement_glyphs: Vec::new(),
            paint: PaintIndex::default(),
            paint_overhang: 0.0..0.0,
            raster_config: row.geometry.raster_config,
            font_runs: row.geometry.font_runs.clone(),
            plain: false,
            metrics: row.widths.metrics,
            drop_executor: None,
            cached_row_clones: AtomicUsize::new(0),
        });
        for (x, expected) in [
            (-1., 0),
            (0., 0),
            (0.5, 0),
            (1., 0),
            (8., 1),
            (16., row.len()),
            (17., row.len()),
        ] {
            assert_eq!(row.column_at_x(x), expected);
            assert_eq!(row.closest_column_for_x(x), expected);
        }
        let font_id = row.widths.base_font;
        let mut piece = retained_piece(
            "xx",
            16.,
            [(0, 1.), (1, 8.)].map(|(index, x)| (font_id, GlyphId(1), index, x, Pixels::ZERO)),
        );
        piece.columns = 1..3;
        piece.x = 4.;
        piece.source_start_column = 1;
        row.geometry = Arc::new(RowGeometry {
            len: row.len(),
            width: 20.,
            compact: None,
            content_end: row.len(),
            atoms: AtomIndex {
                columns: Arc::from([0, 1, 2]),
                positions: Arc::from([0., 5., 12.]),
            },
            pieces: vec![piece],
            replacement_glyphs: Vec::new(),
            paint: PaintIndex::default(),
            paint_overhang: 0.0..0.0,
            raster_config: row.geometry.raster_config,
            font_runs: row.geometry.font_runs.clone(),
            plain: false,
            metrics: row.widths.metrics,
            drop_executor: None,
            cached_row_clones: AtomicUsize::new(0),
        });
        for x in [4., 4.5, 5.] {
            assert_eq!(row.column_at_x(x), 1);
            assert_eq!(row.closest_column_for_x(x), 1);
        }
        let mut piece = retained_piece(
            "abc",
            21.,
            [(FontId(0), 1), (FontId(1), 0), (FontId(1), 2)].map(|(font_id, index)| {
                (font_id, GlyphId(1), index, index as f64 * 7., Pixels::ZERO)
            }),
        );
        piece.columns = 1..4;
        Arc::get_mut(&mut row.geometry)
            .expect("private geometry")
            .pieces = vec![piece];
        assert_eq!(
            (0..5)
                .map(|column| row.font_id_for_column(column, &(0..row.len())))
                .collect::<Vec<_>>(),
            [
                None,
                Some(FontId(0)),
                Some(FontId(0)),
                Some(FontId(1)),
                None
            ],
        );
        for columns in [0..row.len(), 1..3, 2..3, 3..4] {
            let pieces = row.window_layout(columns.clone(), 0.);
            for column in columns.start..columns.end.min(5) {
                let expected = pieces
                    .iter()
                    .find(|piece| piece.columns.contains(&column))
                    .and_then(|piece| {
                        piece
                            .layout
                            .font_id_for_index((column - piece.columns.start) as usize)
                    });
                assert_eq!(row.font_id_for_column(column, &columns), expected);
            }
        }
    }

    #[test]
    fn test_restyles_keep_whole_line_shaping() {
        for (character, len, bold) in [("w", 20_001, 1..20_001), ("x", 20_000, 10_000..10_100)] {
            let cx = &mut shaping_context();
            cx.background_executor.set_block_on_ticks(0..=0);
            let text = character.repeat(len);
            let (buffer, map) = build_display_map(&text, 4, cx);
            set_test_shaping_style(&map, cx);
            let old = test_long_row(&map, cx);
            let old_width = old.width();
            let runs = [
                (0..bold.start, ShapingTextSystem::REGULAR),
                (bold.clone(), ShapingTextSystem::BOLD),
                (bold.end..len as u32, ShapingTextSystem::REGULAR),
            ]
            .map(|(columns, font_id)| FontRun {
                len: columns.len(),
                font_id,
            });
            let snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
            map.update(cx, |map, cx| {
                map.highlight_text(
                    HighlightKey::Editor,
                    vec![
                        snapshot.anchor_after(MultiBufferOffset(bold.start as usize))
                            ..snapshot.anchor_before(MultiBufferOffset(bold.end as usize)),
                    ],
                    HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        ..HighlightStyle::default()
                    },
                    false,
                    cx,
                );
            });
            assert_matches_whole_line_shaping(&map, Some(&runs), cx);
            cx.run_until_parked();
            assert_matches_whole_line_shaping(&map, Some(&runs), cx);
            assert_eq!(old.width(), old_width);
        }
    }

    #[test]
    fn test_contextual_edits_invalidate_complete_runs() {
        let cx = &mut ignored_marks_context();
        cx.background_executor.set_block_on_ticks(0..=0);
        let text = format!(
            "{}Q{}Y{}",
            "x".repeat(8_191),
            "x".repeat(65),
            "x".repeat(1_024)
        );
        let (buffer, map) = build_display_map(&text, 4, cx);
        let original = test_long_row(&map, cx);
        let advance = f64::from(FONT_SIZE * ShapingTextSystem::ADVANCE);
        assert_eq!(original.width(), text.len() as f64 * advance);
        for (suffix, extra) in [("Z", 1), ("Y", 0)] {
            buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(8_257)..MultiBufferOffset(8_258), suffix)],
                    None,
                    cx,
                )
            });
            let row = test_long_row(&map, cx);
            assert_eq!(row.width(), (text.len() + extra) as f64 * advance);
            assert_eq!(row.x_for_column(8_192), (8_192 + extra) as f64 * advance);
            assert_matches_whole_line_shaping(&map, None, cx);
            assert_eq!(original.width(), text.len() as f64 * advance);
        }
    }

    #[test]
    fn test_long_row_shaping_source_updates_offscreen() {
        for syntax_source in [true, false] {
            let mut cx = shaping_context();
            crate::editor_tests::init_test(&mut cx, |_| {});
            let text = format!("\"{}\"", "i".repeat(20_000));
            let language = languages::rust_lang();
            let buffer = cx.new(|cx| {
                Buffer::local(format!("{text}{}{text}", "\n".repeat(100)), cx)
                    .with_language(language.clone(), cx)
            });
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            let cx = cx.add_empty_window();
            let editor = cx.update(|window, cx| {
                cx.new(|cx| Editor::new(EditorMode::full(), buffer.clone(), None, window, cx))
            });
            let map = editor.read_with(cx, |editor, _| editor.display_map.clone());
            let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
            style.text.font_size = gpui::AbsoluteLength::from(px(16.));
            style.text.font_weight = FontWeight::NORMAL;
            editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx)
            });
            cx.run_until_parked();
            cx.background_executor.set_block_on_ticks(0..=0);
            for weight in [None, Some(FontWeight::BOLD), None] {
                let highlight = HighlightStyle {
                    font_weight: weight,
                    color: Some(gpui::blue()),
                    ..HighlightStyle::default()
                };
                style.syntax = Arc::new(theme::SyntaxTheme::new_test_styles([(
                    "string",
                    if syntax_source {
                        highlight
                    } else {
                        HighlightStyle::default()
                    },
                )]));
                language.set_theme(&style.syntax);
                if !syntax_source {
                    editor.update(cx, |editor, cx| {
                        editor.clear_highlights(HighlightKey::Editor, cx);
                        if weight.is_some() {
                            let snapshot = editor.buffer().read(cx).snapshot(cx);
                            editor.highlight_text(
                                HighlightKey::Editor,
                                [0, 100]
                                    .map(|row| {
                                        snapshot.anchor_after(Point::new(row, 0))
                                            ..snapshot
                                                .anchor_before(Point::new(row, text.len() as u32))
                                    })
                                    .to_vec(),
                                highlight,
                                cx,
                            );
                        }
                    });
                }
                cx.draw(point(px(0.), px(0.)), size(px(500.), px(500.)), |_, _| {
                    EditorElement::new(&editor, style.clone())
                });
                let snapshot = ready_snapshot(&map, cx);
                for row in [DisplayRow(0), DisplayRow(100)] {
                    let row = snapshot
                        .long_row(row)
                        .expect("row geometry ready")
                        .expect("long row");
                    assert_eq!(
                        row.width(),
                        text.len() as f64 * (8. + if weight.is_some() { 1. } else { 0. })
                    );
                }
            }
        }
    }

    #[test]
    fn test_long_row_deleted_diff_base_syntax() {
        let cx = &mut shaping_context();
        let text = format!("\"{}\"", "x".repeat(2_048));
        let (buffer, map) = build_display_map(&format!("keep\n{text}\n"), 4, cx);
        set_test_shaping_style(&map, cx);
        let language = languages::rust_lang();
        map.update(cx, |map, cx| {
            let mut style = map
                .long_row_source
                .as_ref()
                .expect("shaping source")
                .style
                .clone();
            style.syntax = Arc::new(theme::SyntaxTheme::new_test_styles([(
                "string",
                HighlightStyle {
                    font_weight: Some(FontWeight::BOLD),
                    ..HighlightStyle::default()
                },
            )]));
            language.set_theme(&style.syntax);
            map.set_shaping_style(&style, SyntaxStyling::default(), cx);
        });
        let main_buffer = buffer.read_with(cx, |buffer, _| {
            buffer.as_singleton().expect("singleton buffer")
        });
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(
                &format!("{text}\nkeep\n{text}\n"),
                &main_buffer.read(cx).text_snapshot(),
                cx,
            )
        });
        let base_buffer = diff.read_with(cx, |diff, _| diff.base_text_buffer().clone());
        buffer.update(cx, |buffer, cx| {
            buffer.set_all_diff_hunks_expanded(cx);
            buffer.add_diff(diff.clone(), cx);
        });
        let base_version = base_buffer.read_with(cx, |buffer, _| buffer.version());
        let main_version = main_buffer.read_with(cx, |buffer, _| buffer.version());

        for (main_styled, base_styled) in [
            (false, false),
            (true, false),
            (true, true),
            (false, true),
            (false, false),
        ] {
            main_buffer.update(cx, |buffer, cx| {
                buffer.set_language(main_styled.then(|| language.clone()), cx)
            });
            base_buffer.update(cx, |buffer, cx| {
                buffer.set_language(base_styled.then(|| language.clone()), cx)
            });
            cx.run_until_parked();
            assert_eq!(
                base_buffer.read_with(cx, |buffer, _| buffer.version()),
                base_version
            );
            assert_eq!(
                main_buffer.read_with(cx, |buffer, _| buffer.version()),
                main_version
            );
            assert_eq!(
                diff.read_with(cx, |diff, _| diff.base_text_buffer().entity_id()),
                base_buffer.entity_id()
            );
            let snapshot = ready_snapshot(&map, cx);
            for (row, styled) in [(0, base_styled), (2, main_styled)] {
                let advance = if styled { 8. } else { 7. };
                assert_eq!(
                    snapshot
                        .long_row(DisplayRow(row))
                        .expect("row geometry ready")
                        .expect("long row")
                        .width(),
                    text.len() as f64 * advance,
                    "row {row}, main syntax {main_styled}, base syntax {base_styled}",
                );
            }
        }
    }

    #[test]
    fn test_long_row_shaping_source_local_updates() {
        let mut cx = shaping_context();
        crate::editor_tests::init_test(&mut cx, |_| {});
        let cx = cx.add_empty_window();
        let editor = cx.update(|window, cx| {
            let buffer = MultiBuffer::build_simple(&"w".repeat(20_000), cx);
            cx.new(|cx| Editor::new(EditorMode::full(), buffer, None, window, cx))
        });
        let map = editor.read_with(cx, |editor, _| editor.display_map.clone());
        let mut style = editor.update(cx, |editor, cx| editor.style(cx).clone());
        style.text.font_size = gpui::AbsoluteLength::from(px(16.));
        style.text.font_weight = FontWeight::NORMAL;
        style.show_underlines = true;
        editor.update(cx, |editor, cx| {
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx)
        });
        cx.draw(point(px(0.), px(0.)), size(px(500.), px(500.)), |_, _| {
            EditorElement::new(&editor, style.clone())
        });
        cx.background_executor.set_block_on_ticks(0..=0);
        let initial = ready_snapshot(&map, cx)
            .long_row(DisplayRow(0))
            .expect("row geometry ready")
            .expect("long row");
        assert_eq!(initial.width(), 120_000.);
        for underlined in [true, false] {
            editor.update(cx, |editor, cx| {
                editor.clear_highlights(HighlightKey::Editor, cx);
                if underlined {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    editor.highlight_text(
                        HighlightKey::Editor,
                        vec![
                            snapshot.anchor_after(MultiBufferOffset(10_001))
                                ..snapshot.anchor_before(MultiBufferOffset(10_002)),
                        ],
                        HighlightStyle {
                            underline: Some(UnderlineStyle {
                                thickness: px(1.),
                                color: None,
                                wavy: false,
                            }),
                            ..HighlightStyle::default()
                        },
                        cx,
                    );
                }
            });
            let row = ready_snapshot(&map, cx)
                .long_row(DisplayRow(0))
                .expect("row geometry ready")
                .expect("long row");
            assert_eq!(row.width(), if underlined { 120_004. } else { 120_000. });
        }
        let buffer = editor.read_with(cx, |editor, _| editor.buffer().clone());
        for (range, inserted, expected) in [
            (10_000..10_000, "i", vec![120_008.]),
            (10_000..10_000, "\n", vec![60_000., 60_008.]),
            (10_000..10_001, "", vec![120_008.]),
        ] {
            buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(
                        MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                        inserted,
                    )],
                    None,
                    cx,
                )
            });
            let snapshot = ready_snapshot(&map, cx);
            for (row, width) in expected.into_iter().enumerate() {
                assert_eq!(
                    snapshot
                        .long_row(DisplayRow(row as u32))
                        .expect("row geometry ready")
                        .expect("edited row")
                        .width(),
                    width
                );
            }
            assert_eq!(initial.width(), 120_000.);
        }
        buffer.update(cx, |buffer, cx| {
            let end = buffer.snapshot(cx).len();
            buffer.edit([(end..end, "\nshort")], None, cx);
        });
        let row = ready_snapshot(&map, cx)
            .long_row(DisplayRow(0))
            .expect("row geometry ready")
            .expect("long row");
        assert_eq!(row.width(), 120_008.);
        let mut highlighted_rows = row.widths.highlighted_rows.load(atomic::Ordering::Relaxed);
        let source_buffer = buffer.read_with(cx, |buffer, _| {
            buffer.as_singleton().expect("singleton buffer")
        });
        for diagnostic_row in [1, 0, 1] {
            source_buffer.update(cx, |buffer, cx| {
                let column = if diagnostic_row == 0 { 10_001 } else { 0 };
                let diagnostics = DiagnosticSet::new(
                    [DiagnosticEntry::new(
                        PointUtf16::new(diagnostic_row, column)
                            ..PointUtf16::new(diagnostic_row, column + 1),
                        Diagnostic::default(),
                    )],
                    buffer,
                );
                buffer.update_diagnostics(LanguageServerId(0), diagnostics, cx);
            });
            let updated = ready_snapshot(&map, cx)
                .long_row(DisplayRow(0))
                .expect("row geometry ready")
                .expect("long row");
            let expected = if diagnostic_row == 0 {
                highlighted_rows += 1;
                120_012.
            } else {
                120_008.
            };
            assert_eq!(updated.width(), expected);
            assert_eq!(
                updated
                    .widths
                    .highlighted_rows
                    .load(atomic::Ordering::Relaxed),
                highlighted_rows
            );
        }
    }

    struct UnderlinePaintView(Rc<dyn Fn(&mut Window, &mut App)>);

    impl Render for UnderlinePaintView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let paint = self.0.clone();
            canvas(|_, _, _| {}, move |_, _, window, cx| paint(window, cx)).size_full()
        }
    }

    struct IgnoredMarksTextSystem(ShapingTextSystem);

    impl PlatformTextSystem for IgnoredMarksTextSystem {
        fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> anyhow::Result<()> {
            self.0.add_fonts(fonts)
        }

        fn all_font_names(&self) -> Vec<String> {
            self.0.all_font_names()
        }

        fn font_id(&self, descriptor: &Font) -> anyhow::Result<FontId> {
            self.0.font_id(descriptor)
        }

        fn font_metrics(&self, font_id: FontId) -> FontMetrics {
            let mut metrics = self.0.font_metrics(font_id);
            if font_id == ShapingTextSystem::ITALIC || font_id == ShapingTextSystem::BOLD {
                let units_per_em = metrics.units_per_em as f32;
                metrics.bounding_box.origin.x = if font_id == ShapingTextSystem::ITALIC {
                    -2. * units_per_em
                } else {
                    2. * units_per_em
                };
                metrics.bounding_box.size.width = 4. * units_per_em;
            }
            metrics
        }

        fn typographic_bounds(
            &self,
            font_id: FontId,
            glyph_id: GlyphId,
        ) -> anyhow::Result<Bounds<f32>> {
            self.0.typographic_bounds(font_id, glyph_id)
        }

        fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Size<f32>> {
            self.0.advance(font_id, glyph_id)
        }

        fn glyph_for_char(&self, font_id: FontId, character: char) -> Option<GlyphId> {
            self.0.glyph_for_char(font_id, character)
        }

        fn glyph_raster_bounds(
            &self,
            params: &RenderGlyphParams,
        ) -> anyhow::Result<Bounds<DevicePixels>> {
            if params.font_id == ShapingTextSystem::ITALIC
                || params.font_id == ShapingTextSystem::BOLD
            {
                let ink = self
                    .font_metrics(params.font_id)
                    .bounding_box(params.font_size);
                let scale = params.scale_factor;
                let padding = 1 + i32::from(params.dilation) + i32::from(params.subpixel_rendering);
                let left = (f32::from(ink.left()) * scale).floor() as i32 - padding;
                let right = (f32::from(ink.right()) * scale).ceil() as i32
                    + padding
                    + i32::from(params.subpixel_variant.x > 0);
                return Ok(Bounds::new(
                    point(DevicePixels(left), DevicePixels(-16)),
                    size(DevicePixels(right - left), DevicePixels(16)),
                ));
            }
            self.0.glyph_raster_bounds(params)
        }

        fn rasterize_glyph(
            &self,
            params: &RenderGlyphParams,
            bounds: Bounds<DevicePixels>,
        ) -> anyhow::Result<(Size<DevicePixels>, Vec<u8>)> {
            self.0.rasterize_glyph(params, bounds)
        }

        fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
            let mut filtered = String::new();
            let mut offsets = Vec::new();
            let mut start = 0;
            let fonts = runs
                .iter()
                .map(|run| {
                    let filtered_start = filtered.len();
                    for (offset, character) in text[start..start + run.len].char_indices() {
                        if character != '\u{34f}' {
                            filtered.push(character);
                            offsets.extend(start + offset..start + offset + character.len_utf8());
                        }
                    }
                    start += run.len;
                    FontRun {
                        len: filtered.len() - filtered_start,
                        font_id: run.font_id,
                    }
                })
                .collect::<Vec<_>>();
            let mut layout = self.0.layout_line(&filtered, font_size, &fonts);
            let patterns = [9, 65].map(|count| format!("Q{}Z", "x".repeat(count)));
            let mut extra = Pixels::ZERO;
            for glyph in layout.runs.iter_mut().flat_map(|run| &mut run.glyphs) {
                glyph.position.x += extra;
                if filtered
                    .get(glyph.index..)
                    .is_some_and(|text| patterns.iter().any(|pattern| text.starts_with(pattern)))
                {
                    glyph.id = GlyphId(9);
                    extra += font_size * ShapingTextSystem::ADVANCE;
                }
                glyph.index = *offsets.get(glyph.index).expect("unfiltered glyph index");
            }
            layout.width += extra;
            layout.len = text.len();
            layout
        }

        fn layout_line_for_retention(
            &self,
            text: &str,
            font_size: Pixels,
            runs: &[FontRun],
        ) -> RetainedLineLayout {
            let (layout, positions, width) =
                self.layout_line_with_precise_positions(text, font_size, runs);
            if !text.starts_with('!')
                && runs.len() == 1
                && !text.is_empty()
                && text.bytes().all(|byte| (b' '..=b'~').contains(&byte))
                && let [run] = layout.runs.as_slice()
                && run.glyphs.len() == text.len()
                && (layout.caret_positions.is_empty()
                    || layout.caret_positions.iter().copied().eq(positions
                        .iter()
                        .copied()
                        .enumerate()
                        .chain([(text.len(), width)])))
                && run.glyphs.iter().enumerate().all(|(index, glyph)| {
                    glyph.index == index
                        && glyph.position.y == Pixels::ZERO
                        && u16::try_from(glyph.id.0).is_ok()
                })
            {
                return RetainedLineLayout::Compact(CompactLineLayout {
                    font_id: run.font_id,
                    font_size,
                    ascent: layout.ascent,
                    descent: layout.descent,
                    width,
                    is_emoji: run.glyphs[0].is_emoji,
                    glyphs: run.glyphs.iter().map(|glyph| glyph.id.0 as u16).collect(),
                    positions: CompactGlyphPositions::new(positions.len(), |range, output| {
                        output.copy_from_slice(&positions[range]);
                        Some(())
                    })
                    .expect("stable native positions"),
                });
            }
            RetainedLineLayout::Dense(layout, positions, width)
        }

        fn recommended_rendering_mode(
            &self,
            font_id: FontId,
            font_size: Pixels,
        ) -> TextRenderingMode {
            self.0.recommended_rendering_mode(font_id, font_size)
        }
    }

    fn merge_chunks<'a, T: PartialEq>(
        chunks: impl Iterator<Item = (&'a str, T)>,
    ) -> Vec<(String, T)> {
        let mut merged = Vec::<(String, T)>::new();
        for (text, value) in chunks {
            match merged.last_mut() {
                Some((last_text, last_value)) if *last_value == value => last_text.push_str(text),
                _ => merged.push((text.to_string(), value)),
            }
        }
        merged
    }

    fn ready_snapshot(map: &Entity<DisplayMap>, cx: &mut TestAppContext) -> DisplaySnapshot {
        loop {
            let (snapshot, width) = map.update(cx, |map, cx| {
                let snapshot = map.snapshot(cx);
                let pending = (0..=snapshot.max_point().row().0)
                    .filter_map(|row| snapshot.long_row(DisplayRow(row)).err())
                    .collect::<Vec<_>>();
                let ready = pending.is_empty();
                map.request_viewport_row_geometry(pending, cx);
                let width = map
                    .request_max_long_row_width(&snapshot, cx)
                    .filter(|_| ready);
                (snapshot, width)
            });
            if width.is_some() {
                return snapshot;
            }
            cx.run_until_parked();
        }
    }

    fn test_long_row(map: &Entity<DisplayMap>, cx: &mut TestAppContext) -> LongRow {
        ready_snapshot(&map, cx)
            .long_row(DisplayRow(0))
            .expect("row geometry ready")
            .expect("a long row")
    }

    fn set_test_shaping_style(map: &Entity<DisplayMap>, cx: &mut TestAppContext) {
        let font = test_font();
        let style = EditorStyle {
            text: gpui::TextStyle {
                font_family: font.family,
                font_features: font.features,
                font_fallbacks: font.fallbacks,
                font_size: gpui::AbsoluteLength::from(FONT_SIZE),
                font_weight: font.weight,
                font_style: font.style,
                ..gpui::TextStyle::default()
            },
            ..EditorStyle::default()
        };
        map.update(cx, |map, cx| {
            map.set_shaping_style(&style, SyntaxStyling::default(), cx);
        });
    }

    fn shaping_context() -> TestAppContext {
        TestAppContext::build_with_text_system(
            gpui::TestDispatcher::new(0),
            None,
            Arc::new(ShapingTextSystem(gpui::NoopTextSystem)),
        )
    }

    fn ignored_marks_context() -> TestAppContext {
        TestAppContext::build_with_text_system(
            gpui::TestDispatcher::new(0),
            None,
            Arc::new(IgnoredMarksTextSystem(ShapingTextSystem(
                gpui::NoopTextSystem,
            ))),
        )
    }

    fn assert_matches_whole_line_shaping(
        map: &Entity<DisplayMap>,
        runs: Option<&[FontRun]>,
        cx: &mut TestAppContext,
    ) {
        let snapshot = ready_snapshot(&map, cx);
        let long_row = snapshot
            .long_row(DisplayRow(0))
            .expect("row geometry ready")
            .expect("a long row");
        let text = snapshot.buffer_snapshot().text();
        assert_long_row_matches_whole_line_shaping(&long_row, &text, runs, cx);
    }

    fn assert_long_row_matches_whole_line_shaping(
        long_row: &LongRow,
        text: &str,
        runs: Option<&[FontRun]>,
        cx: &mut TestAppContext,
    ) {
        let wrapper = cx.update(|cx| cx.text_system().line_wrapper(test_font(), FONT_SIZE));
        let base_runs = [FontRun {
            len: text.len(),
            font_id: wrapper.font_id(),
        }];
        let runs = runs.unwrap_or(&base_runs);
        let layout = wrapper.layout_line(text, runs);
        assert_window_matches_layout(long_row, 0..long_row.len(), 0., &layout);
        let columns = text
            .grapheme_indices(true)
            .map(|(start, _)| start)
            .chain([text.len()])
            .collect::<Vec<_>>();
        for (column, expected) in columns
            .iter()
            .copied()
            .zip(cluster_interpolated_x(&layout, &columns))
        {
            let actual = long_row.x_for_column(column as u32);
            assert!(
                (actual - expected).abs() < 0.01,
                "column {column} of {} bytes: {actual} != {expected}",
                text.len()
            );
        }
    }

    fn cluster_interpolated_x(layout: &LineLayout, columns: &[usize]) -> Vec<ScrollPixelOffset> {
        let mut glyph_starts = std::collections::BTreeMap::<usize, f64>::new();
        for glyph in layout.runs.iter().flat_map(|run| &run.glyphs) {
            let x = f64::from(glyph.position.x);
            glyph_starts
                .entry(glyph.index)
                .and_modify(|start| *start = start.min(x))
                .or_insert(x);
        }
        let width = f64::from(layout.width);
        columns
            .iter()
            .enumerate()
            .map(|(index, &column)| {
                if column >= layout.len {
                    return width;
                }
                let next_column = columns.get(index + 1).copied().unwrap_or(layout.len);
                if glyph_starts.range(column..next_column).next().is_some() {
                    return f64::from(layout.x_for_index(column));
                }
                let Some((&cluster_start, &cluster_x)) = glyph_starts.range(..column).next_back()
                else {
                    return glyph_starts.values().next().copied().unwrap_or(width);
                };
                let (cluster_end, cluster_end_x) = glyph_starts
                    .range(column..)
                    .next()
                    .map_or((layout.len, width), |(index, x)| (*index, *x));
                let interior = columns
                    .iter()
                    .filter(|other| cluster_start < **other && **other < cluster_end)
                    .collect::<Vec<_>>();
                let rank = interior
                    .iter()
                    .position(|other| **other == column)
                    .unwrap_or(0)
                    + 1;
                cluster_x + (cluster_end_x - cluster_x) * rank as f64 / (interior.len() + 1) as f64
            })
            .collect()
    }

    fn assert_window_matches_layout(
        long_row: &LongRow,
        columns: Range<u32>,
        origin_x: ScrollPixelOffset,
        expected: &LineLayout,
    ) {
        let window = long_row.window_layout(columns.clone(), origin_x);

        let actual = window
            .iter()
            .flat_map(|piece| {
                assert!(columns.start <= piece.columns.start && piece.columns.end <= columns.end);
                positioned_glyphs(&piece.layout, piece.columns.start as usize, piece.x)
            })
            .collect::<Vec<_>>();
        let expected = positioned_glyphs(expected, 0, 0.)
            .into_iter()
            .filter(|glyph| columns.contains(&(glyph.2 as u32)))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected, "{columns:?}");
    }

    fn retained_piece(
        text: &str,
        width: f64,
        glyphs: impl IntoIterator<Item = (FontId, GlyphId, usize, f64, Pixels)>,
    ) -> TextPiece {
        let mut runs = Vec::<ShapedRun>::new();
        let mut positions = Vec::new();
        for (font_id, id, index, x, y) in glyphs {
            if runs.last().is_none_or(|run| run.font_id != font_id) {
                runs.push(ShapedRun {
                    font_id,
                    glyphs: Vec::new(),
                });
            }
            let run = runs.last_mut().expect("glyph run");
            run.glyphs.push(ShapedGlyph {
                id,
                position: point(px(x as f32), y),
                index,
                is_emoji: false,
            });
            positions.push(x);
        }
        let layout = Arc::new(LineLayout {
            len: text.len(),
            width: px(width as f32),
            runs,
            ..LineLayout::default()
        });
        TextPiece {
            columns: 0..text.len() as u32,
            x: 0.,
            source_start_column: 0,
            text: SharedString::new(text),
            index: LayoutIndex::new(&layout, positions, width),
            layout,
        }
    }

    fn positioned_glyphs(
        layout: &LineLayout,
        column: usize,
        x: ScrollPixelOffset,
    ) -> Vec<(FontId, GlyphId, usize, ScrollPixelOffset, Pixels)> {
        layout
            .runs
            .iter()
            .flat_map(|run| {
                run.glyphs.iter().map(move |glyph| {
                    (
                        run.font_id,
                        glyph.id,
                        column + glyph.index,
                        x + f64::from(glyph.position.x),
                        glyph.position.y,
                    )
                })
            })
            .collect()
    }

    fn shaped_long_row(text: &str, cx: &mut TestAppContext) -> LongRow {
        let (_, map) = build_display_map(text, 4, cx);
        test_long_row(&map, cx)
    }

    fn build_display_map(
        text: &str,
        tab_size: u32,
        cx: &mut TestAppContext,
    ) -> (Entity<MultiBuffer>, Entity<DisplayMap>) {
        init_settings(tab_size, cx);
        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let map = new_display_map(&buffer, cx);
        (buffer, map)
    }

    fn init_settings(tab_size: u32, cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            crate::init(cx);
            theme_settings::init(LoadThemes::JustBase, cx);
        });
        set_tab_size(tab_size, cx);
    }

    fn set_tab_size(tab_size: u32, cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.tab_size = NonZeroU32::new(tab_size);
                });
            });
        });
    }

    fn new_display_map(
        buffer: &Entity<MultiBuffer>,
        cx: &mut TestAppContext,
    ) -> Entity<DisplayMap> {
        cx.new(|cx| {
            DisplayMap::new(
                buffer.clone(),
                test_font(),
                FONT_SIZE,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        })
    }

    fn randomly_mutate(
        buffer: &Entity<MultiBuffer>,
        map: &Entity<DisplayMap>,
        rng: &mut StdRng,
        cx: &mut TestAppContext,
    ) {
        let text = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text());
        let offset = |rng: &mut StdRng, range: RangeInclusive<usize>| {
            let mut offset = rng.random_range(range);
            while !text.is_char_boundary(offset) {
                offset -= 1;
            }
            offset
        };
        let start = offset(rng, 0..=text.len());
        let end = offset(rng, start..=(start + 60).min(text.len()));
        match rng.random_range(0..8) {
            0 if start < end => {
                let creases = vec![Crease::simple(
                    MultiBufferOffset(start)..MultiBufferOffset(end),
                    FoldPlaceholder::test(),
                )];
                map.update(cx, |map, cx| {
                    map.fold(creases, cx);
                    let snapshot = map.snapshot(cx);
                    let wrapper = cx.text_system().line_wrapper(test_font(), FONT_SIZE);
                    let mut widths = Vec::new();
                    snapshot.for_each_unmeasured_renderer(
                        DisplayRow(0)..snapshot.max_point().row() + DisplayRow(1),
                        |renderer, text| {
                            assert!(renderer.constrain_width);
                            let text = text.materialize();
                            let width = wrapper
                                .layout_line(
                                    &text,
                                    &[FontRun {
                                        len: text.len(),
                                        font_id: wrapper.font_id(),
                                    }],
                                )
                                .width;
                            widths.push((renderer.id, width));
                            ControlFlow::Continue(())
                        },
                    );
                    map.update_renderer_widths(
                        RendererWidthsKey {
                            rem_size: px(16.),
                            font: test_font(),
                            font_size: FONT_SIZE,
                            buffer_font: test_font(),
                            editor_width: px(500.),
                            line_height: px(20.),
                        },
                        widths,
                        cx,
                    );
                });
            }
            1 => {
                let position = buffer.read_with(cx, |buffer, cx| {
                    buffer.snapshot(cx).anchor_after(MultiBufferOffset(start))
                });
                let inlay = Inlay::mock_hint(
                    rng.random_range(0..usize::MAX),
                    position,
                    random_text(rng, 1..200).as_str(),
                );
                map.update(cx, |map, cx| map.splice_inlays(&[], vec![inlay], cx));
            }
            _ => {
                let mut edits = Vec::new();
                let mut edit_start = start;
                for _ in 0..rng.random_range(1..=3) {
                    if edit_start >= text.len() {
                        break;
                    }
                    let edit_end = offset(rng, edit_start..=(edit_start + 40).min(text.len()));
                    let inserted = if rng.random_ratio(1, 6) {
                        "\n".to_string()
                    } else {
                        random_text(rng, 0..200)
                    };
                    edits.push((
                        MultiBufferOffset(edit_start)..MultiBufferOffset(edit_end),
                        inserted,
                    ));
                    edit_start = offset(rng, edit_end..=(edit_end + 80).min(text.len()));
                }
                buffer.update(cx, |buffer, cx| buffer.edit(edits, None, cx));
            }
        }
    }

    fn random_text(rng: &mut StdRng, chars: Range<usize>) -> String {
        const ALPHABET: &[char] = &['a', 'W', ' ', '\t', 'é', '漢', '😀', '\u{0}', '\u{7f}'];
        let len = rng.random_range(chars);
        (0..len)
            .map(|_| ALPHABET[rng.random_range(0..ALPHABET.len())])
            .collect()
    }

    fn assert_round_trips(snapshot: &DisplaySnapshot) {
        for row in 0..=snapshot.max_point().row().0 {
            let Some(long_row) = snapshot
                .long_row(DisplayRow(row))
                .expect("row geometry ready")
            else {
                continue;
            };
            let line = snapshot.line(DisplayRow(row));
            assert_eq!(long_row.len() as usize, line.len());
            let mut previous_x = -1.;
            for (column, _) in line.char_indices().step_by(7) {
                let x = long_row.x_for_column(column as u32);
                assert!(
                    x > previous_x,
                    "row {row} column {column}: {x} <= {previous_x}"
                );
                assert_eq!(long_row.closest_column_for_x(x) as usize, column);
                previous_x = x;
            }
        }
    }

    fn assert_cluster_atoms(snapshot: &DisplaySnapshot) {
        for row in 0..=snapshot.max_point().row().0 {
            let Some(long_row) = snapshot
                .long_row(DisplayRow(row))
                .expect("row geometry ready")
            else {
                continue;
            };

            let line = snapshot.line(DisplayRow(row));
            let expected_atoms = line
                .grapheme_indices(true)
                .map(|(start, grapheme)| start..start + grapheme.len())
                .collect::<Vec<_>>();
            let atom_starts = long_row
                .geometry()
                .atoms
                .columns
                .iter()
                .map(|start| *start as usize)
                .collect::<Vec<_>>();
            assert_eq!(
                atom_starts,
                expected_atoms
                    .iter()
                    .map(|atom| atom.start)
                    .collect::<Vec<_>>(),
                "row {row}"
            );
            for atom in expected_atoms {
                let x = long_row.x_for_column(atom.start as u32);
                let inner =
                    atom.start + line[atom.clone()].chars().next().map_or(0, char::len_utf8);
                if inner < atom.end {
                    assert_eq!(
                        long_row.x_for_column(inner as u32),
                        long_row.x_for_column(atom.end as u32),
                        "row {row}"
                    );
                }
                assert_eq!(
                    long_row.closest_column_for_x(x) as usize,
                    atom.start,
                    "row {row}"
                );
            }
        }
    }

    fn assert_matches_char_widths(snapshot: &DisplaySnapshot, wrapper: &mut LineWrapper) {
        for row in 0..=snapshot.max_point().row().0 {
            let line = snapshot.line(DisplayRow(row));
            let Some(long_row) = snapshot
                .long_row(DisplayRow(row))
                .expect("row geometry ready")
            else {
                assert!(
                    line.len() <= MAX_LINE_LEN,
                    "row {row} has no long row geometry"
                );
                continue;
            };
            let standalone_invisibles = line
                .grapheme_indices(true)
                .filter(|(_, grapheme)| single_invisible(grapheme).is_some())
                .map(|(start, grapheme)| start..start + grapheme.len())
                .collect::<Vec<_>>();
            let mut expected_x = 0.;
            for (index, (column, character)) in line.char_indices().enumerate() {
                if index % 5 == 0 {
                    let x = long_row.x_for_column(column as u32);
                    assert!(
                        (x - expected_x).abs() < 1e-2,
                        "row {row} column {column}: {x} != {expected_x}"
                    );
                }
                let measured = if standalone_invisibles
                    .iter()
                    .any(|range| range.start == column)
                {
                    replacement(character).unwrap_or(character)
                } else {
                    character
                };
                expected_x += f64::from(wrapper.width_for_char(measured));
            }
            assert!((long_row.width() - expected_x).abs() < 1e-2);
        }
    }
}
