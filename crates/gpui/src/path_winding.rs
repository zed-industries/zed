//! Path fills by per-pixel winding.
//!
//! Each pixel casts a ray leftward and a ray upward and sums the curves they
//! cross, as in Eric Lengyel's Slug: a crossing more than half a pixel away
//! counts a whole winding step, a nearer one a fraction. The two sums are
//! blended and mapped through the fill rule.
//!
//! To keep the rays short, painting a [`Path`] bins its curves into tiles. A
//! tile lists the curves that come near it, which the GPU evaluates. Every
//! other curve can only contribute whole steps, which the binner sums into
//! the tile's backdrop: the winding of the unlisted curves along a leftward
//! ray from the tile's reference corner. The shader adjusts the backdrop for
//! its own pixel using the listed curves' endpoints. Contours are closed, so
//! wherever an unlisted curve ends, a curve starts at the same point, and only
//! those that are listed need correcting.
//!
//! Binning works in logical pixels. The shader converts each pixel's center
//! into logical pixels, so it compares exactly the coordinates the binner
//! compared. Only antialiasing distances, and how many pixels each tile
//! shades, depend on the display scale.
//!
//! Tiles come from a grid of square cells, one size per path, chosen to
//! balance the CPU cost of listing curves in more cells against the GPU cost
//! of evaluating them over larger ones. Each cell's quad is trimmed to the
//! pixels its curves can affect. Neighboring cells that list the same curves
//! with the same backdrop would be shaded identically, so they share a tile:
//! a rectangle's edges become strips and its interior one tile.

use crate::{Bounds, FillRule, PathCurve, PathTile, Pixels, Point, point, px};
use lyon_path::geom::{CubicBezierSegment, QuadraticBezierSegment};
use std::{cell::RefCell, sync::Arc};

/// The smallest cell side length, in logical pixels.
const MIN_TILE_SIZE: f32 = 8.0;

/// The grid has at most this many cells, since every cell is counted and
/// walked. Larger paths get larger cells.
const MAX_GRID_CELLS: usize = 4096;

/// How many GPU curve evaluations, one per device pixel, cost as much as
/// listing a curve in one more cell on the CPU. This deliberately favors the
/// CPU. Measured on a desktop CPU and GPU, a listing costs about 640
/// evaluations (10.8 ns against 17 ps), plus about 190 for each cell walked.
/// At those costs, finer cells would save GPU time on dense strokes, but cost
/// Zed's small fills up to twice the CPU time. Tuned at a scale of 1.5.
const VISITS_PER_LISTING: f32 = 1125.0;

/// How many GPU curve evaluations cost as much as walking one more empty
/// cell. Measured at about 190, and weighted like `VISITS_PER_LISTING`.
/// Without it, large paths with few curves get fine cells that are almost all
/// empty. Listed cells are left out: charging them made dashed lines coarser,
/// which cost far more GPU time than it saved.
const VISITS_PER_CELL: f32 = 330.0;

/// A tile lists every curve within this distance of it, in logical pixels.
/// The curves it leaves out are then more than half a device pixel from any
/// pixel it shades at scales of 1 and above, so they contribute only whole
/// steps. The extra 1/256 covers the rasterizer snapping tile edges to its
/// subpixel grid.
const MARGIN: f32 = 0.5 + 1.0 / 256.0;

/// Whole numbers are exact in f32 below this, so cell edges that are
/// computed in different ways still agree.
const MAX_COORDINATE: f32 = 16_777_216.0;

/// Curve indices are stored as `u32`; this leaves room for sentinels.
const MAX_CURVES: usize = (1 << 25) - 2;

/// `f32::min`, `max` and `clamp` without their NaN handling, which costs a
/// compare and blend per call on x86-64's baseline target. Binning calls
/// these per curve and per cell; non-finite coordinates may bin arbitrarily.
trait FastFloat {
    fn fast_min(self, other: f32) -> f32;
    fn fast_max(self, other: f32) -> f32;
    fn fast_clamp(self, min: f32, max: f32) -> f32;
}

impl FastFloat for f32 {
    #[inline(always)]
    fn fast_min(self, other: f32) -> f32 {
        if self < other { self } else { other }
    }

    #[inline(always)]
    fn fast_max(self, other: f32) -> f32 {
        if self > other { self } else { other }
    }

    #[inline(always)]
    fn fast_clamp(self, min: f32, max: f32) -> f32 {
        self.fast_max(min).fast_min(max)
    }
}

/// A filled path, ready to paint. Build one with
/// [`PathBuilder`](crate::PathBuilder).
#[derive(Clone, Debug)]
pub struct Path(Arc<PathData>);

#[derive(Debug)]
struct PathData {
    even_odd: bool,
    /// Closed contours of monotone curves, or none if the path can't be
    /// binned.
    curves: Vec<PathCurve>,
    /// The corners of the curves' bounds, in logical pixels.
    min: Point<f32>,
    max: Point<f32>,
    /// The sum of the curves' widths and heights, which estimates how many
    /// cells they cross.
    perimeter: f32,
}

impl Path {
    pub(crate) fn from_curves(curves: Vec<PathCurve>, fill_rule: FillRule) -> Self {
        let even_odd = matches!(fill_rule, FillRule::EvenOdd);
        let empty = || {
            Self(Arc::new(PathData {
                even_odd,
                curves: Vec::new(),
                min: Point::default(),
                max: Point::default(),
                perimeter: 0.0,
            }))
        };
        if curves.len() > MAX_CURVES {
            log::warn!("path has {} curves; skipping", curves.len());
            return empty();
        }

        // Monotone curves keep their control points inside their endpoint box,
        // so the endpoints alone give the bounds.
        let mut min = point(f32::INFINITY, f32::INFINITY);
        let mut max = point(f32::NEG_INFINITY, f32::NEG_INFINITY);
        let mut perimeter = 0.0;
        for curve in &curves {
            min.x = min.x.fast_min(curve.min_x());
            max.x = max.x.fast_max(curve.max_x());
            min.y = min.y.fast_min(curve.p0.y);
            max.y = max.y.fast_max(curve.p1.y);
            perimeter += curve.max_x() - curve.min_x() + curve.p1.y - curve.p0.y;
        }
        if !(min.x.is_finite() && min.y.is_finite() && max.x.is_finite() && max.y.is_finite()) {
            return empty();
        }
        Self(Arc::new(PathData {
            even_odd,
            curves,
            min,
            max,
            perimeter,
        }))
    }

    pub(crate) fn bounds(&self) -> Bounds<Pixels> {
        let PathData { min, max, .. } = *self.0;
        Bounds::from_corners(point(px(min.x), px(min.y)), point(px(max.x), px(max.y)))
    }

    pub(crate) fn even_odd(&self) -> bool {
        self.0.even_odd
    }

