use std::{
    collections::VecDeque,
    iter::Peekable,
    mem,
    ops::Range,
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
};

use collections::HashMap;
use futures_lite::future::yield_now;
use gpui::{FontId, FontRun, LineLayout, LineWrapper, LineWrapperHandle, Pixels, TextAlign};
use language::LanguageAwareStyling;
use parking_lot::Mutex;
use smallvec::{SmallVec, smallvec};
use sum_tree::{Bias, Cursor, SumTree};
use unicode_segmentation::GraphemeCursor;

use super::{
    ChunkRendererId, Highlights,
    invisibles::{is_invisible, replacement},
    tab_map::{TabChunks, TabEdit, TabPoint, TabSnapshot},
};
use crate::{MAX_LINE_LEN, scroll::ScrollPixelOffset};

const MIN_CHUNK_LEN: u32 = if cfg!(test) { 16 } else { 1024 };
const MAX_CHUNK_LEN: u32 = if cfg!(test) { 64 } else { 8 * 1024 };
const WHITESPACE_CUT_MASK: u64 = if cfg!(test) { 0b11 } else { 0x7f };
const DENSE_CUT_MASK: u64 = if cfg!(test) { 0b1111 } else { 0x7ff };
const DENSE_RUN_LEN: u32 = if cfg!(test) { 8 } else { 64 };
const MAX_CUT_SCAN_LEN: u32 = if cfg!(test) { 2 * 1024 } else { 64 * 1024 };
const SYNC_MEASURE_LEN: u32 = if cfg!(test) { 512 } else { 16 * 1024 };
const MEASURE_YIELD_LEN: u32 = if cfg!(test) { 256 } else { 8 * 1024 };
const SCAN_YIELD_STEPS: u32 = 1024;
const SYNC_SCAN_ROWS: usize = 32;
const MAX_WINDOW_LEN: u32 = 64 * 1024;
const MAX_VISIBLE_WINDOW_LEN: u32 = 1024 * 1024;
const MAX_ATOM_LEN: u32 = if cfg!(test) { 32 } else { MAX_LINE_LEN as u32 };
const GRAPHEME_CONTEXT_LEN: u32 = 256;
pub(super) const MAX_GRAPHEME_CONTEXT_LEN: u32 = 16 * 1024;
const MAX_RESYNC_ATTEMPTS: usize = 16;
const MAX_CACHED_LAYOUT_VARIANTS: usize = 4;

const MAX_CACHED_LAYOUT_BYTES: usize = 256 * 1024;
pub(crate) const POSITION_TOLERANCE: ScrollPixelOffset = 0.01;

static NEXT_EPOCH: AtomicUsize = AtomicUsize::new(1);
static GEAR: [u64; 256] = gear_table();

#[derive(Clone)]
pub struct LongRow {
    snapshot: TabSnapshot,
    row: u32,
    geometry: Arc<RowGeometry>,
    widths: Arc<CharWidths>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowWindow {
    segments: SmallVec<[Range<u32>; 1]>,
}

pub(crate) struct MeasureRequest {
    version: usize,
    row: u32,
    columns: SmallVec<[Range<u32>; 1]>,
    runs: SmallVec<[RowRuns; 1]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RowRuns {
    start: u32,
    runs: Vec<FontRun>,
    ends: Vec<u32>,
}

pub(crate) struct TextPiece {
    pub(crate) columns: Range<u32>,
    pub(crate) x: ScrollPixelOffset,
    pub(crate) text: String,
    pub(crate) layout: Arc<LineLayout>,
}

impl RowRuns {
    pub(crate) fn new(start: u32, runs: Vec<FontRun>) -> Self {
        let mut end = start;
        let ends = runs
            .iter()
            .map(|run| {
                end += run.len as u32;
                end
            })
            .collect();
        Self { start, runs, ends }
    }

    fn end(&self) -> u32 {
        self.ends.last().copied().unwrap_or(self.start)
    }

    fn covers(&self, columns: &Range<u32>) -> bool {
        self.start <= columns.start && columns.end <= self.end()
    }

    fn slice(&self, columns: Range<u32>) -> Vec<FontRun> {
        let columns = columns.start.max(self.start)..columns.end.min(self.end());
        let mut runs = Vec::new();
        if columns.is_empty() {
            return runs;
        }
        let first = self.ends.partition_point(|end| *end <= columns.start);
        for index in first..self.runs.len() {
            let run_start = index
                .checked_sub(1)
                .map_or(self.start, |previous| self.ends[previous]);
            if run_start >= columns.end {
                break;
            }
            let start = run_start.max(columns.start);
            let end = self.ends[index].min(columns.end);
            runs.push(FontRun {
                len: (end - start) as usize,
                font_id: self.runs[index].font_id,
            });
        }
        runs
    }
}

impl RowWindow {
    pub(crate) fn prefix(end: u32) -> Self {
        Self {
            segments: smallvec![0..end],
        }
    }

    pub(crate) fn segments(&self) -> &[Range<u32>] {
        &self.segments
    }

    pub(crate) fn end(&self) -> u32 {
        self.segments.last().map_or(0, |segment| segment.end)
    }
}

struct Piece {
    columns: Range<u32>,
    density: ScrollPixelOffset,
}

impl Piece {
    fn len(&self) -> u32 {
        self.columns.end - self.columns.start
    }
}

impl LongRow {
    pub(crate) fn len(&self) -> u32 {
        self.geometry.len()
    }

    pub(crate) fn width(&self) -> ScrollPixelOffset {
        self.geometry.width()
    }

    pub(crate) fn is_measured(&self) -> bool {
        self.geometry.is_measured()
    }

    pub(crate) fn trailing_whitespace_start(&self) -> Option<u32> {
        self.geometry.trailing_whitespace_start()
    }

    pub(crate) fn x_for_column(&self, column: u32) -> ScrollPixelOffset {
        let atom = self.atom_range(column);
        let column = if atom.start == column {
            column
        } else {
            atom.end
        };
        self.x_for_boundary(column)
    }

    pub(crate) fn closest_column_for_x(&self, x: ScrollPixelOffset) -> u32 {
        if x <= 0. {
            return 0;
        }
        if x >= self.width() {
            return self.len();
        }
        let atom = self.atom_at_x(x);
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
        self.atom_at_x(x).start
    }