    /// Bin the curves into tiles for painting at the given display scale,
    /// which only affects how large the cells are.
    pub(crate) fn bin(&self, scale: f32) -> PathTiles {
        bin(&self.0, scale)
    }
}

/// A path's curves binned into tiles for one display scale.
#[derive(Debug, Default)]
pub(crate) struct PathTiles {
    tiles: Vec<PathTile>,
    /// Every tile's curve list, concatenated.
    curves: Vec<PathCurve>,
}

impl PathTiles {
    /// Tiles in logical pixels, with curve starts indexing `curves`.
    pub(crate) fn tiles(&self) -> &[PathTile] {
        &self.tiles
    }

    pub(crate) fn curves(&self) -> &[PathCurve] {
        &self.curves
    }
}

/// A path's contours as monotone curves, collected by
/// [`PathBuilder`](crate::PathBuilder).
#[derive(Default)]
pub(crate) struct Outline {
    curves: Vec<PathCurve>,
    start: Point<f32>,
    current: Point<f32>,
}

impl Outline {
    /// Close the current contour and start a new one at the given point.
    pub(crate) fn move_to(&mut self, to: Point<f32>) {
        self.close();
        self.start = to;
        self.current = to;
    }

    pub(crate) fn line_to(&mut self, to: Point<f32>) {
        if to != self.current {
            self.curves.extend(PathCurve::line(self.current, to));
            self.current = to;
        }
    }

    pub(crate) fn curve_to(&mut self, to: Point<f32>, ctrl: Point<f32>) {
        if to == self.current && ctrl == self.current {
            return;
        }
        PathCurve::push_monotone(&mut self.curves, self.current, ctrl, to);
        self.current = to;
    }

    pub(crate) fn cubic_to(&mut self, to: Point<f32>, ctrl1: Point<f32>, ctrl2: Point<f32>) {
        /// Maximum error, in logical pixels.
        const TOLERANCE: f32 = 0.25;
        let lyon_point = |p: Point<f32>| lyon_path::math::point(p.x, p.y);
        let cubic = CubicBezierSegment {
            from: lyon_point(self.current),
            ctrl1: lyon_point(ctrl1),
            ctrl2: lyon_point(ctrl2),
            to: lyon_point(to),
        };
        cubic.for_each_quadratic_bezier(TOLERANCE, &mut |quadratic| {
            self.curve_to(
                point(quadratic.to.x, quadratic.to.y),
                point(quadratic.ctrl.x, quadratic.ctrl.y),
            );
        });
    }

    /// Close the current contour with a straight line back to its start.
    pub(crate) fn close(&mut self) {
        let start = self.start;
        self.line_to(start);
    }

    /// The curves, with every contour closed.
    pub(crate) fn into_curves(mut self) -> Vec<PathCurve> {
        self.close();
        self.curves
    }
}

fn bin(path: &PathData, scale: f32) -> PathTiles {
    let mut binned = PathTiles::default();
    let PathData {
        even_odd,
        curves: ref input,
        min,
        max,
        perimeter,
    } = *path;
    if input.is_empty() {
        return binned;
    }

    // Cell edges are whole numbers, so that edges computed in different ways
    // are equal. Cells cover every pixel within reach of a curve.
    let left = (min.x - MARGIN).floor();
    let top = (min.y - MARGIN).floor();
    let size = point(max.x + MARGIN - left, max.y + MARGIN - top);
    let cell_size = cell_size(input.len() as f32, perimeter, size, scale);
    let columns = (size.x / cell_size) as usize + 1;
    let rows = (size.y / cell_size) as usize + 1;
    if left.abs().fast_max(top.abs()) + cell_size * columns.max(rows) as f32 > MAX_COORDINATE {
        log::warn!("path at ({left}, {top}) spans {size:?} logical pixels; skipping");
        return binned;
    }
    let grid = Grid {
        left,
        top,
        cell_size,
        cells_per_pixel: 1.0 / cell_size,
        columns,
        rows,
    };

    SCRATCH.with_borrow_mut(|scratch| {
        let Scratch {
            spans,
            ends,
            steps,
            slots,
            above,
            current,
        } = scratch;
        list_curves(input, &grid, spans, ends, steps);
        sort_listings(spans, ends, slots);
        above.clear();
        current.clear();
        let mut walk = Walk {
            input,
            even_odd,
            grid: &grid,
            slots,
            above,
            current,
            row: 0,
            above_cursor: 0,
            open: None,
            tiles: Vec::new(),
            // Each listing is copied at most once.
            curves: Vec::with_capacity(slots.len()),
        };
        walk.rows(ends, steps);
        binned.tiles = walk.tiles;
        binned.curves = walk.curves;
    });
    binned
}

/// The power-of-two cell size that minimizes the estimated cost of binning
/// and shading. A curve is listed in about `1 + extent / s` cells, or once
/// when the path fits in one. The GPU visits each listing over about half
/// its cell, less where the path is thinner than a cell, at every device
/// pixel it covers. Every cell is walked, and at least `cells - listings` of
/// them are empty.
fn cell_size(curves: f32, perimeter: f32, size: Point<f32>, scale: f32) -> f32 {
    let extent = size.x.fast_max(size.y);
    let mut cell_size = MIN_TILE_SIZE;
    // Every cell is counted and walked, so cells are kept few.
    while ((size.x / cell_size) as usize + 1) * ((size.y / cell_size) as usize + 1) > MAX_GRID_CELLS
    {
        cell_size *= 2.0;
    }
    let mut best = (f32::INFINITY, cell_size);
    loop {
        let listings = if cell_size >= extent {
            curves
        } else {
            curves + perimeter / cell_size
        };
        let area = cell_size.fast_min(size.x) * cell_size.fast_min(size.y) / 2.0;
        let cells = ((size.x / cell_size) as usize + 1) * ((size.y / cell_size) as usize + 1);
        let empty_cells = (cells as f32 - listings).fast_max(0.0);
        let cost =
            listings * (VISITS_PER_LISTING + area * scale * scale) + empty_cells * VISITS_PER_CELL;
        if cost < best.0 {
            best = (cost, cell_size);
        }
        if cell_size >= extent {
            return best.1;
        }
        cell_size *= 2.0;
    }
}

fn fills(winding: i32, even_odd: bool) -> bool {
    if even_odd {
        winding % 2 != 0
    } else {
        winding != 0
    }
}

#[derive(Default)]
struct Scratch {
    spans: Vec<Span>,
    /// Per cell, the end of its listings in `slots`.
    ends: Vec<u32>,
    /// Per cell, the winding steps of curves crossing its row's top to its
    /// left, other than those it lists.
    steps: Vec<i32>,
    /// Each cell's listings: a curve's leftmost x above its index.
    slots: Vec<u64>,
    above: Vec<RowTile>,
    current: Vec<RowTile>,
}

thread_local! {
    static SCRATCH: RefCell<Scratch> = Default::default();
}

/// A curve's run of consecutive cells within a row.
struct Span {
    cell: usize,
    count: usize,
    slot: u64,
}

/// Finds the cells each curve comes within `MARGIN` of, as a span per row,
/// counting listings per cell in `ends`. In each row whose top a curve
/// crosses, it steps the backdrop of every cell right of its span, recorded
/// once at the first of them.
fn list_curves(
    input: &[PathCurve],
    grid: &Grid,
    spans: &mut Vec<Span>,
    ends: &mut Vec<u32>,
    steps: &mut Vec<i32>,
) {
    let cells = grid.rows * grid.columns;
    spans.clear();
    ends.clear();
    ends.resize(cells, 0);
    steps.clear();
    steps.resize(cells, 0);
    for (index, curve) in input.iter().enumerate() {
        // Slots order a cell's curves by leftmost x, for the shader's early
        // exit, then index, so that equal lists compare equal.
        let slot = u64::from(ordered_bits(curve.min_x())) << 32 | index as u64;
        for row in grid.row_of(curve.p0.y - MARGIN)..=grid.row_of(curve.p1.y + MARGIN) {
            let row_top = grid.y(row);
            let (low, high) = curve.x_range(row_top - MARGIN, row_top + grid.cell_size + MARGIN);
            let first = grid.column_of(low - MARGIN);
            let last = grid.column_of(high + MARGIN);
            let cell = row * grid.columns + first;
            let count = last + 1 - first;
            spans.push(Span { cell, count, slot });
            for listings in ends.get_mut(cell..cell + count).unwrap_or_default() {
                *listings += 1;
            }
            // Must match the shader's span test.
            if curve.p0.y <= row_top
                && row_top < curve.p1.y
                && last + 1 < grid.columns
                && let Some(step) = steps.get_mut(cell + count)
            {
                *step += curve.step();
            }
        }
    }
}

/// Places each cell's listings together by counting sort, then sorts each
/// cell's few listings.
fn sort_listings(spans: &[Span], ends: &mut [u32], slots: &mut Vec<u64>) {
    // Each cell's count becomes its start, and then its end as it fills.
    let mut total = 0;
    for end in ends.iter_mut() {
        let count = *end;
        *end = total;
        total += count;
    }
    slots.clear();
    slots.resize(total as usize, 0);
    for span in spans {
        for cursor in ends
            .get_mut(span.cell..span.cell + span.count)
            .unwrap_or_default()
        {
            if let Some(slot) = slots.get_mut(*cursor as usize) {
                *slot = span.slot;
            }
            *cursor += 1;
        }
    }
    let mut start = 0;
    for &end in ends.iter() {
        insertion_sort(slots.get_mut(start..end as usize).unwrap_or_default());
        start = end as usize;
    }
}