    pub(crate) fn visible_x(
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

    pub(crate) fn visible_window(
        &self,
        scroll_x: ScrollPixelOffset,
        text_align: TextAlign,
        viewport_width: ScrollPixelOffset,
    ) -> RowWindow {
        let visible = self.visible_x(scroll_x, text_align, viewport_width);
        match self.visible_columns(visible) {
            Ok((visible_columns, _)) => RowWindow {
                segments: smallvec![visible_columns],
            },
            Err(window) => window,
        }
    }

    pub(crate) fn window(
        &self,
        scroll_x: ScrollPixelOffset,
        text_align: TextAlign,
        viewport_width: ScrollPixelOffset,
    ) -> RowWindow {
        let visible = self.visible_x(scroll_x, text_align, viewport_width);
        let (visible_columns, visible_len) = match self.visible_columns(visible.clone()) {
            Ok(visible_columns) => visible_columns,
            Err(window) => return window,
        };
        let margin = visible.end - visible.start;
        let budget = (MAX_WINDOW_LEN - visible_len.min(MAX_WINDOW_LEN)) / 2;
        let start = self
            .column_at_x(visible.start - margin)
            .max(self.floor_column(visible_columns.start.saturating_sub(budget)))
            .min(visible_columns.start);
        let end = self
            .column_end_at_x(visible.end + margin)
            .min(self.floor_column(visible_columns.end.saturating_add(budget)))
            .max(visible_columns.end);
        RowWindow {
            segments: smallvec![start..end],
        }
    }

    pub(crate) fn window_around(&self, column: u32) -> RowWindow {
        let start = self.floor_column(column.saturating_sub(MAX_WINDOW_LEN / 2));
        let end = column.saturating_add(MAX_WINDOW_LEN / 2);
        let end = if end >= self.len() {
            self.len()
        } else {
            self.floor_column(end)
        };
        RowWindow {
            segments: smallvec![start..end.max(start)],
        }
    }

    pub(crate) fn atom_positions(&self, columns: Range<u32>) -> Vec<(u32, ScrollPixelOffset)> {
        let mut positions = Vec::new();
        let chunks = &self.geometry.chunks;
        for index in self.geometry.chunk_containing(columns.start)..chunks.len() {
            let (chunk_start, chunk_x) = self.geometry.chunk_start(index);
            let chunk = &chunks[index];
            if chunk_start >= columns.end {
                break;
            }
            if chunk.measured {
                let layout = self.chunk_layout(index, false);
                positions.extend(
                    layout
                        .atoms
                        .iter()
                        .filter(|(atom_start, _)| columns.contains(atom_start))
                        .map(|(atom_start, atom_x)| (*atom_start, chunk_x + atom_x)),
                );
            } else {
                let estimate = self.widths.estimate;
                for_each_atom(
                    &self.snapshot,
                    self.row,
                    chunk_start.max(columns.start)..chunk.end.min(columns.end),
                    |atom| {
                        positions.push((
                            atom.start,
                            chunk_x + f64::from(atom.start.saturating_sub(chunk_start)) * estimate,
                        ));
                        true
                    },
                );
            }
        }
        positions.push((columns.end, self.x_for_boundary(columns.end)));
        positions
    }

    pub(crate) fn text_id(&self) -> usize {
        self.geometry.text_id
    }

    pub(crate) fn chunk_aligned(&self, window: &RowWindow) -> RowWindow {
        let geometry = &self.geometry;
        let mut segments = SmallVec::<[Range<u32>; 1]>::new();
        for segment in window.segments() {
            let mut aligned = segment.clone();
            if !segment.is_empty() {
                let first = geometry.chunk_containing(segment.start);
                if geometry.chunks[first].measured {
                    aligned.start = geometry.chunk_start(first).0;
                }
                let last = geometry.chunk_containing(segment.end - 1);
                if geometry.chunks[last].measured {
                    aligned.end = geometry.chunks[last].end;
                }
            }
            match segments.last_mut() {
                Some(previous) if aligned.start <= previous.end => {
                    previous.end = previous.end.max(aligned.end)
                }
                _ => segments.push(aligned),
            }
        }
        RowWindow { segments }
    }

    pub(crate) fn measure_request(
        &self,
        window: &RowWindow,
        runs: &[RowRuns],
    ) -> Option<MeasureRequest> {
        let base = self.widths.base_font;
        let needs_measuring = window
            .segments()
            .iter()
            .any(|segment| self.geometry.has_unmeasured(segment));
        let needs_restyling = runs.iter().any(|row_runs| {
            self.geometry
                .chunks_in(row_runs.start..row_runs.end())
                .any(|index| {
                    let chunk = &self.geometry.chunks[index];
                    chunk.measured
                        && desired_chunk_runs(runs, self.geometry.chunk_range(index), base)
                            .is_some_and(|desired| desired != chunk.runs)
                })
        });
        (needs_measuring || needs_restyling).then(|| MeasureRequest {
            version: self.snapshot.version,
            row: self.row,
            columns: window.segments.clone(),
            runs: runs.iter().cloned().collect(),
        })
    }

    pub(crate) fn restyled(&self, runs: &[RowRuns]) -> Option<LongRow> {
        let mut wrapper = self.widths.wrapper.lock();
        let (geometry, _) = self.geometry.measure_columns(
            &self.snapshot,
            self.row,
            &[],
            runs,
            &mut wrapper,
            &self.widths,
        )?;
        Some(LongRow {
            geometry: Arc::new(geometry),
            ..self.clone()
        })
    }

    pub(crate) fn text_pieces(&self, columns: Range<u32>) -> Vec<TextPiece> {
        let mut pieces = Vec::new();
        let geometry = &self.geometry;
        if columns.is_empty() {
            return pieces;
        }
        for index in geometry.chunk_containing(columns.start)..geometry.chunks.len() {
            let (chunk_start, chunk_x) = geometry.chunk_start(index);
            if chunk_start >= columns.end {
                break;
            }
            if !geometry.chunks[index].measured {
                continue;
            }
            pieces.extend(
                self.chunk_layout(index, true)
                    .pieces
                    .into_iter()
                    .map(|piece| TextPiece {
                        x: chunk_x + piece.x,
                        ..piece
                    }),
            );
        }
        pieces
    }

    pub(crate) fn atom_range(&self, column: u32) -> Range<u32> {
        let geometry = &self.geometry;
        let len = geometry.len();
        if column >= len {
            return len..len;
        }
        let index = geometry.chunk_containing(column);
        let chunk_start = geometry.chunk_start(index).0;
        let chunk = &geometry.chunks[index];
        if chunk.measured {
            let mut atom = chunk_start..chunk.end;
            for_each_atom(&self.snapshot, self.row, chunk_start..chunk.end, |raw| {
                atom = raw.start..raw.start + raw.len;
                atom.end <= column
            });
            return atom;
        }
        let boundary =
            grapheme_floor(&self.snapshot, self.row, len, column, chunk_start).unwrap_or(column);
        let scan_start = clip_column(&self.snapshot, self.row, boundary, Bias::Left)
            .map_or(chunk_start, |clipped| clipped.clamp(chunk_start, boundary));
        let scan_end = chunk.end.min(column.saturating_add(MAX_ATOM_LEN));
        let mut atom = scan_start..scan_end;
        for_each_atom(&self.snapshot, self.row, scan_start..scan_end, |raw| {
            atom = raw.start..raw.start + raw.len;
            atom.end <= column
        });
        atom
    }

    pub(crate) fn floor_column(&self, column: u32) -> u32 {
        self.atom_range(column).start
    }

    fn visible_columns(
        &self,
        visible: Range<ScrollPixelOffset>,
    ) -> Result<(Range<u32>, u32), RowWindow> {
        let pieces = self.pieces(visible.clone());
        let visible_len = pieces.iter().map(Piece::len).sum::<u32>();
        if visible_len > MAX_VISIBLE_WINDOW_LEN {
            return Err(RowWindow {
                segments: densest_segments(&pieces, MAX_VISIBLE_WINDOW_LEN),
            });
        }
        let visible_columns = match (pieces.first(), pieces.last()) {
            (Some(first), Some(last)) => first.columns.start..last.columns.end,
            _ => {
                let column = self.column_at_x(visible.start);
                column..column
            }
        };
        Ok((visible_columns, visible_len))
    }

    fn chunk_layout(&self, index: usize, pieces: bool) -> ChunkLayout {
        let chunk = &self.geometry.chunks[index];
        let mut wrapper = self.widths.wrapper.lock();
        shape_chunk(
            &self.snapshot,
            self.row,
            ChunkRequest {
                columns: self.geometry.chunk_range(index),
                cut: false,
                runs: chunk.runs.as_deref(),
                pieces,
            },
            &mut wrapper,
            Some(&self.widths.layouts),
        )
    }

    fn x_for_boundary(&self, column: u32) -> ScrollPixelOffset {
        let geometry = &self.geometry;
        if column >= geometry.len() {
            return geometry.width();
        }
        let index = geometry.chunk_containing(column);
        let (chunk_start, chunk_x) = geometry.chunk_start(index);
        let chunk = &geometry.chunks[index];
        if column <= chunk_start {
            return chunk_x;
        }
        if !chunk.measured {
            return chunk_x + f64::from(column - chunk_start) * self.widths.estimate;
        }
        let layout = self.chunk_layout(index, false);
        let position = layout
            .atoms
            .partition_point(|(atom_start, _)| *atom_start < column);
        layout
            .atoms
            .get(position)
            .map_or(chunk.x_end, |(_, atom_x)| {
                (chunk_x + atom_x).min(chunk.x_end)
            })
    }

    fn pieces(&self, x: Range<ScrollPixelOffset>) -> Vec<Piece> {
        let start_x = x.start.max(0.);
        let end_x = x.end.min(self.width());
        let mut pieces = Vec::new();
        if start_x >= end_x {
            return pieces;
        }
        let geometry = &self.geometry;
        let first = geometry
            .chunks
            .partition_point(|chunk| chunk.x_end <= start_x);
        for index in first..geometry.chunks.len() {
            let (chunk_start, chunk_x) = geometry.chunk_start(index);
            if chunk_x >= end_x {
                break;
            }
            let chunk = &geometry.chunks[index];
            let start = if chunk_x < start_x {
                self.atom_at_x(start_x).start
            } else {
                chunk_start
            };
            let end = if chunk.x_end > end_x {
                self.atom_at_x(end_x).end
            } else {
                chunk.end
            };
            if start < end {
                pieces.push(Piece {
                    columns: start..end,
                    density: (chunk.x_end - chunk_x) / f64::from(chunk.end - chunk_start),
                });
            }
        }
        pieces
    }

    fn column_end_at_x(&self, x: ScrollPixelOffset) -> u32 {
        if x >= self.width() {
            self.len()
        } else {
            self.atom_at_x(x.max(0.)).end
        }
    }

    fn atom_at_x(&self, x: ScrollPixelOffset) -> Atom {
        let geometry = &self.geometry;
        let index = geometry
            .chunks
            .partition_point(|chunk| chunk.x_end <= x)
            .min(geometry.chunks.len().saturating_sub(1));
        let (chunk_start, chunk_x) = geometry.chunk_start(index);
        let chunk = &geometry.chunks[index];
        if !chunk.measured {
            let estimate = self.widths.estimate;
            let offset = ((x - chunk_x) / estimate).floor().max(0.) as u32;
            let target = (chunk_start + offset).min(chunk.end.saturating_sub(1));
            let Range { start, end } = self.atom_range(target);
            return Atom {
                start,
                end,
                start_x: chunk_x + f64::from(start - chunk_start) * estimate,
                end_x: chunk_x + f64::from(end - chunk_start) * estimate,
            };
        }
        let layout = self.chunk_layout(index, false);
        let position = layout
            .atoms
            .partition_point(|(_, atom_x)| chunk_x + atom_x <= x);
        let Some(atom_index) = position.checked_sub(1) else {
            return Atom {
                start: chunk.end,
                end: chunk.end,
                start_x: chunk.x_end,
                end_x: chunk.x_end,
            };
        };
        let (start, start_x) = layout.atoms[atom_index];
        let (end, end_x) = layout
            .atoms
            .get(atom_index + 1)
            .map_or((chunk.end, chunk.x_end), |(end, end_x)| {
                (*end, chunk_x + end_x)
            });
        Atom {
            start,
            end,
            start_x: chunk_x + start_x,
            end_x,
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct LongRows {
    widths: Option<Arc<CharWidths>>,
    rows: SumTree<RowEntry>,
    pending: Arc<Vec<PendingRows>>,
    epoch: usize,
}

#[derive(Clone, Debug)]
struct PendingRows {
    rows: Range<u32>,
    text_id: usize,
}

impl LongRows {
    pub(crate) fn new(mut wrapper: LineWrapperHandle) -> Self {
        let estimate = f64::from(wrapper.width_for_char('0')).max(1.);
        let em_width = f64::from(wrapper.width_for_char('m'));
        let base_font = wrapper.font_id();
        Self {
            widths: Some(Arc::new(CharWidths {
                wrapper: Mutex::new(wrapper),
                estimate,
                em_width,
                base_font,
                layouts: Mutex::default(),
            })),
            rows: SumTree::default(),
            pending: Arc::new(vec![PendingRows {
                rows: 0..u32::MAX,
                text_id: next_epoch(),
            }]),
            epoch: next_epoch(),
        }
    }

    pub(crate) fn epoch(&self) -> usize {
        self.epoch
    }

    pub(crate) fn max_width(&self) -> ScrollPixelOffset {
        self.rows.summary().max_width
    }

    pub(crate) fn row_widths(&self) -> impl Iterator<Item = (u32, ScrollPixelOffset)> + '_ {
        self.rows.iter().scan(0, |end, entry| {
            let row = *end + entry.gap;
            *end = row + 1;
            Some((row, entry.geometry.width()))
        })
    }

    pub(crate) fn em_width(&self) -> Option<ScrollPixelOffset> {
        Some(self.widths.as_ref()?.em_width)
    }

    pub(crate) fn row(&self, snapshot: &TabSnapshot, row: u32) -> Option<LongRow> {
        let widths = self.widths.as_ref()?;
        let len = snapshot.line_len(row);
        if len as usize <= MAX_LINE_LEN {
            return None;
        }
        let mut cursor = self.rows.cursor::<RowCount>(());
        cursor.seek(&RowCount(row), Bias::Right);
        let geometry = cursor
            .item()
            .filter(|entry| cursor.start().0 + entry.gap == row)
            .map(|entry| entry.geometry.clone())
            .filter(|geometry| geometry.len() == len)
            .unwrap_or_else(|| {
                Arc::new(RowGeometry::unmeasured(
                    len,
                    widths.estimate,
                    self.pending_text_id(row),
                ))
            });
        Some(LongRow {
            snapshot: snapshot.clone(),
            row,
            geometry,
            widths: widths.clone(),
        })
    }

    pub(crate) fn clear_pending(&mut self) {
        self.pending = Arc::default();
    }

    fn pending_text_id(&self, row: u32) -> usize {
        let index = self
            .pending
            .partition_point(|pending| pending.rows.end <= row);
        self.pending
            .get(index)
            .filter(|pending| pending.rows.contains(&row))
            .map_or(0, |pending| pending.text_id)
    }

    pub(crate) fn has_pending(&self) -> bool {
        self.widths.is_some() && !self.pending.is_empty()
    }

    pub(crate) fn interpolate(&self, snapshot: &TabSnapshot, edits: &[TabEdit]) -> Self {
        let mut budget = SYNC_MEASURE_LEN;
        self.interpolate_within(snapshot, edits, &mut budget)
    }

    pub(crate) fn replay(self, edits: Vec<(TabSnapshot, Vec<TabEdit>)>) -> Self {
        let mut budget = SYNC_MEASURE_LEN;
        edits
            .into_iter()
            .fold(self, |long_rows, (snapshot, edits)| {
                long_rows.interpolate_within(&snapshot, &edits, &mut budget)
            })
    }

    pub(crate) fn measure(
        &self,
        snapshot: &TabSnapshot,
        requests: &[MeasureRequest],
    ) -> Option<Self> {
        let widths = self.widths.as_ref()?;
        let mut requests = requests
            .iter()
            .filter(|request| request.version == snapshot.version)
            .collect::<Vec<_>>();
        requests.sort_unstable_by_key(|request| request.row);
        requests.dedup_by_key(|request| request.row);
        let mut wrapper = widths.wrapper.lock();
        let mut rows = SumTree::default();
        let mut old_rows = self.rows.cursor::<RowCount>(());
        old_rows.seek(&RowCount(0), Bias::Right);
        let mut changed = false;
        let mut moved = false;
        for request in requests {
            let len = snapshot.line_len(request.row);
            if len as usize <= MAX_LINE_LEN {
                continue;
            }
            copy_rows_before(&mut old_rows, &mut rows, request.row, 0);
            let existing = old_rows
                .item()
                .filter(|entry| old_rows.start().0 + entry.gap == request.row)
                .map(|entry| entry.geometry.clone());
            if existing.is_some() {
                old_rows.next();
            }
            let geometry = existing
                .filter(|geometry| geometry.len() == len)
                .unwrap_or_else(|| {
                    Arc::new(RowGeometry::unmeasured(
                        len,
                        widths.estimate,
                        self.pending_text_id(request.row),
                    ))
                });
            let geometry = match geometry.measure_columns(
                snapshot,
                request.row,
                &request.columns,
                &request.runs,
                &mut wrapper,
                widths,
            ) {
                Some((measured, measured_moved)) => {
                    changed = true;
                    moved |= measured_moved;
                    Arc::new(measured)
                }
                None => geometry,
            };
            push_row(&mut rows, request.row, geometry);
        }
        copy_rows_before(&mut old_rows, &mut rows, u32::MAX, 0);
        changed.then(|| Self {
            widths: Some(widths.clone()),
            rows,
            pending: self.pending.clone(),
            epoch: if moved { next_epoch() } else { self.epoch },
        })
    }

    pub(crate) async fn measure_pending(
        self,
        snapshot: TabSnapshot,
        mut wrapper: LineWrapperHandle,
    ) -> Self {
        let Some(widths) = self.widths.clone() else {
            return self;
        };
        let row_count = snapshot.max_point().row() + 1;
        let mut rows = SumTree::default();
        let mut old_rows = self.rows.cursor::<RowCount>(());
        old_rows.seek(&RowCount(0), Bias::Right);
        let mut steps = 0;
        let mut moved = false;
        for pending in self.pending.iter() {
            let range = pending.rows.start.min(row_count)..pending.rows.end.min(row_count);
            copy_rows_before(&mut old_rows, &mut rows, range.start, 0);
            let mut scan = LongRowScan::new(range.clone());
            loop {
                let step = scan.step(&snapshot);
                steps += 1;
                if steps >= SCAN_YIELD_STEPS {
                    steps = 0;
                    yield_now().await;
                }
                let (row, len) = match step {
                    ScanStep::Done => break,
                    ScanStep::Continue => continue,
                    ScanStep::Row { row, len } => (row, len),
                };
                old_rows.seek_forward(&RowCount(row), Bias::Right);
                let old = old_rows
                    .item()
                    .filter(|entry| old_rows.start().0 + entry.gap == row)
                    .map(|entry| entry.geometry.clone())
                    .filter(|old| old.len() == len);
                let geometry = old.unwrap_or_else(|| {
                    Arc::new(RowGeometry::unmeasured(
                        len,
                        widths.estimate,
                        pending.text_id,
                    ))
                });
                let geometry = if geometry.is_settled() {
                    geometry
                } else {
                    let (measured, measured_moved) = geometry
                        .measure_remaining(&snapshot, row, &mut wrapper, widths.estimate)
                        .await;
                    moved |= measured_moved;
                    Arc::new(measured)
                };
                push_row(&mut rows, row, geometry);
            }
            old_rows.seek_forward(&RowCount(range.end), Bias::Right);
        }
        copy_rows_before(&mut old_rows, &mut rows, u32::MAX, 0);
        let epoch = if moved { next_epoch() } else { self.epoch };
        Self::with_rows(widths, rows, Vec::new(), epoch)
    }

    fn interpolate_within(
        &self,
        snapshot: &TabSnapshot,
        edits: &[TabEdit],
        budget: &mut u32,
    ) -> Self {
        let Some(widths) = &self.widths else {
            return self.clone();
        };
        if edits.is_empty() {
            return self.clone();
        }
        let mut wrapper = widths.wrapper.lock();
        let mut measurer = Measurer {
            snapshot,
            wrapper: &mut wrapper,
            estimate: widths.estimate,
            budget: *budget,
        };
        let mut rows = SumTree::default();
        let mut pending = Vec::new();
        let mut old_rows = self.rows.cursor::<RowCount>(());
        old_rows.seek(&RowCount(0), Bias::Right);
        let mut old_pending = self.pending.iter().rev().cloned().collect::<Vec<_>>();
        let mut row_delta = 0i64;
        let mut edits = edits.iter().peekable();
        while let Some(group) = RowEditGroup::next(&mut edits) {
            copy_rows_before(&mut old_rows, &mut rows, group.old_rows.start, row_delta);
            while let Some(old) = old_pending.pop_if(|old| old.rows.start < group.old_rows.end) {
                if old.rows.start < group.old_rows.start {
                    pending.push(PendingRows {
                        rows: shift_rows(
                            old.rows.start..old.rows.end.min(group.old_rows.start),
                            row_delta,
                        ),
                        text_id: old.text_id,
                    });
                }
                if old.rows.end > group.old_rows.end {
                    old_pending.push(PendingRows {
                        rows: group.old_rows.end..old.rows.end,
                        text_id: old.text_id,
                    });
                }
            }
            old_rows.seek_forward(&RowCount(group.old_rows.end), Bias::Right);
            if group.new_rows.len() > SYNC_SCAN_ROWS {
                pending.push(PendingRows {
                    rows: group.new_rows.clone(),
                    text_id: next_epoch(),
                });
            } else {
                for new_row in group.new_rows.clone() {
                    let len = snapshot.line_len(new_row);
                    if len as usize <= MAX_LINE_LEN {
                        continue;
                    }
                    let text_id = next_epoch();
                    let geometry = self
                        .reuse_geometry(&group, new_row, len, text_id, &mut measurer)
                        .unwrap_or_else(|| measurer.measure_prefix(new_row, len, text_id));
                    if !geometry.is_settled() {
                        pending.push(PendingRows {
                            rows: new_row..new_row + 1,
                            text_id,
                        });
                    }
                    push_row(&mut rows, new_row, Arc::new(geometry));
                }
            }
            row_delta +=
                i64::from(group.new_rows.len() as u32) - i64::from(group.old_rows.len() as u32);
        }
        copy_rows_before(&mut old_rows, &mut rows, u32::MAX, row_delta);
        pending.extend(old_pending.into_iter().rev().map(|old| PendingRows {
            rows: shift_rows(old.rows, row_delta),
            text_id: old.text_id,
        }));
        *budget = measurer.budget;
        Self::with_rows(widths.clone(), rows, pending, self.epoch)
    }

    fn reuse_geometry(
        &self,
        group: &RowEditGroup,
        new_row: u32,
        new_len: u32,
        text_id: usize,
        measurer: &mut Measurer,
    ) -> Option<RowGeometry> {
        if group.is_single_row() {
            let old = self.geometry_at(group.old_rows.start)?;
            return measurer.splice(&old, &group.edits, new_row, new_len, text_id);
        }
        let first = group.edits.first()?;
        let last = group.edits.last()?;
        let prefix = (new_row == group.new_rows.start
            && first.old.start.row() == group.old_rows.start
            && first.new.start.row() == new_row)
            .then(|| self.geometry_at(group.old_rows.start))
            .flatten();
        let suffix = (new_row + 1 == group.new_rows.end
            && last.old.end.row() + 1 == group.old_rows.end
            && last.new.end.row() == new_row)
            .then(|| self.geometry_at(last.old.end.row()))
            .flatten();
        if prefix.is_none() && suffix.is_none() {
            return None;
        }
        let prefix_len = prefix.as_ref().map_or(0, |prefix| prefix.len());
        let prefix_width = prefix.as_ref().map_or(0., |prefix| prefix.width());
        let mut chunks = prefix
            .as_ref()
            .map_or_else(Vec::new, |prefix| prefix.chunks.clone());
        if let Some(suffix) = &suffix {
            chunks.extend(
                suffix
                    .chunks
                    .iter()
                    .map(|chunk| chunk.shifted(i64::from(prefix_len), prefix_width)),
            );
        }
        let edit_start = if prefix.is_some() {
            first.new.start.column()
        } else {
            0
        };
        let (old_end, new_end) = match &suffix {
            Some(_) => (prefix_len + last.old.end.column(), last.new.end.column()),
            None => (prefix_len, new_len),
        };
        let edit = TabEdit {
            old: TabPoint::new(new_row, edit_start)..TabPoint::new(new_row, old_end),
            new: TabPoint::new(new_row, edit_start)..TabPoint::new(new_row, new_end),
        };
        measurer.splice(
            &RowGeometry { chunks, text_id },
            &[&edit],
            new_row,
            new_len,
            text_id,
        )
    }

    fn geometry_at(&self, row: u32) -> Option<Arc<RowGeometry>> {
        let mut cursor = self.rows.cursor::<RowCount>(());
        cursor.seek(&RowCount(row), Bias::Right);
        cursor
            .item()
            .filter(|entry| cursor.start().0 + entry.gap == row)
            .map(|entry| entry.geometry.clone())
    }

    fn with_rows(
        widths: Arc<CharWidths>,
        rows: SumTree<RowEntry>,
        mut pending: Vec<PendingRows>,
        epoch: usize,
    ) -> Self {
        pending.retain(|pending| !pending.rows.is_empty());
        pending.sort_unstable_by_key(|pending| pending.rows.start);
        let mut merged = Vec::<PendingRows>::with_capacity(pending.len());
        for next in pending {
            match merged.last_mut() {
                Some(last)
                    if next.rows.start < last.rows.end
                        || (next.rows.start == last.rows.end && next.text_id == last.text_id) =>
                {
                    last.rows.end = last.rows.end.max(next.rows.end);
                    last.text_id = last.text_id.max(next.text_id);
                }
                _ => merged.push(next),
            }
        }
        Self {
            widths: Some(widths),
            rows,
            pending: Arc::new(merged),
            epoch,
        }
    }
}

fn next_epoch() -> usize {
    NEXT_EPOCH.fetch_add(1, atomic::Ordering::Relaxed)
}

#[derive(Clone)]
struct RowEntry {
    gap: u32,
    geometry: Arc<RowGeometry>,
}

#[derive(Clone, Copy, Default)]
struct RowEntrySummary {
    rows: u32,
    max_width: ScrollPixelOffset,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RowCount(u32);

impl sum_tree::Item for RowEntry {
    type Summary = RowEntrySummary;

    fn summary(&self, (): ()) -> RowEntrySummary {
        RowEntrySummary {
            rows: self.gap + 1,
            max_width: self.geometry.width(),
        }
    }
}

impl sum_tree::ContextLessSummary for RowEntrySummary {
    fn zero() -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.rows += summary.rows;
        self.max_width = self.max_width.max(summary.max_width);
    }
}

impl<'a> sum_tree::Dimension<'a, RowEntrySummary> for RowCount {
    fn zero((): ()) -> Self {
        Self(0)
    }

    fn add_summary(&mut self, summary: &'a RowEntrySummary, (): ()) {
        self.0 += summary.rows;
    }
}

fn push_row(rows: &mut SumTree<RowEntry>, row: u32, geometry: Arc<RowGeometry>) {
    let gap = row.saturating_sub(rows.summary().rows);
    rows.push(RowEntry { gap, geometry }, ());
}

fn copy_rows_before(
    old_rows: &mut Cursor<'_, '_, RowEntry, RowCount>,
    rows: &mut SumTree<RowEntry>,
    limit: u32,
    row_delta: i64,
) {
    let Some(entry) = old_rows.item() else {
        return;
    };
    let row = old_rows.start().0 + entry.gap;
    if row >= limit {
        return;
    }
    push_row(rows, shift_row(row, row_delta), entry.geometry.clone());
    old_rows.next();
    rows.append(old_rows.slice(&RowCount(limit), Bias::Right), ());
}

struct CharWidths {
    wrapper: Mutex<LineWrapperHandle>,
    estimate: ScrollPixelOffset,
    em_width: ScrollPixelOffset,
    base_font: FontId,
    layouts: Mutex<LayoutCache>,
}

type LayoutVariants = SmallVec<[(SmallVec<[FontRun; 1]>, Arc<LineLayout>); 1]>;

#[derive(Default)]
struct LayoutCache {
    layouts: HashMap<String, LayoutVariants>,
    order: VecDeque<String>,
    bytes: usize,
}

impl LayoutCache {
    fn get(&self, text: &str, runs: &[FontRun]) -> Option<Arc<LineLayout>> {
        self.layouts
            .get(text)?
            .iter()
            .find(|(variant_runs, _)| variant_runs.as_slice() == runs)
            .map(|(_, layout)| layout.clone())
    }

    fn insert(&mut self, text: &str, runs: SmallVec<[FontRun; 1]>, layout: Arc<LineLayout>) {
        if text.len() > MAX_CACHED_LAYOUT_BYTES {
            return;
        }
        if let Some(variants) = self.layouts.get_mut(text) {
            if variants.len() >= MAX_CACHED_LAYOUT_VARIANTS {
                variants.remove(0);
            }
            variants.push((runs, layout));
            return;
        }
        while self.bytes + text.len() > MAX_CACHED_LAYOUT_BYTES {
            let Some(oldest) = self.order.pop_front() else {
                break;
            };
            self.bytes -= oldest.len();
            self.layouts.remove(&oldest);
        }
        self.bytes += text.len();
        self.order.push_back(text.to_string());
        self.layouts
            .insert(text.to_string(), smallvec![(runs, layout)]);
    }
}

struct LongRowScan {
    stack: Vec<Range<u32>>,
}

enum ScanStep {
    Row { row: u32, len: u32 },
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
                ScanStep::Row {
                    row: rows.start,
                    len,
                }
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

struct Atom {
    start: u32,
    end: u32,
    start_x: ScrollPixelOffset,
    end_x: ScrollPixelOffset,
}

struct RowGeometry {
    chunks: Vec<GeometryChunk>,
    text_id: usize,
}

#[derive(Clone)]
struct GeometryChunk {
    end: u32,
    x_end: ScrollPixelOffset,
    measured: bool,
    canonical: bool,
    content_end: Option<u32>,
    runs: Option<Arc<[FontRun]>>,
}

impl RowGeometry {
    fn unmeasured(len: u32, estimate: ScrollPixelOffset, text_id: usize) -> Self {
        let mut chunks = Vec::new();
        push_unmeasured(&mut chunks, len, estimate);
        Self { chunks, text_id }
    }

    fn len(&self) -> u32 {
        self.chunks.last().map_or(0, |chunk| chunk.end)
    }

    fn width(&self) -> ScrollPixelOffset {
        self.chunks.last().map_or(0., |chunk| chunk.x_end)
    }

    fn is_measured(&self) -> bool {
        self.chunks.iter().all(|chunk| chunk.measured)
    }

    fn is_settled(&self) -> bool {
        self.chunks.iter().all(GeometryChunk::is_canonical)
    }

    fn trailing_whitespace_start(&self) -> Option<u32> {
        for chunk in self.chunks.iter().rev() {
            if !chunk.measured {
                return None;
            }
            if let Some(content_end) = chunk.content_end {
                return Some(content_end);
            }
        }
        Some(0)
    }

    fn chunk_start(&self, index: usize) -> (u32, ScrollPixelOffset) {
        index
            .checked_sub(1)
            .and_then(|previous| self.chunks.get(previous))
            .map_or((0, 0.), |chunk| (chunk.end, chunk.x_end))
    }

    fn chunk_range(&self, index: usize) -> Range<u32> {
        self.chunk_start(index).0..self.chunks[index].end
    }

    fn chunk_containing(&self, column: u32) -> usize {
        self.chunks
            .partition_point(|chunk| chunk.end <= column)
            .min(self.chunks.len().saturating_sub(1))
    }

    fn chunk_starting_at(&self, column: u32) -> Option<usize> {
        let index = self.chunks.partition_point(|chunk| chunk.end <= column);
        (index < self.chunks.len() && self.chunk_start(index).0 == column).then_some(index)
    }

    fn chunks_in(&self, columns: Range<u32>) -> impl Iterator<Item = usize> + '_ {
        let first = self
            .chunks
            .partition_point(|chunk| chunk.end <= columns.start);
        (first..self.chunks.len()).take_while(move |index| self.chunk_start(*index).0 < columns.end)
    }

    fn is_canonical_boundary(&self, column: u32) -> bool {
        if column == 0 || column >= self.len() {
            return true;
        }
        let index = self.chunks.partition_point(|chunk| chunk.end < column);
        self.chunks.get(index).is_some_and(|chunk| {
            chunk.end == column
                && (chunk.is_canonical()
                    || self
                        .chunks
                        .get(index + 1)
                        .is_some_and(GeometryChunk::is_canonical))
        })
    }

    fn has_unmeasured(&self, columns: &Range<u32>) -> bool {
        if columns.is_empty() {
            return false;
        }
        self.chunks_in(columns.clone())
            .any(|index| !self.chunks[index].measured)
    }

    fn measure_columns(
        &self,
        snapshot: &TabSnapshot,
        row: u32,
        ranges: &[Range<u32>],
        runs: &[RowRuns],
        wrapper: &mut LineWrapper,
        widths: &CharWidths,
    ) -> Option<(Self, bool)> {
        let len = self.len();
        let estimate = widths.estimate;
        let base = widths.base_font;
        let mut chunks = Vec::with_capacity(self.chunks.len() + 2 * ranges.len());
        let mut x_delta = 0.;
        let mut changed = false;
        let mut moved = false;
        for (index, chunk) in self.chunks.iter().enumerate() {
            let columns = self.chunk_range(index);
            if chunk.measured {
                match desired_chunk_runs(runs, columns.clone(), base) {
                    Some(desired) if desired != chunk.runs => {
                        let x = chunks.last().map_or(0., |last: &GeometryChunk| last.x_end);
                        let layout = shape_chunk(
                            snapshot,
                            row,
                            ChunkRequest {
                                columns,
                                cut: false,
                                runs: desired.as_deref(),
                                pieces: false,
                            },
                            wrapper,
                            None,
                        );
                        chunks.push(GeometryChunk {
                            end: chunk.end,
                            x_end: x + layout.width,
                            measured: true,
                            canonical: chunk.canonical,
                            content_end: layout.content_end,
                            runs: desired,
                        });
                        changed = true;
                        moved = true;
                    }
                    _ => chunks.push(chunk.shifted(0, x_delta)),
                }
                x_delta = chunks.last().map_or(0., |last| last.x_end) - chunk.x_end;
                continue;
            }
            let (chunk_start, chunk_x) = self.chunk_start(index);
            let estimated = EstimatedPositions {
                column: chunk_start,
                x: chunk_x,
                estimate,
            };
            let end_is_canonical = self.is_canonical_boundary(chunk.end);
            let mut cursor = chunk_start;
            let mut cursor_is_canonical = self.is_canonical_boundary(chunk_start);
            for range in ranges {
                if range.is_empty() || range.end <= cursor || range.start >= chunk.end {
                    continue;
                }
                let target = range.start.max(cursor);
                let (start, canonical) =
                    if cursor_is_canonical && target - cursor <= MAX_CUT_SCAN_LEN {
                        (
                            last_cut_before(snapshot, row, cursor, target, chunk.end),
                            true,
                        )
                    } else {
                        let Some(start) = grapheme_floor(snapshot, row, len, target, cursor) else {
                            continue;
                        };
                        (start, cursor_is_canonical && start == cursor)
                    };
                if start > cursor {
                    push_unmeasured(&mut chunks, start, estimate);
                }
                let x = chunks.last().map_or(0., |last| last.x_end);
                moved |= estimated.differs_at(start, x);
                let walked = measure_chunks(
                    ChunkWalk {
                        snapshot,
                        row,
                        columns: start..chunk.end,
                        max_len: range.end.saturating_sub(start).max(1),
                        canonical,
                        end_is_canonical,
                        runs,
                        base,
                        estimated: Some(&estimated),
                        cache: None,
                    },
                    x,
                    wrapper,
                    &mut chunks,
                );
                cursor = walked.end;
                cursor_is_canonical = canonical;
                moved |= walked.moved;
                changed = true;
            }
            if cursor < chunk.end {
                push_unmeasured(&mut chunks, chunk.end, estimate);
            }
            x_delta = chunks.last().map_or(0., |last| last.x_end) - chunk.x_end;
            moved |= x_delta.abs() > POSITION_TOLERANCE;
        }
        changed.then_some((
            Self {
                chunks,
                text_id: self.text_id,
            },
            moved,
        ))
    }

    async fn measure_remaining(
        &self,
        snapshot: &TabSnapshot,
        row: u32,
        wrapper: &mut LineWrapper,
        estimate: ScrollPixelOffset,
    ) -> (Self, bool) {
        let len = self.len();
        let base = wrapper.font_id();
        let mut chunks = Vec::with_capacity(self.chunks.len());
        let mut x_delta = 0.;
        let mut moved = false;
        let mut index = 0;
        let mut measured_since_yield = 0;
        let cache = Mutex::new(LayoutCache::default());
        while index < self.chunks.len() {
            let chunk = &self.chunks[index];
            if chunk.is_canonical() {
                chunks.push(chunk.shifted(0, x_delta));
                index += 1;
                continue;
            }
            let (mut start, _) = self.chunk_start(index);
            let mut x = chunks.last().map_or(0., |last: &GeometryChunk| last.x_end);
            loop {
                if start >= len {
                    index = self.chunks.len();
                    break;
                }
                if let Some(next) = self
                    .chunk_starting_at(start)
                    .filter(|next| *next > index && self.chunks[*next].is_canonical())
                {
                    x_delta = x - self.chunk_start(next).1;
                    moved |= x_delta.abs() > POSITION_TOLERANCE;
                    index = next;
                    break;
                }
                let old_index = self.chunk_containing(start);
                let old_chunk = &self.chunks[old_index];
                let (old_start, old_x) = self.chunk_start(old_index);
                let estimated = (!old_chunk.measured).then(|| EstimatedPositions {
                    column: old_start,
                    x: old_x,
                    estimate,
                });
                moved |= old_chunk.measured;
                let walked = measure_chunks(
                    ChunkWalk {
                        snapshot,
                        row,
                        columns: start..len,
                        max_len: 1,
                        canonical: true,
                        end_is_canonical: true,
                        runs: &[],
                        base,
                        estimated: estimated.as_ref(),
                        cache: Some(&cache),
                    },
                    x,
                    wrapper,
                    &mut chunks,
                );
                moved |= walked.moved;
                measured_since_yield += walked.end - start;
                start = walked.end;
                x = walked.x;
                if measured_since_yield >= MEASURE_YIELD_LEN {
                    measured_since_yield = 0;
                    yield_now().await;
                }
            }
        }
        (
            Self {
                chunks,
                text_id: self.text_id,
            },
            moved,
        )
    }
}

struct Measurer<'a> {
    snapshot: &'a TabSnapshot,
    wrapper: &'a mut LineWrapper,
    estimate: ScrollPixelOffset,
    budget: u32,
}

enum Resync {
    At(usize),
    RowEnd,
    Exhausted,
}

impl Measurer<'_> {
    fn measure_prefix(&mut self, row: u32, len: u32, text_id: usize) -> RowGeometry {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut x = 0.;
        while start < len {
            match self.measure_chunk(row, start, len, x, true, &mut chunks) {
                Some((end, x_end)) => (start, x) = (end, x_end),
                None => {
                    push_unmeasured(&mut chunks, len, self.estimate);
                    break;
                }
            }
        }
        RowGeometry { chunks, text_id }
    }

    fn measure_chunk(
        &mut self,
        row: u32,
        start: u32,
        len: u32,
        x: ScrollPixelOffset,
        canonical: bool,
        chunks: &mut Vec<GeometryChunk>,
    ) -> Option<(u32, ScrollPixelOffset)> {
        if self.budget == 0 {
            return None;
        }
        let layout = shape_chunk(
            self.snapshot,
            row,
            ChunkRequest {
                columns: start..len,
                cut: true,
                runs: None,
                pieces: false,
            },
            self.wrapper,
            None,
        );
        self.budget = self.budget.saturating_sub(layout.end - start);
        let x_end = x + layout.width;
        chunks.push(GeometryChunk {
            end: layout.end,
            x_end,
            measured: true,
            canonical,
            content_end: layout.content_end,
            runs: None,
        });
        Some((layout.end, x_end))
    }

    fn splice(
        &mut self,
        old: &RowGeometry,
        edits: &[&TabEdit],
        row: u32,
        new_len: u32,
        text_id: usize,
    ) -> Option<RowGeometry> {
        let edit_delta = |edit: &TabEdit| {
            i64::from(edit.new.end.column()) - i64::from(edit.new.start.column())
                + i64::from(edit.old.start.column())
                - i64::from(edit.old.end.column())
        };
        let len_delta = edits.iter().map(|edit| edit_delta(edit)).sum::<i64>();
        if i64::from(old.len()) + len_delta != i64::from(new_len) {
            return None;
        }

        let mut chunks = Vec::with_capacity(old.chunks.len() + 2);
        let mut copied = 0;
        let mut column_delta = 0i64;
        let mut x_delta = 0.;
        let mut tail_canonical = true;
        let mut edits = edits.iter().copied().peekable();
        while let Some(first) = edits.next() {
            let start_index = old
                .chunk_containing(first.old.start.column().saturating_sub(1))
                .max(copied);
            chunks.extend(
                old.chunks
                    .get(copied..start_index)?
                    .iter()
                    .map(|chunk| chunk.copied(column_delta, x_delta, tail_canonical)),
            );
            let (mut start, mut x) = chunks
                .last()
                .map_or((0, 0.), |chunk| (chunk.end, chunk.x_end));
            let canonical = tail_canonical
                && (start == 0
                    || chunks.last().is_some_and(GeometryChunk::is_canonical)
                    || old.chunks[start_index].is_canonical());
            let mut delta = column_delta + edit_delta(first);
            let mut edits_end = first.new.end.column();
            let resync = loop {
                while let Some(next) = edits.next_if(|next| next.new.start.column() <= start) {
                    delta += edit_delta(next);
                    edits_end = edits_end.max(next.new.end.column());
                }
                if start >= new_len {
                    break Resync::RowEnd;
                }
                if start >= edits_end
                    && let Ok(old_start) = u32::try_from(i64::from(start) - delta)
                    && let Some(index) = old
                        .chunk_starting_at(old_start)
                        .filter(|index| *index >= start_index)
                {
                    break Resync::At(index);
                }
                match self.measure_chunk(row, start, new_len, x, canonical, &mut chunks) {
                    Some((end, x_end)) => (start, x) = (end, x_end),
                    None => break Resync::Exhausted,
                }
            };
            let index = match resync {
                Resync::At(index) => index,
                Resync::RowEnd => return Some(RowGeometry { chunks, text_id }),
                Resync::Exhausted => {
                    let next_edit = edits.peek().map(|next| next.old.start.column());
                    let Some(index) = (start >= edits_end)
                        .then(|| {
                            self.grapheme_aligned_chunk(old, row, start, delta, new_len, next_edit)
                        })
                        .flatten()
                    else {
                        push_unmeasured(&mut chunks, new_len, self.estimate);
                        return Some(RowGeometry { chunks, text_id });
                    };
                    let gap_end = shift_column(old.chunk_start(index).0, delta);
                    let layout = shape_chunk(
                        self.snapshot,
                        row,
                        ChunkRequest {
                            columns: start..gap_end,
                            cut: false,
                            runs: None,
                            pieces: false,
                        },
                        self.wrapper,
                        None,
                    );
                    if gap_end > start {
                        x += layout.width;
                        chunks.push(GeometryChunk {
                            end: gap_end,
                            x_end: x,
                            measured: true,
                            canonical: false,
                            content_end: layout.content_end,
                            runs: None,
                        });
                    }
                    tail_canonical = false;
                    index
                }
            };
            column_delta = delta;
            x_delta = x - old.chunk_start(index).1;
            copied = index;
            tail_canonical &= canonical;
        }
        chunks.extend(
            old.chunks
                .get(copied..)?
                .iter()
                .map(|chunk| chunk.copied(column_delta, x_delta, tail_canonical)),
        );
        Some(RowGeometry { chunks, text_id })
    }

    fn grapheme_aligned_chunk(
        &self,
        old: &RowGeometry,
        row: u32,
        start: u32,
        delta: i64,
        new_len: u32,
        next_edit: Option<u32>,
    ) -> Option<usize> {
        let old_start = u32::try_from(i64::from(start) - delta).ok()?;
        let first = old.chunks.partition_point(|chunk| chunk.end < old_start) + 1;
        (first..old.chunks.len())
            .take(MAX_RESYNC_ATTEMPTS)
            .take_while(|index| next_edit.is_none_or(|next| old.chunk_start(*index).0 <= next))
            .find(|index| {
                let column = shift_column(old.chunk_start(*index).0, delta);
                grapheme_floor(self.snapshot, row, new_len, column, start) == Some(column)
            })
    }
}

impl GeometryChunk {
    fn is_canonical(&self) -> bool {
        self.measured && self.canonical
    }

    fn shifted(&self, column_delta: i64, x_delta: ScrollPixelOffset) -> Self {
        Self {
            end: shift_column(self.end, column_delta),
            x_end: self.x_end + x_delta,
            measured: self.measured,
            canonical: self.canonical,
            content_end: self
                .content_end
                .map(|content_end| shift_column(content_end, column_delta)),
            runs: self.runs.clone(),
        }
    }

    fn copied(&self, column_delta: i64, x_delta: ScrollPixelOffset, canonical: bool) -> Self {
        let mut chunk = self.shifted(column_delta, x_delta);
        chunk.canonical &= canonical;
        chunk
    }
}

struct RowEditGroup<'a> {
    old_rows: Range<u32>,
    new_rows: Range<u32>,
    edits: Vec<&'a TabEdit>,
}

impl<'a> RowEditGroup<'a> {
    fn next(edits: &mut Peekable<impl Iterator<Item = &'a TabEdit>>) -> Option<Self> {
        let first = edits.next()?;
        let mut group = Self {
            old_rows: first.old.start.row()..first.old.end.row() + 1,
            new_rows: first.new.start.row()..first.new.end.row() + 1,
            edits: vec![first],
        };
        while let Some(edit) = edits.next_if(|edit| edit.old.start.row() < group.old_rows.end) {
            group.old_rows.end = group.old_rows.end.max(edit.old.end.row() + 1);
            group.new_rows.end = group.new_rows.end.max(edit.new.end.row() + 1);
            group.edits.push(edit);
        }
        Some(group)
    }