/// Cells list few curves, so this beats the library sort's dispatch.
fn insertion_sort(slots: &mut [u64]) {
    if slots.len() > 16 {
        slots.sort_unstable();
        return;
    }
    for i in 1..slots.len() {
        let mut j = i;
        while j > 0 && slots[j - 1] > slots[j] {
            slots.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Maps finite floats to integers in the same order.
fn ordered_bits(x: f32) -> u32 {
    let bits = x.to_bits();
    if bits >> 31 == 1 {
        !bits
    } else {
        bits | 1 << 31
    }
}

/// A tile in the previous row, so that the current row can extend it.
#[derive(Clone, Copy)]
struct RowTile {
    tile: usize,
    start: usize,
    end: usize,
    list: (usize, usize),
}

/// A tile being extended along a row, over columns `start..end`.
struct Run {
    start: usize,
    end: usize,
    backdrop: i32,
    /// A range of the slots; empty for an interior run.
    list: (usize, usize),
    reference: Point<f32>,
    top_left: Point<f32>,
    bottom_right: Point<f32>,
}

/// Walks the cells row by row, keeping each cell's backdrop as a running
/// sum of steps. A tile extends the one before it in its row, or the one
/// above it spanning the same columns, when both have the same backdrop and
/// list the same curves: they would be shaded identically, and together
/// still cover a rectangle of cells. A curveless cell is the case of an
/// empty list, so a run of them inside the fill becomes one interior tile.
struct Walk<'a> {
    input: &'a [PathCurve],
    even_odd: bool,
    grid: &'a Grid,
    slots: &'a [u64],
    above: &'a mut Vec<RowTile>,
    current: &'a mut Vec<RowTile>,
    row: usize,
    above_cursor: usize,
    open: Option<Run>,
    tiles: Vec<PathTile>,
    curves: Vec<PathCurve>,
}

impl Walk<'_> {
    fn rows(&mut self, ends: &[u32], steps: &[i32]) {
        let mut start = 0;
        for row in 0..self.grid.rows {
            self.row = row;
            self.above_cursor = 0;
            let mut winding = 0;
            for column in 0..self.grid.columns {
                let cell = row * self.grid.columns + column;
                winding += steps.get(cell).copied().unwrap_or(0);
                let end = ends.get(cell).map_or(start, |&end| end as usize);
                if end > start {
                    self.boundary(column, winding, (start, end));
                } else if fills(winding, self.even_odd) {
                    self.interior(column, winding);
                }
                start = end;
            }
            if let Some(open) = self.open.take() {
                self.close(open);
            }
            std::mem::swap(self.above, self.current);
            self.current.clear();
        }
    }

    fn interior(&mut self, column: usize, winding: i32) {
        let cell = self.grid.cell(column, self.row);
        self.push(Run {
            start: column,
            end: column + 1,
            backdrop: winding,
            list: (0, 0),
            reference: cell.top_left(),
            top_left: cell.top_left(),
            bottom_right: cell.bottom_right(),
        });
    }

    fn boundary(&mut self, column: usize, backdrop: i32, list: (usize, usize)) {
        let cell = self.grid.cell(column, self.row);
        let mut measure = Measure::new(cell, backdrop);
        for &slot in self.slots.get(list.0..list.1).unwrap_or_default() {
            if let Some(curve) = self.input.get(slot as u32 as usize) {
                measure.add(curve);
            }
        }
        if let Some((top_left, bottom_right)) = measure.quad(self.even_odd) {
            self.push(Run {
                start: column,
                end: column + 1,
                backdrop,
                list,
                reference: cell.top_left(),
                top_left,
                bottom_right,
            });
        }
    }

    #[inline]
    fn push(&mut self, run: Run) {
        if let Some(open) = &mut self.open
            && open.end == run.start
            && open.backdrop == run.backdrop
            && same_list(self.slots, open.list, run.list)
        {
            open.end = run.end;
            open.top_left.y = open.top_left.y.fast_min(run.top_left.y);
            open.bottom_right.x = run.bottom_right.x;
            open.bottom_right.y = open.bottom_right.y.fast_max(run.bottom_right.y);
            return;
        }
        if let Some(open) = self.open.replace(run) {
            self.close(open);
        }
    }

    fn close(&mut self, run: Run) {
        // Both rows' tiles are in column order.
        while self
            .above
            .get(self.above_cursor)
            .is_some_and(|above| above.start < run.start)
        {
            self.above_cursor += 1;
        }
        if let Some(&above) = self.above.get(self.above_cursor)
            && above.start == run.start
            && above.end == run.end
            && same_list(self.slots, above.list, run.list)
            && let Some(tile) = self.tiles.get_mut(above.tile)
            && tile.backdrop == run.backdrop
        {
            tile.top_left.x = tile.top_left.x.fast_min(run.top_left.x);
            tile.bottom_right.x = tile.bottom_right.x.fast_max(run.bottom_right.x);
            tile.bottom_right.y = run.bottom_right.y;
            self.current.push(above);
            return;
        }
        let curve_start = self.curves.len();
        for &slot in self.slots.get(run.list.0..run.list.1).unwrap_or_default() {
            if let Some(&curve) = self.input.get(slot as u32 as usize) {
                self.curves.push(curve);
            }
        }
        self.current.push(RowTile {
            tile: self.tiles.len(),
            start: run.start,
            end: run.end,
            list: run.list,
        });
        self.tiles.push(PathTile {
            order: 0,
            paint: 0,
            curve_start: curve_start as u32,
            curve_count: (self.curves.len() - curve_start) as u32,
            top_left: run.top_left,
            bottom_right: run.bottom_right,
            reference: run.reference,
            backdrop: run.backdrop,
        });
    }
}

fn same_list(slots: &[u64], a: (usize, usize), b: (usize, usize)) -> bool {
    a.1 - a.0 == b.1 - b.0 && slots.get(a.0..a.1) == slots.get(b.0..b.1)
}

struct Grid {
    left: f32,
    top: f32,
    cell_size: f32,
    /// `1 / cell_size`, exact since cell sizes are powers of two.
    cells_per_pixel: f32,
    columns: usize,
    rows: usize,
}

// Clamping before converting keeps values within `u32`, whose conversion is
// much cheaper than the saturating 64-bit one.
impl Grid {
    fn column_of(&self, x: f32) -> usize {
        let column = (x - self.left) * self.cells_per_pixel;
        column.fast_clamp(0.0, (self.columns - 1) as f32) as u32 as usize
    }

    fn row_of(&self, y: f32) -> usize {
        let row = (y - self.top) * self.cells_per_pixel;
        row.fast_clamp(0.0, (self.rows - 1) as f32) as u32 as usize
    }

    fn y(&self, row: usize) -> f32 {
        self.top + row as f32 * self.cell_size
    }

    fn cell(&self, column: usize, row: usize) -> Cell {
        Cell {
            left: self.left + column as f32 * self.cell_size,
            top: self.y(row),
            size: self.cell_size,
        }
    }
}

/// A square grid cell, with edges on whole logical pixels.
#[derive(Clone, Copy)]
struct Cell {
    left: f32,
    top: f32,
    size: f32,
}

impl Cell {
    fn right(self) -> f32 {
        self.left + self.size
    }

    fn bottom(self) -> f32 {
        self.top + self.size
    }

    fn top_left(self) -> Point<f32> {
        point(self.left, self.top)
    }

    fn bottom_right(self) -> Point<f32> {
        point(self.right(), self.bottom())
    }
}

/// Accumulates the bounds and windings of a cell's curves, to trim its quad.
struct Measure {
    cell: Cell,
    backdrop: i32,
    min: Point<f32>,
    max: Point<f32>,
    /// Left of and above every curve, no correction applies, so the winding
    /// is the backdrop. Right of every curve, it is the backdrop plus the
    /// curves crossing the cell's top; below every curve, the backdrop
    /// corrected for the endpoints beside the cell's left edge.
    right_winding: i32,
    bottom_winding: i32,
}

impl Measure {
    fn new(cell: Cell, backdrop: i32) -> Self {
        Self {
            cell,
            backdrop,
            min: point(f32::INFINITY, f32::INFINITY),
            max: point(f32::NEG_INFINITY, f32::NEG_INFINITY),
            right_winding: backdrop,
            bottom_winding: backdrop,
        }
    }

    #[inline(always)]
    fn add(&mut self, curve: &PathCurve) {
        let cell = self.cell;
        self.min.x = self.min.x.fast_min(curve.min_x());
        self.max.x = self.max.x.fast_max(curve.max_x());
        self.min.y = self.min.y.fast_min(curve.p0.y);
        self.max.y = self.max.y.fast_max(curve.p1.y);
        if curve.p0.y <= cell.top && cell.top < curve.p1.y {
            self.right_winding += curve.step();
        }
        for (endpoint, step) in curve.endpoints() {
            if endpoint.x <= cell.left && endpoint.y > cell.top {
                self.bottom_winding -= step;
            }
        }
    }

    /// The top-left and bottom-right corners of the quad to shade, if any:
    /// the cell, trimmed to within `MARGIN` of the curves on each side where
    /// the winding beyond them leaves pixels unfilled.
    #[inline(always)]
    fn quad(&self, even_odd: bool) -> Option<(Point<f32>, Point<f32>)> {
        let cell = self.cell;
        let clamp_x = |x: f32| x.fast_clamp(cell.left, cell.right());
        let clamp_y = |y: f32| y.fast_clamp(cell.top, cell.bottom());
        let unfilled = !fills(self.backdrop, even_odd);
        let left = if unfilled {
            clamp_x(self.min.x - MARGIN)
        } else {
            cell.left
        };
        let top = if unfilled {
            clamp_y(self.min.y - MARGIN)
        } else {
            cell.top
        };
        let right = if fills(self.right_winding, even_odd) {
            cell.right()
        } else {
            clamp_x(self.max.x + MARGIN)
        };
        let bottom = if fills(self.bottom_winding, even_odd) {
            cell.bottom()
        } else {
            clamp_y(self.max.y + MARGIN)
        };
        (left < right && top < bottom).then(|| (point(left, top), point(right, bottom)))
    }
}

impl PathCurve {
    fn new(p0: Point<f32>, ctrl: Point<f32>, p1: Point<f32>, sign: f32) -> Self {
        Self {
            p0,
            p1,
            a: point(p0.x - 2.0 * ctrl.x + p1.x, p0.y - 2.0 * ctrl.y + p1.y),
            b: point(2.0 * (ctrl.x - p0.x), 2.0 * (ctrl.y - p0.y)),
            sign,
        }
    }

    /// A line, with exactly zero quadratic terms. Returns `None` for
    /// degenerate or non-finite lines.
    fn line(from: Point<f32>, to: Point<f32>) -> Option<Self> {
        let finite = |p: Point<f32>| p.x.is_finite() && p.y.is_finite();
        if from == to || !(finite(from) && finite(to)) {
            return None;
        }
        let (p0, p1, sign) = downward(from, to);
        Some(Self {
            p0,
            p1,
            a: point(0.0, 0.0),
            b: point(p1.x - p0.x, p1.y - p0.y),
            sign,
        })
    }

    /// Split a quadratic segment at its extrema into xy-monotone curves.
    /// Non-finite input is dropped.
    fn push_monotone(curves: &mut Vec<Self>, p0: Point<f32>, ctrl: Point<f32>, p1: Point<f32>) {
        let finite = |p: Point<f32>| p.x.is_finite() && p.y.is_finite();
        if !(finite(p0) && finite(ctrl) && finite(p1)) {
            return;
        }
        let curve = QuadraticBezierSegment {
            from: lyon_path::math::point(p0.x, p0.y),
            ctrl: lyon_path::math::point(ctrl.x, ctrl.y),
            to: lyon_path::math::point(p1.x, p1.y),
        };
        // Unlike `split_range`, this also clamps each piece's control point
        // back into its endpoint box when rounding pushes it out.
        curve.for_each_monotonic(&mut |monotone| {
            let from = point(monotone.from.x, monotone.from.y);
            let to = point(monotone.to.x, monotone.to.y);
            if from == to {
                return;
            }
            // Horizontal curves are kept: upward rays cross them.
            let (p0, p1, sign) = downward(from, to);
            let ctrl = point(
                monotone
                    .ctrl
                    .x
                    .fast_clamp(p0.x.fast_min(p1.x), p0.x.fast_max(p1.x)),
                monotone.ctrl.y.fast_clamp(p0.y, p1.y),
            );
            curves.push(Self::new(p0, ctrl, p1, sign));
        });
    }

    fn min_x(&self) -> f32 {
        self.p0.x.fast_min(self.p1.x)
    }

    fn max_x(&self) -> f32 {
        self.p0.x.fast_max(self.p1.x)
    }

    fn step(&self) -> i32 {
        if self.sign < 0.0 { -1 } else { 1 }
    }

    /// Each endpoint with the step the shader corrects for it.
    fn endpoints(&self) -> [(Point<f32>, i32); 2] {
        [(self.p0, self.step()), (self.p1, -self.step())]
    }

    /// The curve's x extent between two heights.
    fn x_range(&self, top: f32, bottom: f32) -> (f32, f32) {
        let top = top.fast_max(self.p0.y);
        let bottom = bottom.fast_min(self.p1.y);
        let x_top = if top <= self.p0.y {
            self.p0.x
        } else {
            self.x_at_y(top)
        };
        let x_bottom = if bottom >= self.p1.y {
            self.p1.x
        } else {
            self.x_at_y(bottom)
        };
        // Clamped so rounding can't carry the range past the curve's extent.
        (
            x_top.fast_min(x_bottom).fast_max(self.min_x()),
            x_top.fast_max(x_bottom).fast_min(self.max_x()),
        )
    }

    fn x_at_y(&self, y: f32) -> f32 {
        if self.a.y == 0.0 {
            if self.b.y == 0.0 {
                return self.p0.x;
            }
            let t = ((y - self.p0.y) / self.b.y).fast_clamp(0.0, 1.0);
            return self.b.x * t + self.p0.x;
        }
        let t = monotone_quadratic_root(self.a.y, self.b.y, self.p0.y - y, 1.0);
        (self.a.x * t + self.b.x) * t + self.p0.x
    }
}

fn downward(from: Point<f32>, to: Point<f32>) -> (Point<f32>, Point<f32>, f32) {
    if from.y > to.y {
        (to, from, -1.0)
    } else {
        (from, to, 1.0)
    }
}

/// The root in `[0, 1]` of `a t² + b t + c` for a monotone piece, where
/// `direction` is the sign of the solved component's derivative. That sign
/// selects the in-range root in the cancellation-free form
/// `-2c / (b + direction * sqrt(b² - 4ac))`, which also covers `a = 0`.
/// Mirrors the shader.
fn monotone_quadratic_root(a: f32, b: f32, c: f32, direction: f32) -> f32 {
    let discriminant = (b * b - 4.0 * a * c).fast_max(0.0);
    let denominator = b + direction * discriminant.sqrt();
    // On a monotone piece both terms share `direction`'s sign, so this is
    // zero only where the curve starts tangent to the line, at `t = 0`.
    if denominator == 0.0 {
        return 0.0;
    }
    (-2.0 * c / denominator).fast_clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContentMask, PaintedPath, PathBuilder, Scene, black, size};

    /// The common scales, and others whose products with whole and lattice
    /// coordinates round.
    const SCALES: [f32; 10] = [1.0, 1.1, 1.2, 1.25, 4.0 / 3.0, 1.5, 1.75, 2.0, 2.25, 3.0];

    /// Mirrors `PATH_SORT_SLACK` in `shaders.hlsl`. Tiles sort their curves
    /// by exact leftmost x, so the shader's allowance is never needed.
    const SORT_SLACK: f32 = 2.0 / 256.0;

    /// Every shape, under both fill rules, is painted at every scale through
    /// a scene. The shader's output, emulated from the
    /// scene's tiles, must match a winding sum over all of the path's curves
    /// at every sampled pixel, and no pixel may lie in two tiles.
    #[test]
    fn tiles_match_reference() {
        let mut checked = 0;
        for offset in [point(px(0.), px(0.)), point(px(0.3), px(-0.7))] {
            for (name, curves) in shapes(offset) {
                for fill_rule in [FillRule::NonZero, FillRule::EvenOdd] {
                    let path = Path::from_curves(curves.clone(), fill_rule);
                    for scale in SCALES {
                        checked += check(name, &curves, &path, scale, huge_mask());
                    }
                    for scale in [1.25, 2.0] {
                        checked += check(name, &curves, &path, scale, partial_mask(&path));
                    }
                }
            }
        }
        assert!(checked > 100_000, "only {checked} pixels checked");
    }

    /// Random polygons, curves and strokes at random offsets and scales.
    #[test]
    fn random_paths_match_reference() {
        let mut random = xorshift(0x2545_F491);
        for case in 0..80 {
            let mut builder = if case % 2 == 0 {
                PathBuilder::fill()
            } else {
                PathBuilder::stroke(px(0.25 + random() * 6.0))
            };
            let origin = point(random() * 600.0 - 200.0, random() * 400.0 - 150.0);
            let extent = 10.0 + random() * 200.0;
            let mut vertex = || {
                point(
                    px(origin.x + random() * extent),
                    px(origin.y + random() * extent),
                )
            };
            builder.move_to(vertex());
            for _ in 0..3 + case % 10 {
                let to = vertex();
                if case % 3 == 0 {
                    let ctrl = vertex();
                    builder.curve_to(to, ctrl);
                } else {
                    builder.line_to(to);
                }
            }
            if case % 4 != 1 {
                builder.close();
            }
            let scale = 1.0 + random() * 2.0;
            let (curves, fill_rule) = builder.outline();
            let path = Path::from_curves(curves.clone(), fill_rule);
            check("random", &curves, &path, scale, huge_mask());
        }
    }

    /// A rectangle spanning many cells is four edge strips, four corners and
    /// one interior, however many cells it covers.
    #[test]
    fn rectangle_is_nine_tiles() {
        let (curves, fill_rule) =
            polygon(&[(100., 40.), (900., 40.), (900., 600.), (100., 600.)]).outline();
        let path = Path::from_curves(curves.clone(), fill_rule);
        let binned = path.bin(1.25);
        assert_eq!(binned.tiles().len(), 9, "{:#?}", binned.tiles());
        let mask = Bounds {
            origin: point(px(0.), px(0.)),
            size: size(px(1000.), px(700.)),
        };
        check("rectangle", &curves, &path, 1.25, mask);
    }

    /// A path far larger than the window is binned coarsely, and painting it
    /// keeps only the tiles its clip shows.
    #[test]
    fn huge_path_paints_only_visible_tiles() {
        let mut builder = PathBuilder::fill();
        builder.add_polygon(
            &[
                point(px(-20000.), px(-20000.)),
                point(px(20000.), px(-20000.)),
                point(px(20000.), px(20000.)),
                point(px(-20000.), px(20000.)),
            ],
            true,
        );
        let (curves, fill_rule) = builder.outline();
        let path = Path::from_curves(curves.clone(), fill_rule);
        let tile_count = path.bin(1.5).tiles().len();
        assert!(tile_count < 10_000, "{tile_count} tiles");

        let mask = Bounds {
            origin: point(px(300.), px(200.)),
            size: size(px(120.), px(90.)),
        };
        let scene = paint(&path, 1.5, mask);
        assert!(!scene.path_tiles.is_empty());
        assert!(
            scene.path_tiles.len() < 20,
            "{} tiles",
            scene.path_tiles.len()
        );
        check("huge square", &curves, &path, 1.5, mask);
    }

    /// Paths painted into a scene, and replayed from it into another, keep
    /// their own tiles and curves.
    #[test]
    fn scene_paths_keep_their_tiles() {
        let paths: Vec<Path> = shapes(point(px(0.), px(0.)))
            .iter()
            .map(|(_, curves)| Path::from_curves(curves.clone(), FillRule::NonZero))
            .collect();
        // Each tile with its curves resolved, per paint.
        let resolve = |tiles: &[PathTile], curves: &[PathCurve], paints: usize| {
            let mut per_path = vec![Vec::new(); paints];
            for tile in tiles {
                let start = tile.curve_start as usize;
                let listed = &curves[start..start + tile.curve_count as usize];
                per_path[tile.paint as usize].push(format!(
                    "{:?} {:?} {:?} {} {listed:?}",
                    tile.top_left, tile.bottom_right, tile.reference, tile.backdrop
                ));
            }
            per_path
        };
        // Scenes order paths by draw order, so compare them as sets.
        let mut standalone: Vec<_> = paths
            .iter()
            .map(|path| {
                let binned = path.bin(1.5);
                resolve(binned.tiles(), binned.curves(), 1).remove(0)
            })
            .collect();
        standalone.sort();

        let mut scene = Scene::default();
        for path in &paths {
            scene.insert_primitive(PaintedPath::new(
                path.clone(),
                1.5,
                ContentMask {
                    bounds: huge_mask(),
                },
                black().into(),
            ));
        }
        scene.finish();
        let mut painted = resolve(&scene.path_tiles, &scene.path_curves, paths.len());
        painted.sort();
        assert_eq!(painted, standalone);

        // An unrelated path first, so every range moves.
        let mut replayed = Scene::default();
        replayed.insert_primitive(PaintedPath::new(
            paths[0].clone(),
            2.0,
            ContentMask {
                bounds: huge_mask(),
            },
            black().into(),
        ));
        replayed.replay(0..scene.paint_operations.len(), &scene);
        replayed.finish();
        let mut replayed = resolve(&replayed.path_tiles, &replayed.path_curves, paths.len() + 1);
        let first_binned = paths[0].bin(2.0);
        let first = resolve(first_binned.tiles(), first_binned.curves(), 1).remove(0);
        let first_index = replayed.iter().position(|tiles| *tiles == first);
        assert!(first_index.is_some());
        if let Some(index) = first_index {
            replayed.remove(index);
        }
        replayed.sort();
        assert_eq!(replayed, standalone);
    }

    /// The shader's backdrop corrections need curves that meet to share
    /// bit-identical endpoints. A break shows up as streaks, not as a
    /// failure, so check it directly.
    #[test]
    fn endpoints_balance() {
        for (name, curves) in shapes(point(px(0.3), px(-0.7))) {
            let mut balance = collections::HashMap::<(u32, u32), i32>::default();
            for curve in &curves {
                for (endpoint, step) in curve.endpoints() {
                    *balance
                        .entry((endpoint.x.to_bits(), endpoint.y.to_bits()))
                        .or_default() += step;
                }
            }
            assert!(!balance.is_empty(), "{name} produced no curves");
            for ((x, y), sum) in balance {
                assert_eq!(
                    sum,
                    0,
                    "{name}: endpoints at ({}, {}) do not balance",
                    f32::from_bits(x),
                    f32::from_bits(y),
                );
            }
        }
    }

    fn xorshift(mut state: u32) -> impl FnMut() -> f32 {
        move || {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state as f32 / u32::MAX as f32
        }
    }

    fn huge_mask() -> Bounds<Pixels> {
        Bounds {
            origin: point(px(-100000.), px(-100000.)),
            size: size(px(200000.), px(200000.)),
        }
    }

    /// A clip through the middle of the path, so that tiles cross its edges.
    fn partial_mask(path: &Path) -> Bounds<Pixels> {
        let bounds = path.bounds();
        let third = size(bounds.size.width / 3.0, bounds.size.height / 3.0);
        Bounds {
            origin: point(
                bounds.origin.x + third.width + px(0.3),
                bounds.origin.y + third.height - px(0.4),
            ),
            size: size(third.width * 1.5, third.height * 1.5),
        }
    }

    fn paint(path: &Path, scale: f32, mask: Bounds<Pixels>) -> Scene {
        let mut scene = Scene::default();
        scene.insert_primitive(PaintedPath::new(
            path.clone(),
            scale,
            ContentMask { bounds: mask },
            black().into(),
        ));
        scene.finish();
        scene
    }

    /// Returns the number of pixels checked.
    fn check(
        name: &str,
        curves: &[PathCurve],
        path: &Path,
        scale: f32,
        mask: Bounds<Pixels>,
    ) -> usize {
        const SAMPLES: usize = 40;
        let scene = paint(path, scale, mask);
        let even_odd = path.even_odd();
        let clip = mask.scale(scale);
        let area = path.bounds().scale(scale).intersect(&clip);
        // Two pixels beyond the path, to catch stray coverage.
        let left = area.origin.x.0.floor() - 2.0;
        let top = area.origin.y.0.floor() - 2.0;
        let width = area.right().0.ceil() + 2.0 - left;
        let height = area.bottom().0.ceil() + 2.0 - top;
        // Every pixel of a small path, or an even spread over a large one.
        let columns = (width as usize).min(SAMPLES);
        let rows = (height as usize).min(SAMPLES);
        let mut checked = 0;
        for row in 0..rows {
            for column in 0..columns {
                let center = point(
                    (left + width * column as f32 / columns as f32).floor() + 0.5,
                    (top + height * row as f32 / rows as f32).floor() + 0.5,
                );
                if center.x < clip.origin.x.0
                    || center.x >= clip.right().0
                    || center.y < clip.origin.y.0
                    || center.y >= clip.bottom().0
                {
                    continue;
                }
                let logical = point(center.x / scale, center.y / scale);
                let mut winding = [0.0; 2];
                let mut weight = [0.0; 2];
                for curve in curves {
                    add_curve_winding(curve, logical, scale, &mut winding, &mut weight);
                }
                let expected = coverage(winding, weight, even_odd);
                let actual = shade(&scene, center).unwrap_or_else(|owners| {
                    panic!("{name} at scale {scale}: pixel {center:?} lies in {owners} tiles")
                });
                assert!(
                    (actual - expected).abs() < 2e-4,
                    "{name} at scale {scale}, even-odd {even_odd}, mask {mask:?}: pixel {center:?} has coverage {actual}, expected {expected}",
                );
                checked += 1;
            }
        }
        checked
    }

    /// Emulates `path_tile_vertex`, the rasterizer's top-left rule, and
    /// `path_tile_fragment` for the device pixel centered at `center`. Errors
    /// with the number of tiles covering the pixel if more than one does.
    fn shade(scene: &Scene, center: Point<f32>) -> Result<f32, usize> {
        let mut owners = 0;
        let mut result = 0.0;
        for tile in &scene.path_tiles {
            let paint = &scene.path_paints[tile.paint as usize];
            let scale = paint.scale;
            let mask = paint.content_mask.bounds;
            let mask_top_left = point(mask.origin.x.0, mask.origin.y.0);
            let mask_bottom_right = point(
                mask.origin.x.0 + mask.size.width.0,
                mask.origin.y.0 + mask.size.height.0,
            );
            let top_left = point(
                (tile.top_left.x * scale).clamp(mask_top_left.x, mask_bottom_right.x),
                (tile.top_left.y * scale).clamp(mask_top_left.y, mask_bottom_right.y),
            );
            let bottom_right = point(
                (tile.bottom_right.x * scale).clamp(top_left.x, mask_bottom_right.x),
                (tile.bottom_right.y * scale).clamp(top_left.y, mask_bottom_right.y),
            );
            if center.x < top_left.x
                || center.x >= bottom_right.x
                || center.y < top_left.y
                || center.y >= bottom_right.y
            {
                continue;
            }
            owners += 1;

            let logical = point(center.x / scale, center.y / scale);
            let half_pixel = 0.5 / scale;
            let mut winding = [tile.backdrop as f32; 2];
            let mut weight = [0.0; 2];
            let start = tile.curve_start as usize;
            for curve in &scene.path_curves[start..start + tile.curve_count as usize] {
                if curve.p0.x.min(curve.p1.x) >= logical.x + half_pixel + SORT_SLACK {
                    break;
                }
                if curve.p0.y < logical.y + half_pixel {
                    add_curve_winding(curve, logical, scale, &mut winding, &mut weight);
                    for (endpoint, step) in [(curve.p0, curve.sign), (curve.p1, -curve.sign)] {
                        if endpoint.x <= tile.reference.x {
                            winding[0] -= step
                                * ((endpoint.y <= logical.y) as u8 as f32
                                    - (endpoint.y <= tile.reference.y) as u8 as f32);
                        }
                        if endpoint.y <= tile.reference.y && endpoint.x <= logical.x {
                            winding[1] += step;
                        }
                    }
                }
            }
            result = coverage(winding, weight, paint.even_odd.as_bool());
        }
        if owners > 1 { Err(owners) } else { Ok(result) }
    }

    fn add_curve_winding(
        curve: &PathCurve,
        center: Point<f32>,
        scale: f32,
        winding: &mut [f32; 2],
        weight: &mut [f32; 2],
    ) {
        let half_pixel = 0.5 / scale;
        let min_x = curve.p0.x.min(curve.p1.x);
        let max_x = curve.p0.x.max(curve.p1.x);
        if curve.p0.y <= center.y && center.y < curve.p1.y {
            if max_x <= center.x - half_pixel {
                winding[0] += curve.sign;
            } else {
                let t = monotone_quadratic_root(curve.a.y, curve.b.y, curve.p0.y - center.y, 1.0);
                let x = (curve.a.x * t + curve.b.x) * t + curve.p0.x;
                let distance = (center.x - x) * scale;
                winding[0] += curve.sign * (distance + 0.5).clamp(0.0, 1.0);
                weight[0] = weight[0].max((1.0 - 2.0 * distance.abs()).clamp(0.0, 1.0));
            }
        }
        if min_x <= center.x && center.x < max_x {
            let direction = if curve.p1.x > curve.p0.x { 1.0 } else { -1.0 };
            if curve.p1.y <= center.y - half_pixel {
                winding[1] -= curve.sign * direction;
            } else if curve.p0.y < center.y + half_pixel {
                let t =
                    monotone_quadratic_root(curve.a.x, curve.b.x, curve.p0.x - center.x, direction);
                let y = (curve.a.y * t + curve.b.y) * t + curve.p0.y;
                let distance = (center.y - y) * scale;
                winding[1] -= curve.sign * direction * (distance + 0.5).clamp(0.0, 1.0);
                weight[1] = weight[1].max((1.0 - 2.0 * distance.abs()).clamp(0.0, 1.0));
            }
        }
    }

    fn coverage(winding: [f32; 2], weight: [f32; 2], even_odd: bool) -> f32 {
        let coverage = ((winding[0] * weight[0] + winding[1] * weight[1]).abs()
            / (weight[0] + weight[1]).max(1.0 / 65536.0))
        .max(winding[0].abs().min(winding[1].abs()));
        if even_odd {
            1.0 - (1.0 - (coverage * 0.5).fract() * 2.0).abs()
        } else {
            coverage.clamp(0.0, 1.0)
        }
    }

    fn polygon(points: &[(f32, f32)]) -> PathBuilder {
        let mut builder = PathBuilder::fill();
        let points: Vec<_> = points.iter().map(|&(x, y)| point(px(x), px(y))).collect();
        builder.add_polygon(&points, true);
        builder
    }

    fn shapes(offset: Point<Pixels>) -> Vec<(&'static str, Vec<PathCurve>)> {
        let mut shapes = Vec::new();
        let mut add = |name, mut builder: PathBuilder| {
            builder.translate(offset);
            shapes.push((name, builder.outline().0));
        };

        add(
            "star",
            polygon(&[
                (350., 200.),
                (370., 260.),
                (430., 260.),
                (380., 300.),
                (400., 360.),
                (350., 320.),
                (300., 360.),
                (320., 300.),
                (270., 260.),
                (330., 260.),
            ]),
        );

        // Self-intersecting, so the fill rules differ inside.
        add(
            "pentagram",
            polygon(&[
                (100., 10.),
                (159., 190.),
                (5., 78.),
                (195., 78.),
                (41., 190.),
            ]),
        );

        let mut curves = PathBuilder::fill();
        curves.move_to(point(px(450.), px(280.)));
        curves.curve_to(point(px(530.), px(230.)), point(px(450.), px(230.)));
        curves.arc_to(
            point(px(30.), px(30.)),
            px(0.),
            false,
            false,
            point(px(590.), px(230.)),
        );
        curves.close();
        add("curves", curves);

        for (name, left, top, right, bottom) in [
            ("aligned rectangle", 0.0, 0.0, 32.0, 32.0),
            ("offset aligned rectangle", 16.0, 16.0, 64.0, 48.0),
            ("negative aligned rectangle", -32.0, -16.0, 16.0, 16.0),
            ("fractional rectangle", 1.6, 1.6, 60.0, 40.0),
        ] {
            add(
                name,
                polygon(&[(left, top), (right, top), (right, bottom), (left, bottom)]),
            );
        }

        // The bounds start a margin past whole pixels, so cells start at 31,
        // and 63 and 95 are cell edges. Vertices land on cell corners, edges
        // run along cell edges, and other edges lie exactly a margin away.
        let near = 31.0 + MARGIN;
        let mut cell_edges = PathBuilder::fill();
        for points in [
            [(near, near), (63.0, near), (63.0, 63.0), (near, 63.0)],
            [(63.0, 63.0), (95.0, 63.0), (95.0, 95.0), (63.0, 95.0)],
            [
                (63.0 + MARGIN, 40.0),
                (95.0 - MARGIN, 40.0),
                (95.0 - MARGIN, 63.0 - MARGIN),
                (63.0 + MARGIN, 63.0 - MARGIN),
            ],
            [
                (40.0, 63.0 + MARGIN),
                (63.0 - MARGIN, 63.0 + MARGIN),
                (63.0 - MARGIN, 127.0),
                (40.0, 127.0),
            ],
        ] {
            let points: Vec<_> = points.iter().map(|&(x, y)| point(px(x), px(y))).collect();
            cell_edges.add_polygon(&points, true);
        }
        add("cell edges", cell_edges);

        add(
            "sliver",
            polygon(&[(100., 100.), (400., 100.00002), (400., 100.)]),
        );

        let mut far = PathBuilder::fill();
        far.move_to(point(px(12000.3), px(9000.7)));
        far.line_to(point(px(12060.3), px(9000.7)));
        far.arc_to(
            point(px(30.), px(30.)),
            px(0.),
            false,
            true,
            point(px(12030.3), px(9050.)),
        );
        far.close();
        add("far away", far);

        let mut selection = PathBuilder::fill();
        selection.move_to(point(px(104.), px(40.)));
        for (corner, next) in [
            ((900., 40.), (904., 44.)),
            ((904., 600.), (900., 604.)),
            ((100., 604.), (100., 600.)),
            ((100., 40.), (104., 40.)),
        ] {
            let start = point(px(corner.0), px(corner.1));
            selection.line_to(start);
            selection.arc_to(
                point(px(4.), px(4.)),
                px(0.),
                false,
                true,
                point(px(next.0), px(next.1)),
            );
        }
        selection.close();
        add("selection", selection);

        let mut wave = PathBuilder::stroke(px(2.));
        wave.move_to(point(px(40.), px(120.)));
        for i in 1..20 {
            let x = i as f32 * 10.0;
            wave.line_to(point(px(40. + x), px(120. + x.sin() * 40.)));
        }
        add("wave", wave);

        let mut dashed_arc = PathBuilder::stroke(px(3.)).dash_array(&[px(9.), px(4.)]);
        dashed_arc.move_to(point(px(50.), px(10.)));
        dashed_arc.arc_to(
            point(px(40.), px(40.)),
            px(0.),
            false,
            true,
            point(px(90.), px(50.)),
        );
        add("dashed arc", dashed_arc);

        let mut ring = PathBuilder::stroke(px(2.));
        ring.move_to(point(px(106.), px(100.)));
        for end in [94., 106.] {
            ring.arc_to(
                point(px(6.), px(6.)),
                px(0.),
                false,
                true,
                point(px(end), px(100.)),
            );
        }
        ring.close();
        add("small ring", ring);

        for (name, width, from, to) in [
            ("diagonal", 2.0, (40., 40.), (900., 500.)),
            ("horizontal line", 1.0, (40., 88.), (900., 88.)),
            ("vertical line", 1.0, (88., 40.), (88., 600.)),
            ("hairline", 0.25, (40., 300.), (700., 310.)),
        ] {
            let mut line = PathBuilder::stroke(px(width));
            line.move_to(point(px(from.0), px(from.1)));
            line.line_to(point(px(to.0), px(to.1)));
            add(name, line);
        }

        let mut curved = PathBuilder::stroke(px(2.));
        curved.move_to(point(px(80.), px(500.)));
        curved.cubic_bezier_to(
            point(px(900.), px(250.)),
            point(px(300.), px(50.)),
            point(px(700.), px(800.)),
        );
        add("curved stroke", curved);

        // A random walk like the painting example's scribble, crossing
        // itself many times.
        let mut scribble = PathBuilder::stroke(px(1.));
        let mut random = xorshift(0x9E37_79B9);
        let (mut x, mut y, mut angle) = (300.0f32, 200.0f32, 0.0f32);
        scribble.move_to(point(px(x), px(y)));
        for _ in 0..300 {
            angle += (random() - 0.5) * 1.2;
            x = (x + angle.cos() * 12.0).clamp(40.0, 600.0);
            y = (y + angle.sin() * 12.0).clamp(40.0, 400.0);
            scribble.line_to(point(px(x), px(y)));
        }
        add("scribble", scribble);

        shapes
    }
}