    fn is_single_row(&self) -> bool {
        self.old_rows.len() == 1 && self.new_rows.len() == 1
    }
}

struct ChunkLayout {
    end: u32,
    width: ScrollPixelOffset,
    content_end: Option<u32>,
    atoms: Vec<(u32, ScrollPixelOffset)>,
    pieces: Vec<TextPiece>,
}

struct ChunkRequest<'a> {
    columns: Range<u32>,
    cut: bool,
    runs: Option<&'a [FontRun]>,
    pieces: bool,
}

#[derive(Default)]
struct PendingText {
    start: u32,
    text: String,
    atoms: Vec<(u32, usize)>,
}

impl ChunkLayout {
    fn push_text(
        &mut self,
        pending: &mut PendingText,
        request: &ChunkRequest<'_>,
        wrapper: &mut LineWrapper,
        cache: Option<&Mutex<LayoutCache>>,
    ) {
        if pending.text.is_empty() {
            return;
        }
        let offset = (pending.start - request.columns.start) as usize;
        let runs = piece_runs(
            request.runs,
            offset..offset + pending.text.len(),
            wrapper.font_id(),
        );
        let layout = shape_text(&pending.text, runs, wrapper, cache);
        let x = self.width;
        let x_end = x + f64::from(layout.width);
        let mut glyphs = layout.runs.iter().flat_map(|run| &run.glyphs).peekable();
        for (atom_start, offset) in pending.atoms.drain(..) {
            while glyphs.next_if(|glyph| glyph.index < offset).is_some() {}
            let glyph_x = glyphs.peek().map_or(layout.width, |glyph| glyph.position.x);
            let previous_x = self.atoms.last().map_or(x, |(_, previous_x)| *previous_x);
            let atom_x = (x + f64::from(glyph_x)).clamp(previous_x, x_end.max(previous_x));
            self.atoms.push((atom_start, atom_x));
        }
        self.width = x_end;
        if request.pieces {
            let start = pending.start;
            let text = mem::take(&mut pending.text);
            self.pieces.push(TextPiece {
                columns: start..start + text.len() as u32,
                x,
                text,
                layout,
            });
        } else {
            pending.text.clear();
        }
    }
}

fn shape_text(
    text: &str,
    runs: SmallVec<[FontRun; 1]>,
    wrapper: &mut LineWrapper,
    cache: Option<&Mutex<LayoutCache>>,
) -> Arc<LineLayout> {
    let Some(cache) = cache else {
        return Arc::new(wrapper.layout_line(text, &runs));
    };
    if let Some(layout) = cache.lock().get(text, &runs) {
        return layout;
    }
    let layout = Arc::new(wrapper.layout_line(text, &runs));
    cache.lock().insert(text, runs, layout.clone());
    layout
}

fn shape_chunk(
    snapshot: &TabSnapshot,
    row: u32,
    request: ChunkRequest<'_>,
    wrapper: &mut LineWrapper,
    cache: Option<&Mutex<LayoutCache>>,
) -> ChunkLayout {
    let start = request.columns.start;
    let mut layout = ChunkLayout {
        end: request.columns.end,
        width: 0.,
        content_end: None,
        atoms: Vec::new(),
        pieces: Vec::new(),
    };
    let mut pending = PendingText::default();
    let mut cutter = ChunkCutter::new(start);
    for_each_atom(snapshot, row, request.columns.clone(), |atom| {
        if atom.start < start {
            return true;
        }
        if request.cut && cutter.cuts_before(&atom) {
            layout.end = atom.start;
            return false;
        }
        cutter.feed(&atom);
        if !atom.is_whitespace {
            layout.content_end = Some(atom.start + atom.len);
        }
        let width = match atom.kind {
            AtomKind::Text { text } => match standalone_invisible(text) {
                Some(character) => {
                    f64::from(wrapper.width_for_char(replacement(character).unwrap_or(character)))
                }
                None => {
                    if pending.text.is_empty() {
                        pending.start = atom.start;
                    }
                    pending.atoms.push((atom.start, pending.text.len()));
                    pending.text.push_str(text);
                    return true;
                }
            },
            AtomKind::Renderer {
                measured_width: Some(width),
                ..
            } => f64::from(width),
            AtomKind::Renderer {
                measured_width: None,
                texts,
            } => texts
                .iter()
                .flat_map(|text| text.chars())
                .map(|character| f64::from(wrapper.width_for_char(character)))
                .sum(),
        };
        layout.push_text(&mut pending, &request, wrapper, cache);
        layout.atoms.push((atom.start, layout.width));
        layout.width += width;
        true
    });
    layout.push_text(&mut pending, &request, wrapper, cache);
    layout
}

fn standalone_invisible(text: &str) -> Option<char> {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(character), None) if is_invisible(character) => Some(character),
        _ => None,
    }
}

struct RawAtom<'a> {
    start: u32,
    len: u32,
    kind: AtomKind<'a>,
    is_whitespace: bool,
}

enum AtomKind<'a> {
    Text {
        text: &'a str,
    },
    Renderer {
        measured_width: Option<Pixels>,
        texts: &'a [&'a str],
    },
}

struct PendingRenderer<'a> {
    id: ChunkRendererId,
    start: u32,
    len: u32,
    measured_width: Option<Pixels>,
    texts: SmallVec<[&'a str; 2]>,
}

fn starts_new_atom(cluster: &str, offset: usize, character: char) -> bool {
    offset > 0
        && ((cluster.len() > MAX_ATOM_LEN as usize && !is_regional_indicator(character))
            || is_cluster_boundary(cluster, offset))
}

fn densest_segments(pieces: &[Piece], budget: u32) -> SmallVec<[Range<u32>; 1]> {
    let mut order = (0..pieces.len())
        .filter(|&index| pieces[index].density > 0.)
        .collect::<Vec<_>>();
    let mut kept = vec![false; pieces.len()];
    let mut remaining = budget;
    if let Some(&first_visible) = order.first() {
        kept[first_visible] = true;
    }
    order.sort_by(|&left, &right| {
        pieces[right]
            .density
            .total_cmp(&pieces[left].density)
            .then(left.cmp(&right))
    });
    for index in order {
        let len = pieces[index].len();
        if !kept[index] && len <= remaining {
            kept[index] = true;
            remaining -= len;
        }
    }
    let mut segments = SmallVec::<[Range<u32>; 1]>::new();
    for (piece, _) in pieces.iter().zip(kept).filter(|(_, kept)| *kept) {
        match segments.last_mut() {
            Some(last) if last.end == piece.columns.start => last.end = piece.columns.end,
            _ => segments.push(piece.columns.clone()),
        }
    }
    segments
}

fn for_each_atom(
    snapshot: &TabSnapshot,
    row: u32,
    columns: Range<u32>,
    mut f: impl FnMut(RawAtom<'_>) -> bool,
) {
    if columns.is_empty() {
        return;
    }
    let (start, chunks) = row_chunks(snapshot, row, columns);
    let mut cluster = String::new();
    let mut cluster_is_whitespace = true;
    let mut cluster_start = start;
    let mut renderer = None::<PendingRenderer<'_>>;
    let mut column = start;
    for chunk in chunks {
        if let Some(chunk_renderer) = &chunk.renderer {
            match &mut renderer {
                Some(pending) if pending.id == chunk_renderer.id => {
                    pending.len += chunk.text.len() as u32;
                    pending.texts.push(chunk.text);
                }
                _ => {
                    if !emit_cluster(
                        &mut cluster,
                        &mut cluster_is_whitespace,
                        cluster_start,
                        &mut f,
                    ) || !emit_renderer(renderer.take(), &mut f)
                    {
                        return;
                    }
                    renderer = Some(PendingRenderer {
                        id: chunk_renderer.id,
                        start: column,
                        len: chunk.text.len() as u32,
                        measured_width: chunk_renderer.measured_width,
                        texts: smallvec![chunk.text],
                    });
                }
            }
            column += chunk.text.len() as u32;
            cluster_start = column;
            continue;
        }
        if !emit_renderer(renderer.take(), &mut f) {
            return;
        }
        for character in chunk.text.chars() {
            let cluster_len = cluster.len();
            cluster.push(character);
            if starts_new_atom(&cluster, cluster_len, character) {
                cluster.truncate(cluster_len);
                if !emit_cluster(
                    &mut cluster,
                    &mut cluster_is_whitespace,
                    cluster_start,
                    &mut f,
                ) {
                    return;
                }
                cluster.push(character);
                cluster_start = column;
            }
            cluster_is_whitespace &=
                !chunk.is_inlay && character.is_whitespace() && !is_invisible(character);
            column += character.len_utf8() as u32;
        }
    }
    if emit_renderer(renderer.take(), &mut f) {
        emit_cluster(
            &mut cluster,
            &mut cluster_is_whitespace,
            cluster_start,
            &mut f,
        );
    }
}

fn emit_renderer(
    renderer: Option<PendingRenderer<'_>>,
    f: &mut impl FnMut(RawAtom<'_>) -> bool,
) -> bool {
    let Some(renderer) = renderer else {
        return true;
    };
    f(RawAtom {
        start: renderer.start,
        len: renderer.len,
        kind: AtomKind::Renderer {
            measured_width: renderer.measured_width,
            texts: &renderer.texts,
        },
        is_whitespace: false,
    })
}

fn emit_cluster(
    cluster: &mut String,
    cluster_is_whitespace: &mut bool,
    start: u32,
    f: &mut impl FnMut(RawAtom<'_>) -> bool,
) -> bool {
    if cluster.is_empty() {
        return true;
    }
    let is_whitespace = mem::replace(cluster_is_whitespace, true);
    let keep_going = f(RawAtom {
        start,
        len: cluster.len() as u32,
        kind: AtomKind::Text { text: cluster },
        is_whitespace,
    });
    cluster.clear();
    keep_going
}

fn is_regional_indicator(character: char) -> bool {
    ('\u{1f1e6}'..='\u{1f1ff}').contains(&character)
}

#[derive(Default)]
pub(super) struct StandaloneInvisibles {
    atom: String,
}

impl StandaloneInvisibles {
    pub(super) fn reset(&mut self) {
        self.atom.clear();
    }

    pub(super) fn find(&mut self, text: &str, next: Option<char>) -> SmallVec<[Range<usize>; 1]> {
        let mut found = SmallVec::new();
        let mut candidate = None::<Range<usize>>;
        let mut confirm = |candidate: &mut Option<Range<usize>>, starts_atom: bool| {
            if let Some(range) = candidate.take()
                && starts_atom
            {
                found.push(range);
            }
        };
        let ascii_suffix = text.bytes().rev().take_while(u8::is_ascii).count();
        let general_end = (text.len() - ascii_suffix + 1).min(text.len());
        for (offset, character) in text[..general_end].char_indices() {
            let starts_atom = self.push(character);
            confirm(&mut candidate, starts_atom);
            if starts_atom && is_invisible(character) {
                candidate = Some(offset..offset + character.len_utf8());
            }
        }
        let bytes = text.as_bytes();
        let mut last_atom_start = None;
        for offset in general_end..text.len() {
            let starts_atom = !(bytes[offset - 1] == b'\r' && bytes[offset] == b'\n');
            confirm(&mut candidate, starts_atom);
            if starts_atom {
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
            let starts_atom = next.is_none_or(|next| self.would_start_atom(next));
            confirm(&mut candidate, starts_atom);
        }
        found
    }

    fn push(&mut self, character: char) -> bool {
        let len = self.atom.len();
        self.atom.push(character);
        let starts_atom = len == 0 || starts_new_atom(&self.atom, len, character);
        if starts_atom && len > 0 {
            self.atom.clear();
            self.atom.push(character);
        }
        starts_atom
    }

    fn would_start_atom(&mut self, character: char) -> bool {
        let len = self.atom.len();
        if len == 0 {
            return true;
        }
        self.atom.push(character);
        let starts_atom = starts_new_atom(&self.atom, len, character);
        self.atom.truncate(len);
        starts_atom
    }
}

pub(super) fn is_cluster_boundary(text: &str, offset: usize) -> bool {
    let bytes = text.as_bytes();
    if let (Some(&previous), Some(&next)) = (bytes.get(offset.wrapping_sub(1)), bytes.get(offset))
        && previous.is_ascii()
        && next.is_ascii()
    {
        return !(previous == b'\r' && next == b'\n');
    }
    GraphemeCursor::new(offset, text.len(), true)
        .is_boundary(text, 0)
        .unwrap_or(true)
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

struct ChunkWalk<'a> {
    snapshot: &'a TabSnapshot,
    row: u32,
    columns: Range<u32>,
    max_len: u32,
    canonical: bool,
    end_is_canonical: bool,
    runs: &'a [RowRuns],
    base: FontId,
    estimated: Option<&'a EstimatedPositions>,
    cache: Option<&'a Mutex<LayoutCache>>,
}

struct Walked {
    end: u32,
    x: ScrollPixelOffset,
    moved: bool,
}

fn measure_chunks(
    walk: ChunkWalk<'_>,
    mut x: ScrollPixelOffset,
    wrapper: &mut LineWrapper,
    chunks: &mut Vec<GeometryChunk>,
) -> Walked {
    let mut chunk_start = walk.columns.start;
    let mut moved = false;
    while chunk_start < walk.columns.end && chunk_start - walk.columns.start < walk.max_len {
        let runs = runs_from(walk.runs, chunk_start);
        let layout = shape_chunk(
            walk.snapshot,
            walk.row,
            ChunkRequest {
                columns: chunk_start..walk.columns.end,
                cut: true,
                runs: runs.as_deref(),
                pieces: false,
            },
            wrapper,
            walk.cache,
        );
        if let Some(estimated) = walk.estimated {
            moved |= layout
                .atoms
                .iter()
                .any(|(atom_start, atom_x)| estimated.differs_at(*atom_start, x + atom_x));
        }
        x += layout.width;
        if let Some(estimated) = walk.estimated {
            moved |= estimated.differs_at(layout.end, x);
        }
        chunks.push(GeometryChunk {
            end: layout.end,
            x_end: x,
            measured: true,
            canonical: walk.canonical && (layout.end < walk.columns.end || walk.end_is_canonical),
            content_end: layout.content_end,
            runs: effective_runs(runs.as_deref(), layout.end - chunk_start, walk.base),
        });
        chunk_start = layout.end;
    }
    Walked {
        end: chunk_start,
        x,
        moved,
    }
}

struct EstimatedPositions {
    column: u32,
    x: ScrollPixelOffset,
    estimate: ScrollPixelOffset,
}

impl EstimatedPositions {
    fn differs_at(&self, column: u32, x: ScrollPixelOffset) -> bool {
        let estimated_x = self.x + f64::from(column.saturating_sub(self.column)) * self.estimate;
        (x - estimated_x).abs() > POSITION_TOLERANCE
    }
}

fn push_unmeasured(
    chunks: &mut Vec<GeometryChunk>,
    end: u32,
    estimate: ScrollPixelOffset,
) -> ScrollPixelOffset {
    let (start, x) = chunks
        .last()
        .map_or((0, 0.), |chunk| (chunk.end, chunk.x_end));
    let x_end = x + f64::from(end - start) * estimate;
    match chunks.last_mut() {
        Some(last) if !last.measured => {
            last.end = end;
            last.x_end = x_end;
        }
        _ => chunks.push(GeometryChunk {
            end,
            x_end,
            measured: false,
            canonical: false,
            content_end: None,
            runs: None,
        }),
    }
    x_end
}

struct ChunkCutter {
    start: u32,
    hash: u64,
    whitespace_end: u32,
    after_whitespace: bool,
}

impl ChunkCutter {
    fn new(start: u32) -> Self {
        Self {
            start,
            hash: 0,
            whitespace_end: start,
            after_whitespace: false,
        }
    }

    fn cuts_before(&self, atom: &RawAtom<'_>) -> bool {
        let offset = atom.start - self.start;
        if offset >= MAX_CHUNK_LEN {
            return true;
        }
        offset >= MIN_CHUNK_LEN
            && if self.after_whitespace {
                self.hash & WHITESPACE_CUT_MASK == 0
            } else {
                atom.start - self.whitespace_end >= DENSE_RUN_LEN && self.hash & DENSE_CUT_MASK == 0
            }
    }

    fn feed(&mut self, atom: &RawAtom<'_>) {
        let mut feed_bytes = |text: &str| {
            for byte in text.bytes() {
                self.hash = (self.hash << 1).wrapping_add(GEAR[byte as usize]);
            }
        };
        match &atom.kind {
            AtomKind::Text { text } => feed_bytes(text),
            AtomKind::Renderer { texts, .. } => texts.iter().for_each(|text| feed_bytes(text)),
        }
        if atom.is_whitespace {
            self.whitespace_end = atom.start + atom.len;
        }
        self.after_whitespace = atom.is_whitespace;
    }
}

const fn gear_table() -> [u64; 256] {
    let mut table = [0; 256];
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let mut index = 0;
    while index < table.len() {
        state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        table[index] = value ^ (value >> 31);
        index += 1;
    }
    table
}

fn next_cut(snapshot: &TabSnapshot, row: u32, start: u32, limit: u32) -> u32 {
    let mut cutter = ChunkCutter::new(start);
    let mut end = limit;
    for_each_atom(snapshot, row, start..limit, |atom| {
        if atom.start < start {
            return true;
        }
        if cutter.cuts_before(&atom) {
            end = atom.start;
            return false;
        }
        cutter.feed(&atom);
        true
    });
    end
}

fn last_cut_before(snapshot: &TabSnapshot, row: u32, start: u32, target: u32, limit: u32) -> u32 {
    let mut cut = start;
    while cut < limit {
        let next = next_cut(snapshot, row, cut, limit);
        if next > target || next >= limit || next <= cut {
            break;
        }
        cut = next;
    }
    cut
}

fn piece_runs(
    runs: Option<&[FontRun]>,
    range: Range<usize>,
    base: FontId,
) -> SmallVec<[FontRun; 1]> {
    let mut sliced = SmallVec::<[FontRun; 1]>::new();
    let mut run_start = 0;
    for run in runs.into_iter().flatten() {
        let run_end = run_start + run.len;
        let start = run_start.max(range.start);
        let end = run_end.min(range.end);
        if start < end {
            sliced.push(FontRun {
                len: end - start,
                font_id: run.font_id,
            });
        }
        run_start = run_end;
        if run_start >= range.end {
            break;
        }
    }
    let covered = sliced.iter().map(|run| run.len).sum::<usize>();
    if covered < range.len() {
        sliced.push(FontRun {
            len: range.len() - covered,
            font_id: base,
        });
    }
    sliced
}

fn effective_runs(runs: Option<&[FontRun]>, len: u32, base: FontId) -> Option<Arc<[FontRun]>> {
    let runs = piece_runs(Some(runs?), 0..len as usize, base);
    if runs.len() == 1 && runs[0].font_id == base {
        None
    } else {
        Some(runs.into_iter().collect())
    }
}

fn desired_chunk_runs(
    runs: &[RowRuns],
    columns: Range<u32>,
    base: FontId,
) -> Option<Option<Arc<[FontRun]>>> {
    let row_runs = runs.iter().find(|row_runs| row_runs.covers(&columns))?;
    let len = columns.end - columns.start;
    Some(effective_runs(Some(&row_runs.slice(columns)), len, base))
}

fn runs_from(runs: &[RowRuns], column: u32) -> Option<Vec<FontRun>> {
    let row_runs = runs
        .iter()
        .find(|row_runs| row_runs.start <= column && column < row_runs.end())?;
    Some(row_runs.slice(column..column.saturating_add(MAX_CHUNK_LEN + MAX_ATOM_LEN)))
}

fn grapheme_floor(
    snapshot: &TabSnapshot,
    row: u32,
    len: u32,
    column: u32,
    floor: u32,
) -> Option<u32> {
    if column <= floor {
        return Some(floor);
    }
    if column >= len {
        return Some(len);
    }
    let end = clip_column(snapshot, row, column + 1, Bias::Right).map_or(len, |end| end.min(len));
    let mut context = GRAPHEME_CONTEXT_LEN;
    loop {
        let start = clip_column(
            snapshot,
            row,
            column.saturating_sub(context).max(floor),
            Bias::Left,
        )
        .map_or(floor, |start| start.clamp(floor, column));
        let (start, chunks) = row_chunks(snapshot, row, start..end);
        let start = start.max(floor);
        let mut text = String::new();
        let mut text_start = start;
        let mut position = start;
        let mut renderer_run = None;
        for chunk in chunks {
            let chunk_end = position + chunk.text.len() as u32;
            if let Some(renderer) = &chunk.renderer {
                let renderer_start = match renderer_run {
                    Some((id, renderer_start)) if id == renderer.id => renderer_start,
                    _ => position,
                };
                renderer_run = Some((renderer.id, renderer_start));
                if renderer_start == column {
                    return Some(column);
                }
                if position > column {
                    break;
                }
                if column < chunk_end {
                    return Some(renderer_start.max(floor));
                }
                text.clear();
                text_start = chunk_end;
            } else {
                renderer_run = None;
                text.push_str(chunk.text);
            }
            position = chunk_end;
        }
        if text_start >= column {
            return Some(column);
        }
        let base = if text_start > start {
            text_start
        } else {
            floor
        };
        let chunk_start = (text_start - base) as usize;
        let offset = chunk_start + text.floor_char_boundary((column - text_start) as usize);
        let mut cursor = GraphemeCursor::new(offset, (len - base) as usize, true);
        let boundary = match cursor.is_boundary(&text, chunk_start) {
            Ok(true) => Ok(Some(offset)),
            Ok(false) => cursor.prev_boundary(&text, chunk_start),
            Err(error) => Err(error),
        };
        match boundary {
            Ok(boundary) => return Some(base + boundary.unwrap_or(0) as u32),
            Err(_) if start > floor && context < MAX_GRAPHEME_CONTEXT_LEN => {
                context = context.saturating_mul(2)
            }
            Err(_) => return None,
        }
    }
}

fn shift_row(row: u32, delta: i64) -> u32 {
    (i64::from(row) + delta).clamp(0, i64::from(u32::MAX)) as u32
}

fn shift_rows(rows: Range<u32>, delta: i64) -> Range<u32> {
    shift_row(rows.start, delta)..shift_row(rows.end, delta)
}

fn shift_column(column: u32, delta: i64) -> u32 {
    (i64::from(column) + delta).max(0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        EditorStyle, SelectionsCollection,
        display_map::{
            ChunkRendererId, ChunkReplacement, Crease, DisplayMap, DisplayPoint, DisplayRow,
            DisplaySnapshot, FoldPlaceholder, HighlightKey, RendererWidthsKey,
        },
        inlays::Inlay,
        test::test_font,
    };
    use gpui::{
        AppContext as _, BorrowAppContext as _, Entity, HighlightStyle, SharedString,
        TestAppContext, px, rgba,
    };
    use language::{
        Buffer, Diagnostic, DiagnosticEntry, DiagnosticSet, Point, Selection, SelectionGoal,
    };
    use lsp::LanguageServerId;
    use multi_buffer::{MultiBuffer, MultiBufferOffset};
    use project::{InlayId, project_settings::DiagnosticSeverity};
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{num::NonZeroU32, ops::RangeInclusive};
    use text::PointUtf16;
    use theme::LoadThemes;
    use unicode_segmentation::UnicodeSegmentation as _;

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
                let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
                assert_round_trips(&snapshot);
            }
            cx.run_until_parked();
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_matches_char_widths(&snapshot, &mut wrapper);
            assert_matches_fresh_geometry(&snapshot);
        }
    }

    #[gpui::test(iterations = 5)]
    async fn test_long_row_geometry_is_independent_of_edit_history(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        let (buffer, map) = build_display_map(&random_text(&mut rng, 1_500..2_500), 4, cx);
        cx.run_until_parked();
        for _ in 0..200 {
            let text = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text());
            let mut offset = if rng.random_bool(0.3) {
                0
            } else {
                rng.random_range(0..=text.len())
            };
            while !text.is_char_boundary(offset) {
                offset -= 1;
            }
            let mut end = (offset + rng.random_range(0..3)).min(text.len());
            while !text.is_char_boundary(end) {
                end += 1;
            }
            let inserted = random_text(&mut rng, 0..3).replace('\n', "");
            buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(offset)..MultiBufferOffset(end), inserted)],
                    None,
                    cx,
                )
            });
            map.update(cx, |map, cx| map.snapshot(cx));
        }
        cx.run_until_parked();
        let text = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text());
        let (_, fresh_map) = build_display_map(&text, 4, cx);
        cx.run_until_parked();

        let chunks = |map: &Entity<DisplayMap>, cx: &mut TestAppContext| {
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
            assert!(long_row.geometry.is_settled());
            long_row
                .geometry
                .chunks
                .iter()
                .map(|chunk| (chunk.end, chunk.x_end))
                .collect::<Vec<_>>()
        };
        let edited = chunks(&map, cx);
        let fresh = chunks(&fresh_map, cx);
        assert_eq!(
            edited.iter().map(|(end, _)| *end).collect::<Vec<_>>(),
            fresh.iter().map(|(end, _)| *end).collect::<Vec<_>>()
        );
        for ((end, edited_x), (_, fresh_x)) in edited.iter().zip(&fresh) {
            assert!(
                (edited_x - fresh_x).abs() < 1e-6,
                "chunk ending at {end}: {edited_x} != {fresh_x}"
            );
        }
        assert!(edited.len() as u32 <= text.len() as u32 / MIN_CHUNK_LEN + 1);
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
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
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
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.line_len(DisplayRow(0)), 129 * 8 + 50 * 2);
        assert_matches_char_widths(&snapshot, &mut wrapper);
    }

    #[gpui::test]
    async fn test_long_rows_keep_grapheme_clusters_across_chunk_splices(cx: &mut TestAppContext) {
        let text = (0..150)
            .map(|index| {
                let letter = char::from(b'a' + (index % 26) as u8);
                format!("{letter}\u{200d}\u{1f600}b\u{1f1fa}\u{1f1f8}e\u{301}")
            })
            .collect::<String>();
        let (buffer, map) = build_display_map(&text, 4, cx);
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_cluster_atoms(&snapshot);

        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        let boundary = long_row
            .geometry
            .chunks
            .iter()
            .map(|chunk| chunk.end as usize)
            .find(|&end| text[end..].starts_with('\u{1f600}'))
            .expect("a chunk boundary before an emoji");
        let joiner_start = boundary - "a\u{200d}".len();
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(
                    MultiBufferOffset(joiner_start)..MultiBufferOffset(joiner_start + 1),
                    "\u{1f600}",
                )],
                None,
                cx,
            )
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_cluster_atoms(&snapshot);
        cx.run_until_parked();
        assert_cluster_atoms(&map.update(cx, |map, cx| map.snapshot(cx)));
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
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
            long_row.x_for_column(10 + "\u{25fc}".len() as u32) - long_row.x_for_column(10)
        };
        let mut wrapper = cx.update(|cx| cx.text_system().line_wrapper(test_font(), FONT_SIZE));
        assert_eq!(
            inlay_width(&map, cx),
            f64::from(wrapper.width_for_char('\u{25fc}'))
        );

        let key = |font_size: f32| RendererWidthsKey {
            rem_size: px(16.),
            font: test_font(),
            font_size: px(font_size),
        };
        let renderer_widths = [(ChunkRendererId::Inlay(InlayId::Color(0)), px(37.))];
        assert!(map.update(cx, |map, cx| {
            map.update_renderer_widths(key(14.), renderer_widths, cx)
        }));
        assert_eq!(inlay_width(&map, cx), 37.);
        assert!(!map.update(cx, |map, cx| {
            map.update_renderer_widths(key(14.), renderer_widths, cx)
        }));

        assert!(map.update(cx, |map, cx| {
            map.update_renderer_widths(key(28.), [], cx)
        }));
        assert_eq!(
            inlay_width(&map, cx),
            f64::from(wrapper.width_for_char('\u{25fc}'))
        );
    }

    #[gpui::test]
    async fn test_soft_wrap_counts_renderer_width_once(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map("a", 4, cx);
        let position = buffer.read_with(cx, |buffer, cx| {
            buffer.snapshot(cx).anchor_after(MultiBufferOffset(1))
        });
        let inlay = Inlay::repl_result(0, position, "r".repeat(300).as_str());
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
                },
                [(ChunkRendererId::Inlay(InlayId::ReplResult(0)), px(100.))],
                cx,
            )
        });
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.max_point().row(), DisplayRow(0));
    }

    #[gpui::test]
    async fn test_invisibles_are_replaced_regardless_of_chunk_boundaries(cx: &mut TestAppContext) {
        let text = format!("{}ab\u{200c}c a\u{200b}b", "x".repeat(1_100));
        let (buffer, map) = build_display_map(&text, 4, cx);
        let highlight_end = text.find('\u{200c}').unwrap_or_default();
        let highlighted = text.find("ab").unwrap_or_default()..highlight_end;
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
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
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
        assert_eq!(replaced, ["\u{200b}"]);

        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        let zwnj = highlight_end as u32;
        assert_eq!(long_row.atom_range(zwnj).start, zwnj - 1);
    }

    #[gpui::test]
    async fn test_long_rows_resync_grapheme_boundaries_after_splices(cx: &mut TestAppContext) {
        let text = format!("{}{}", "\u{1f1e8}\u{1f1e6}".repeat(40), "ab".repeat(600));
        let (buffer, map) = build_display_map(&text, 4, cx);
        cx.run_until_parked();
        assert_cluster_atoms(&map.update(cx, |map, cx| map.snapshot(cx)));

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(MultiBufferOffset(0)..MultiBufferOffset(0), "\u{1f1e6}")],
                None,
                cx,
            )
        });
        assert_cluster_atoms(&map.update(cx, |map, cx| map.snapshot(cx)));
        cx.run_until_parked();
        assert_cluster_atoms(&map.update(cx, |map, cx| map.snapshot(cx)));
    }

    #[gpui::test]
    async fn test_long_rows_give_up_resync_after_long_runs(cx: &mut TestAppContext) {
        let text = format!("{}{}", "\u{1f1e8}\u{1f1e6}".repeat(300), "ab".repeat(600));
        let (buffer, map) = build_display_map(&text, 4, cx);
        cx.run_until_parked();

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(MultiBufferOffset(0)..MultiBufferOffset(0), "\u{1f1e6}")],
                None,
                cx,
            )
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        assert_eq!(long_row.len() as usize, text.len() + "\u{1f1e6}".len());
        cx.run_until_parked();
        assert_cluster_atoms(&map.update(cx, |map, cx| map.snapshot(cx)));
    }

    #[gpui::test]
    async fn test_long_row_windows_cover_dense_visible_text(cx: &mut TestAppContext) {
        let text = format!("a{}", "\u{301}".repeat(500)).repeat(300);
        let (_, map) = build_display_map(&text, 4, cx);
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        let viewport_width = long_row.x_for_column(100 * 1_001);
        let window = long_row.window(0., TextAlign::Left, viewport_width);
        assert_eq!(window.segments().len(), 1);
        let columns = window.segments()[0].clone();
        assert!(columns.end - columns.start > MAX_WINDOW_LEN);
        assert!(long_row.x_for_column(columns.end) >= viewport_width);
    }

    #[gpui::test]
    async fn test_long_rows_cap_oversized_grapheme_clusters(cx: &mut TestAppContext) {
        let grapheme = format!("a{}", "\u{301}".repeat(512));
        let text = format!("{grapheme}b{}", "x".repeat(1_024));
        let (_, map) = build_display_map(&text, 4, cx);
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_cluster_atoms(&snapshot);

        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        let grapheme_end = grapheme.len() as u32;
        let atom_lens = long_row
            .atom_positions(0..grapheme_end)
            .windows(2)
            .map(|pair| pair[1].0 - pair[0].0)
            .collect::<Vec<_>>();
        assert_eq!(atom_lens.iter().sum::<u32>(), grapheme_end);
        assert_eq!(atom_lens.first(), Some(&(1 + 15 * 2)));
        assert_eq!(atom_lens.iter().max(), Some(&(16 * 2)));
        let window = long_row.window(0., TextAlign::Left, 1.);
        assert_eq!(
            window.segments().first().map(|segment| segment.start),
            Some(0)
        );
        assert_eq!(
            snapshot.grapheme_at(DisplayPoint::new(DisplayRow(0), 0)),
            Some(SharedString::from(grapheme))
        );
    }

    #[gpui::test]
    async fn test_pending_long_rows_scan_next_to_fold_placeholders(cx: &mut TestAppContext) {
        let placeholder_start = 17_000;
        let text = format!(
            "{}hide{}",
            "x".repeat(placeholder_start),
            "x".repeat(20_000)
        );
        cx.background_executor.set_block_on_ticks(0..=0);
        let (_, map) = build_display_map(&text, 4, cx);
        let creases = vec![Crease::simple(
            MultiBufferOffset(placeholder_start)..MultiBufferOffset(placeholder_start + 4),
            FoldPlaceholder::test(),
        )];
        map.update(cx, |map, cx| map.fold(creases, cx));
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        assert!(!long_row.is_measured());

        let placeholder_len = "\u{22ef}".len() as u32;
        let placeholder_start = placeholder_start as u32;
        for column in (placeholder_start - 16 * 1024)..=(placeholder_start - 16 * 1024 + 2) {
            assert_eq!(long_row.floor_column(column), column);
        }
        for column in placeholder_start + 1..placeholder_start + placeholder_len {
            assert_eq!(
                long_row.atom_range(column),
                placeholder_start..placeholder_start + placeholder_len
            );
        }
        let window = long_row.window(
            long_row.x_for_column(placeholder_start - 2),
            TextAlign::Left,
            50.,
        );
        assert!(window.segments().iter().any(|segment| {
            segment.contains(&placeholder_start)
                && segment.end > placeholder_start + placeholder_len
        }));
        let style = EditorStyle::default();
        let language_aware = LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: true,
        };
        for segment in window.segments() {
            let text = snapshot
                .highlighted_chunks_in_row(DisplayRow(0), segment.clone(), language_aware, &style)
                .map(|chunk| chunk.text)
                .collect::<String>();
            assert_eq!(text.len() as u32, segment.end - segment.start);
        }
    }

    #[gpui::test]
    async fn test_pending_long_rows_measure_only_requested_windows(cx: &mut TestAppContext) {
        let text = "é".repeat(20_000);
        cx.background_executor.set_block_on_ticks(0..=0);
        let (_, map) = build_display_map(&text, 4, cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        assert!(!long_row.is_measured());

        let window_start = text.len() as u32 / 2;
        let window_len = 100 * "é".len() as u32;
        let window = RowWindow {
            segments: smallvec![window_start..window_start + window_len],
        };
        let window_x = long_row.x_for_column(window_start);
        let epoch = snapshot.long_row_epoch();
        let request = long_row
            .measure_request(&window, &[])
            .expect("the window is not measured yet");
        assert!(map.update(cx, |map, cx| map.measure_long_row_columns(&[request], cx)));

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
        assert!(!long_row.is_measured());
        assert!(long_row.measure_request(&window, &[]).is_none());
        assert_ne!(snapshot.long_row_epoch(), epoch);
        assert_eq!(long_row.x_for_column(window_start), window_x);
        let mut wrapper = cx.update(|cx| cx.text_system().line_wrapper(test_font(), FONT_SIZE));
        let window_width = 100. * f64::from(wrapper.width_for_char('é'));
        assert!(
            (long_row.x_for_column(window_start + window_len) - window_x - window_width).abs()
                < 1e-2
        );
    }

    #[gpui::test]
    async fn test_selection_goals_follow_their_origins_when_rows_move(cx: &mut TestAppContext) {
        let text = format!("{}\n{}\nab", "x".repeat(20_000), "é".repeat(20_000));
        cx.background_executor.set_block_on_ticks(0..=0);
        let (_, map) = build_display_map(&text, 4, cx);
        let origin_column = 30_000;
        let x_at_origin = |snapshot: &DisplaySnapshot| {
            snapshot
                .long_row(DisplayRow(1))
                .expect("a long row")
                .x_for_column(origin_column)
        };
        let cursor = |id, point, goal| Selection {
            id,
            start: point,
            end: point,
            reversed: false,
            goal,
        };
        let mut selections = SelectionsCollection::new();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        selections.change_with(&snapshot, |selections| {
            selections.select(vec![
                cursor(1, Point::new(1, origin_column), SelectionGoal::None),
                cursor(2, Point::new(2, 0), SelectionGoal::None),
                cursor(3, Point::new(0, 100), SelectionGoal::None),
            ])
        });
        let estimated_x = x_at_origin(&snapshot);
        let untracked_goal = SelectionGoal::HorizontalPosition(12_345.);
        let end_of_line_goal = SelectionGoal::HorizontalPosition(f64::INFINITY);
        selections.change_with(&snapshot, |selections| {
            selections.select(vec![
                cursor(
                    1,
                    Point::new(2, 1),
                    SelectionGoal::HorizontalPosition(estimated_x),
                ),
                cursor(2, Point::new(0, 5), end_of_line_goal),
                cursor(3, Point::new(0, 200), untracked_goal),
            ])
        });

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let window = RowWindow {
            segments: smallvec![10_000..10_200],
        };
        let request = snapshot
            .long_row(DisplayRow(1))
            .expect("a long row")
            .measure_request(&window, &[])
            .expect("an unmeasured window");
        assert!(map.update(cx, |map, cx| map.measure_long_row_columns(&[request], cx)));
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let measured_x = x_at_origin(&snapshot);
        assert!(
            (measured_x - estimated_x).abs() > 1.,
            "{measured_x} {estimated_x}"
        );

        selections.change_with(&snapshot, |_| {});
        let mut goals = selections
            .disjoint_anchors()
            .iter()
            .map(|selection| (selection.id, selection.goal))
            .collect::<Vec<_>>();
        goals.sort_by_key(|(id, _)| *id);
        assert_eq!(
            goals,
            [
                (1, SelectionGoal::HorizontalPosition(measured_x)),
                (2, end_of_line_goal),
                (3, untracked_goal),
            ]
        );
    }

    #[gpui::test]
    async fn test_selection_goals_stay_put_when_origin_rows_change(cx: &mut TestAppContext) {
        let text = ["x".repeat(4_096), "x".repeat(4_096), "x".repeat(4_096)].join("\n");
        let (buffer, map) = build_display_map(&text, 4, cx);
        cx.run_until_parked();
        let cursor = |point, goal| Selection {
            id: 1,
            start: point,
            end: point,
            reversed: false,
            goal,
        };
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let goal_x = snapshot
            .long_row(DisplayRow(0))
            .expect("a long row")
            .x_for_column(100);
        let goal = SelectionGoal::HorizontalPosition(goal_x);
        let mut selections = SelectionsCollection::new();
        selections.change_with(&snapshot, |selections| {
            selections.select(vec![cursor(Point::new(0, 100), SelectionGoal::None)])
        });
        selections.change_with(&snapshot, |selections| {
            selections.select(vec![cursor(Point::new(1, 100), goal)])
        });

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(MultiBufferOffset(50)..MultiBufferOffset(50), "\n")],
                None,
                cx,
            )
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let untouched = snapshot.long_row(DisplayRow(3)).expect("a long row");
        let base = untouched.widths.base_font;
        let split_runs = [RowRuns::new(
            0,
            vec![
                FontRun {
                    len: 10,
                    font_id: base,
                },
                FontRun {
                    len: untouched.len() as usize - 10,
                    font_id: base,
                },
            ],
        )];
        let request = untouched
            .measure_request(
                &RowWindow {
                    segments: smallvec![0..untouched.len()],
                },
                &split_runs,
            )
            .expect("a restyle request");
        assert!(map.update(cx, |map, cx| map.measure_long_row_columns(&[request], cx)));
        let restyled = map.update(cx, |map, cx| map.snapshot(cx));
        assert_ne!(restyled.long_row_epoch(), snapshot.long_row_epoch());

        selections.change_with(&restyled, |_| {});
        assert_eq!(
            selections
                .disjoint_anchors()
                .iter()
                .map(|selection| selection.goal)
                .collect::<Vec<_>>(),
            [goal]
        );
    }

    #[gpui::test]
    async fn test_line_splits_and_joins_keep_measured_geometry(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map(&"é".repeat(10_000), 4, cx);
        cx.run_until_parked();
        let measured = map
            .update(cx, |map, cx| map.snapshot(cx))
            .long_row(DisplayRow(0))
            .expect("a long row");
        assert!(measured.is_measured());
        cx.background_executor.set_block_on_ticks(0..=0);
        let edit = |range: Range<usize>, new_text: &str, cx: &mut TestAppContext| {
            buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(
                        MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                        new_text,
                    )],
                    None,
                    cx,
                )
            });
            map.update(cx, |map, cx| map.snapshot(cx))
        };
        let assert_shifted = |long_row: &LongRow, columns: &[u32], shift: u32| {
            assert!(long_row.is_measured());
            let shift_x = measured.x_for_column(shift);
            for column in columns {
                let expected = measured.x_for_column(column + shift) - shift_x;
                let actual = long_row.x_for_column(*column);
                assert!(
                    (actual - expected).abs() <= POSITION_TOLERANCE,
                    "column {column}: {actual} != {expected}"
                );
            }
        };
        let split = 6_000;

        let snapshot = edit(split..split, "\n", cx);
        let rows = [0, 1].map(|row| snapshot.long_row(DisplayRow(row)).expect("a long row"));
        assert_shifted(&rows[0], &[2_000, 5_998], 0);
        assert_shifted(&rows[1], &[2_000, 13_998], split as u32);

        let snapshot = edit(split..split + 1, "", cx);
        let joined = snapshot.long_row(DisplayRow(0)).expect("a long row");
        assert_shifted(&joined, &[2_000, 15_000, 19_998], 0);
    }

    #[gpui::test]
    async fn test_inlay_only_windows_keep_buffer_ranges(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map("ab", 4, cx);
        let position = buffer.read_with(cx, |buffer, cx| {
            buffer.snapshot(cx).anchor_after(MultiBufferOffset(1))
        });
        let hint = Inlay::mock_hint(0, position, "h".repeat(4_096));
        map.update(cx, |map, cx| map.splice_inlays(&[], vec![hint], cx));
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let row = DisplayRow(0);
        let long_row = snapshot.long_row(row).expect("a long row");
        let window = RowWindow {
            segments: smallvec![2_000..2_100],
        };
        assert_eq!(
            snapshot.visible_buffer_ranges(
                Point::new(0, 0)..Point::new(0, 2),
                &[(row, long_row, window)]
            ),
            [Point::new(0, 1)..Point::new(0, 1)]
        );
    }

    #[test]
    fn test_densest_segments_skip_zero_width_pieces() {
        let pieces = [
            Piece {
                columns: 0..600,
                density: 0.,
            },
            Piece {
                columns: 600..1_200,
                density: 0.,
            },
            Piece {
                columns: 1_200..1_207,
                density: 8.,
            },
            Piece {
                columns: 1_207..1_400,
                density: 0.5,
            },
        ];
        assert_eq!(densest_segments(&pieces, 100).as_slice(), [1_200..1_207]);
        assert_eq!(densest_segments(&pieces, 1_000).as_slice(), [1_200..1_400]);

        let oversized = [
            Piece {
                columns: 0..2_000,
                density: 0.001,
            },
            Piece {
                columns: 2_000..2_010,
                density: 8.,
            },
        ];
        assert_eq!(densest_segments(&oversized, 100).as_slice(), [0..2_010]);
    }

    #[gpui::test]
    async fn test_long_rows_keep_flag_pairs_after_capped_clusters(cx: &mut TestAppContext) {
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
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            let long_row = snapshot.long_row(DisplayRow(0)).expect("a long row");
            let flags_start = 1_100 + prepend_count as u32 * "\u{d4e}".len() as u32;
            let atom_starts = long_row
                .atom_positions(flags_start..flags_start + flags.len() as u32)
                .into_iter()
                .map(|(start, _)| start - flags_start)
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
        assert_matches_char_widths(&map.update(cx, |map, cx| map.snapshot(cx)), &mut wrapper);

        set_tab_size(8, cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_round_trips(&snapshot);
        cx.run_until_parked();
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(snapshot.long_row(DisplayRow(50)).is_some());
        assert_matches_char_widths(&snapshot, &mut wrapper);
    }

    #[gpui::test]
    async fn test_folded_buffers_do_not_widen_long_rows(cx: &mut TestAppContext) {
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
        assert!(
            map.update(cx, |map, cx| map.snapshot(cx))
                .max_long_row_width()
                > 0.
        );

        let long_buffer_id = buffer.read_with(cx, |buffer, cx| {
            buffer
                .snapshot(cx)
                .excerpts()
                .next()
                .map(|excerpt| excerpt.context.start.buffer_id)
                .expect("an excerpt")
        });
        map.update(cx, |map, cx| map.fold_buffers([long_buffer_id], cx));
        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx))
                .max_long_row_width(),
            0.
        );
    }

    #[gpui::test]
    async fn test_long_rows_track_trailing_whitespace(cx: &mut TestAppContext) {
        let (buffer, map) = build_display_map(&format!("x{}", " ".repeat(10_000)), 4, cx);
        let trailing_whitespace_start = |map: &Entity<DisplayMap>, cx: &mut TestAppContext| {
            cx.run_until_parked();
            map.update(cx, |map, cx| map.snapshot(cx))
                .long_row(DisplayRow(0))
                .and_then(|long_row| long_row.trailing_whitespace_start())
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
                map.update(cx, |map, cx| map.fold(creases, cx));
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
            let Some(long_row) = snapshot.long_row(DisplayRow(row)) else {
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
            let Some(long_row) = snapshot.long_row(DisplayRow(row)) else {
                continue;
            };

            let line = snapshot.line(DisplayRow(row));
            let expected_atoms = line
                .grapheme_indices(true)
                .flat_map(|(start, grapheme)| capped_atoms(start, grapheme))
                .collect::<Vec<_>>();
            let atom_starts = long_row
                .atom_positions(0..long_row.len())
                .into_iter()
                .map(|(start, _)| start as usize)
                .filter(|start| *start < line.len())
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

    fn capped_atoms(start: usize, grapheme: &str) -> Vec<Range<usize>> {
        let mut atoms = Vec::new();
        let mut atom_start = start;
        for (offset, character) in grapheme.char_indices() {
            let end = start + offset + character.len_utf8();
            if end - atom_start > MAX_ATOM_LEN as usize && !is_regional_indicator(character) {
                atoms.push(atom_start..start + offset);
                atom_start = start + offset;
            }
        }
        atoms.push(atom_start..start + grapheme.len());
        atoms
    }

    fn assert_matches_fresh_geometry(snapshot: &DisplaySnapshot) {
        for row in 0..=snapshot.max_point().row().0 {
            let Some(long_row) = snapshot.long_row(DisplayRow(row)) else {
                continue;
            };
            assert!(long_row.geometry.is_settled(), "row {row}");
            let mut wrapper = long_row.widths.wrapper.lock();
            let fresh = Measurer {
                snapshot: &long_row.snapshot,
                wrapper: &mut wrapper,
                estimate: long_row.widths.estimate,
                budget: u32::MAX,
            }
            .measure_prefix(long_row.row, long_row.len(), 0);
            let ends = |geometry: &RowGeometry| {
                geometry
                    .chunks
                    .iter()
                    .map(|chunk| chunk.end)
                    .collect::<Vec<_>>()
            };
            assert_eq!(ends(&long_row.geometry), ends(&fresh), "row {row}");
            for (stored, fresh) in long_row.geometry.chunks.iter().zip(&fresh.chunks) {
                assert!(
                    (stored.x_end - fresh.x_end).abs() < 1e-6,
                    "row {row} chunk ending at {}: {} != {}",
                    stored.end,
                    stored.x_end,
                    fresh.x_end
                );
            }
        }
    }

    fn assert_matches_char_widths(snapshot: &DisplaySnapshot, wrapper: &mut LineWrapper) {
        for row in 0..=snapshot.max_point().row().0 {
            let line = snapshot.line(DisplayRow(row));
            let Some(long_row) = snapshot.long_row(DisplayRow(row)) else {
                assert!(
                    line.len() <= MAX_LINE_LEN,
                    "row {row} has no long row geometry"
                );
                continue;
            };
            let standalone_invisibles = StandaloneInvisibles::default().find(&line, None);
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
