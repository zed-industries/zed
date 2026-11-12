//! Path fills by per-pixel winding.
//!
//! [`PathBuilder`](crate::PathBuilder) turns contours into closed chains of
//! monotone quadratic curves. To paint the resulting [`Path`](crate::Path), we compute the
//! winding number at each pixel on the GPU, as in Eric Lengyel's Slug: we
//! cast rays leftward and upward from the pixel and count crossings with the
//! path's curves. Crossings within the pixel contribute partial winding to
//! approximate coverage across its area. Coverage is then determined by the
//! fill rule.
//!
//! To reduce the number of curves tested against each ray, we preprocess the
//! path on the CPU, binning its curves into square cells. Each cell lists the
//! curves intersecting or near it. The cell size is chosen per path to balance
//! the cost of binning against the cost of shading (see `cell_size`).
//!
//! Curves outside a cell's list cannot contribute partial winding, so the
//! binner sums their winding contributions along a leftward ray from the
//! cell's top-left corner into a *backdrop*. For each pixel, the shader starts
//! with this backdrop and adds the contributions of the listed curves. It also
//! corrects the backdrop to account for unlisted curves' contribution between the
//! corner and the pixel.
//!
//! Neighboring cells with the same curve list and backdrop are merged into
//! rectangular [`PathTile`]s, which are uploaded to and drawn by the GPU. For
//! example, a large rectangle can become nine tiles: four corners, four edges,
//! and an interior.

use crate::{Bounds, PathCurve, PathTile, Point, ScaledPixels, point};
use std::cell::RefCell;

/// A tile lists every curve within this distance of its bounds. Any other curve
/// is more than half a pixel away from the tile, so it cannot contribute partial winding.
/// The extra 1/256 covers the rasterizer snapping tile edges to its subpixel grid.
const MARGIN: f32 = 0.5 + 1.0 / 256.0;

/// `f32::min`, `max` and `clamp` without IEEE-compliant NaN handling, which costs a
/// compare and blend per call on x86-64's baseline target.
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

/// A path's curves binned into tiles for one display scale.
#[derive(Debug, Default)]
pub(crate) struct PathTiles {
    bounds: Bounds<ScaledPixels>,
    tiles: Vec<PathTile>,
    /// Every tile's curve list, concatenated.
    curves: Vec<PathCurve>,
}

impl PathTiles {
    pub(crate) fn bounds(&self) -> Bounds<ScaledPixels> {
        self.bounds
    }

    pub(crate) fn painted_bounds(&self) -> Bounds<ScaledPixels> {
        if self.tiles.is_empty() {
            return self.bounds;
        }
        self.bounds.dilate(ScaledPixels(MARGIN))
    }

    pub(crate) fn tiles(&self) -> &[PathTile] {
        &self.tiles
    }

    pub(crate) fn curves(&self) -> &[PathCurve] {
        &self.curves
    }

    /// Drops the tiles that lie entirely outside `mask`.
    pub(crate) fn retain_visible(&mut self, mask: Bounds<ScaledPixels>) {
        self.tiles.retain(|tile| {
            tile.top_left.x < mask.right().0
                && tile.top_left.y < mask.bottom().0
                && tile.bottom_right.x > mask.origin.x.0
                && tile.bottom_right.y > mask.origin.y.0
        });
    }
}

pub(crate) fn bin(curves: &[PathCurve], even_odd: bool, scale: f32) -> PathTiles {
    debug_assert!(scale > 0.0 && scale.is_finite(), "invalid scale {scale}");
    if curves.is_empty() {
        return PathTiles::default();
    }
    BINNER.with_borrow_mut(|binner| {
        let (min, max, length) = binner.scale(curves, scale);
        let Some(grid) = Grid::covering(min, max, curves.len(), length) else {
            log::warn!("path spanning {min:?} to {max:?} device pixels overflows; skipping");
            return PathTiles::default();
        };
        binner.find_spans(&grid);
        binner.fill_cells();
        let (tiles, curves) = binner.merge_cells(&grid, even_odd);
        let device = |p: Point<f32>| point(ScaledPixels(p.x), ScaledPixels(p.y));
        PathTiles {
            bounds: Bounds::from_corners(device(min), device(max)),
            tiles,
            curves,
        }
    })
}

/// Picks the power-of-two cell size that minimizes the estimated time spent
/// binning and shading. Smaller cells list each curve in more cells, which
/// costs CPU time. Larger cells make the shader evaluate each listed curve at
/// more pixels, which costs GPU time.
///
/// We estimate the binning and shading time by the following cost model,
/// expressed in terms of shader evaluations of one curve at one pixel:
///
/// ```text
/// cost = listings * (LISTING_CPU_COST + area) + empty_cells * EMPTY_CELL_CPU_COST
/// ```
fn cell_size(curves: f32, length: f32, size: Point<f32>) -> f32 {
    // The CPU time of a listing and an empty cell, in shader evaluations,
    // measured about 634 and 207 on a desktop (RX 6600) and 68 and 21 on an
    // integrated GPU (HD 4600). Of the values in between, these lost the least
    // on either machine, measuring CPU and GPU time on realistic paths. Walking
    // a cell that lists curves has no cost: giving it one made dashed lines use
    // larger cells, which cost far more GPU time than it saved on the CPU.
    const LISTING_CPU_COST: f32 = 320.0;
    const EMPTY_CELL_CPU_COST: f32 = 95.0;
    // Smaller cells save GPU time, but on realistic paths they measured
    // costing more CPU time than they saved, on both machines.
    const MIN_CELL_SIZE: f32 = 16.0;
    // Limits the grid's memory and walking time when the estimate undercounts
    // empty cells, as when most curves crowd into a few cells of a large path.
    const MAX_GRID_CELLS: f32 = 4096.0;

    let grid_cells = |cell_size: f32| {
        ((size.x / cell_size).floor() + 1.0) * ((size.y / cell_size).floor() + 1.0)
    };
    let extent = size.x.fast_max(size.y);
    let mut cell_size = MIN_CELL_SIZE;
    while grid_cells(cell_size) > MAX_GRID_CELLS {
        cell_size *= 2.0;
    }
    let mut best = (f32::INFINITY, cell_size);
    loop {
        let listings = if cell_size >= extent {
            curves
        } else {
            // Since curves are monotone, each crosses about `(w + h) / cell_size` grid lines.
            curves + length / cell_size
        };
        let area = cell_size.fast_min(size.x) * cell_size.fast_min(size.y) / 2.0;
        let empty_cells = (grid_cells(cell_size) - listings).fast_max(0.0);
        let cost = listings * (LISTING_CPU_COST + area) + empty_cells * EMPTY_CELL_CPU_COST;
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
struct Binner {
    /// The path's curves, scaled into device pixels.
    input: Vec<PathCurve>,
    spans: Vec<Span>,
    /// Per cell, where its list ends in `keys`. Reused while binning, as in a
    /// counting sort: `find_spans` stores each cell's count here, and
    /// `fill_cells` turns the counts into starts and advances them as it
    /// fills each list.
    cell_ends: Vec<usize>,
    /// Per cell, the winding steps of curves crossing its row's top to its
    /// left, other than those it lists.
    steps: Vec<i32>,
    /// Each cell's listings: a curve's leftmost x above its index.
    keys: Vec<u64>,
    /// The current row's runs.
    runs: Vec<Run>,
    /// The previous row's runs, whose tiles the current row can extend down.
    above: Vec<Run>,
}

thread_local! {
    static BINNER: RefCell<Binner> = Default::default();
}

/// A curve's run of consecutive cells within a row.
struct Span {
    cell: usize,
    count: usize,
    key: u64,
}

impl Binner {
    fn scale(&mut self, curves: &[PathCurve], scale: f32) -> (Point<f32>, Point<f32>, f32) {
        let mut min = point(f32::INFINITY, f32::INFINITY);
        let mut max = point(f32::NEG_INFINITY, f32::NEG_INFINITY);
        let mut length = 0.0;
        self.input.clear();
        self.input.extend(curves.iter().map(|curve| {
            let curve = curve.scaled(scale);
            // The extents of a monotone curve can be computed from its endpoints.
            min.x = min.x.fast_min(curve.min_x());
            max.x = max.x.fast_max(curve.max_x());
            min.y = min.y.fast_min(curve.p0.y);
            max.y = max.y.fast_max(curve.p1.y);
            length += curve.max_x() - curve.min_x() + curve.p1.y - curve.p0.y;
            curve
        }));
        (min, max, length)
    }

    fn find_spans(&mut self, grid: &Grid) {
        let cells = grid.rows * grid.columns;
        self.spans.clear();
        self.cell_ends.clear();
        self.cell_ends.resize(cells, 0);
        self.steps.clear();
        self.steps.resize(cells, 0);
        for (index, curve) in self.input.iter().enumerate() {
            // This key sorts curves by leftmost x, which is used by the shader's early exit;
            // then by index, so that curves are totally ordered.
            let key = u64::from(ordered_bits(curve.min_x())) << 32 | index as u64;
            for row in grid.row_of(curve.p0.y - MARGIN)..=grid.row_of(curve.p1.y + MARGIN) {
                let row_top = grid.y(row);
                let (low, high) = curve.x_range(row_top - MARGIN, grid.y(row + 1) + MARGIN);
                let first = grid.column_of(low - MARGIN);
                let last = grid.column_of(high + MARGIN);
                let cell = row * grid.columns + first;
                let count = last + 1 - first;
                self.spans.push(Span { cell, count, key });
                for listings in self
                    .cell_ends
                    .get_mut(cell..cell + count)
                    .unwrap_or_default()
                {
                    *listings += 1;
                }

                // If this curve contributes to cell backdrops, record its contribution
                // in the first cell to the right of its span.
                if curve.crosses(row_top)
                    && last + 1 < grid.columns
                    && let Some(step) = self.steps.get_mut(cell + count)
                {
                    *step += curve.step();
                }
            }
        }
    }

    fn fill_cells(&mut self) {
        let mut total = 0;
        for end in self.cell_ends.iter_mut() {
            let count = *end;
            *end = total;
            total += count;
        }
        self.keys.clear();
        self.keys.resize(total, 0);
        for span in &self.spans {
            for cursor in self
                .cell_ends
                .get_mut(span.cell..span.cell + span.count)
                .unwrap_or_default()
            {
                if let Some(key) = self.keys.get_mut(*cursor) {
                    *key = span.key;
                }
                *cursor += 1;
            }
        }
        let mut start = 0;
        for &end in &self.cell_ends {
            self.keys
                .get_mut(start..end)
                .unwrap_or_default()
                .sort_unstable();
            start = end;
        }
    }

    fn merge_cells(&mut self, grid: &Grid, even_odd: bool) -> (Vec<PathTile>, Vec<PathCurve>) {
        let Binner {
            input,
            steps,
            cell_ends,
            keys,
            runs,
            above,
            ..
        } = self;
        let mut tiles: Vec<PathTile> = Vec::new();
        // Each listing is copied at most once.
        let mut curves = Vec::with_capacity(keys.len());
        above.clear();
        let mut cell_start = 0;
        for row in 0..grid.rows {
            runs.clear();
            let mut backdrop = 0;
            for column in 0..grid.columns {
                let index = row * grid.columns + column;
                backdrop += steps.get(index).copied().unwrap_or(0);
                let cell_end = cell_ends.get(index).copied().unwrap_or(cell_start);
                let list = (cell_start, cell_end);
                cell_start = cell_end;

                let cell = grid.cell(column, row);
                let quad = if list.0 == list.1 {
                    fills(backdrop, even_odd).then(|| (cell.top_left(), cell.bottom_right()))
                } else {
                    let mut measure = Measure::new(cell, backdrop);
                    for curve in listed(keys, input, list) {
                        measure.add(curve);
                    }
                    measure.quad(even_odd)
                };
                let Some((top_left, bottom_right)) = quad else {
                    continue;
                };
                let run = Run {
                    start: column,
                    end: column + 1,
                    backdrop,
                    list,
                    top_left,
                    bottom_right,
                    tile: 0,
                };
                match runs.last_mut() {
                    Some(last) if last.end == run.start && last.shades_like(&run, keys) => {
                        last.end = run.end;
                        extend_quad(&mut last.top_left, &mut last.bottom_right, &run);
                    }
                    _ => runs.push(run),
                }
            }

            // Both rows' runs are in column order.
            let mut above_runs = above.iter().peekable();
            for run in runs.iter_mut() {
                while above_runs
                    .next_if(|other| other.start < run.start)
                    .is_some()
                {}
                let matching = above_runs.next_if(|other| {
                    other.start == run.start && other.end == run.end && other.shades_like(run, keys)
                });
                run.tile = match matching {
                    Some(above_run) => {
                        if let Some(tile) = tiles.get_mut(above_run.tile) {
                            extend_quad(&mut tile.top_left, &mut tile.bottom_right, run);
                        }
                        above_run.tile
                    }
                    None => {
                        let curve_start = curves.len();
                        // Tiles are trimmed to their curves' extents, so the path is known to be
                        // filled for pixels outside this box.
                        let mut curves_top_left = point(f32::INFINITY, f32::INFINITY);
                        let mut curves_bottom_right = point(f32::NEG_INFINITY, f32::NEG_INFINITY);
                        for curve in listed(keys, input, run.list) {
                            // The rasterizer snaps tile edges to its subpixel grid, so it can
                            // shade a pixel just outside a tile. Where the tile is trimmed, that
                            // pixel would also be outside the box, and be wrongly filled. Dilate
                            // the box by more than the largest snapping error to prevent this.
                            const BOX_MARGIN: f32 = MARGIN + 1.0 / 16.0;

                            curves_top_left.x =
                                curves_top_left.x.fast_min(curve.min_x() - BOX_MARGIN);
                            curves_top_left.y = curves_top_left.y.fast_min(curve.p0.y - BOX_MARGIN);
                            curves_bottom_right.x =
                                curves_bottom_right.x.fast_max(curve.max_x() + BOX_MARGIN);
                            curves_bottom_right.y =
                                curves_bottom_right.y.fast_max(curve.p1.y + BOX_MARGIN);
                            curves.push(*curve);
                        }
                        tiles.push(PathTile {
                            paint: 0,
                            curve_start: curve_start as u32,
                            curve_count: (curves.len() - curve_start) as u32,
                            top_left: run.top_left,
                            bottom_right: run.bottom_right,
                            reference: grid.cell(run.start, row).top_left(),
                            backdrop: run.backdrop,
                            curves_top_left,
                            curves_bottom_right,
                        });
                        tiles.len() - 1
                    }
                };
            }
            std::mem::swap(above, runs);
        }
        (tiles, curves)
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

/// Consecutive cells in a row, over columns `start..end`, that would be
/// shaded identically.
#[derive(Clone, Copy)]
struct Run {
    start: usize,
    end: usize,
    backdrop: i32,
    /// The cells' list, as a range of `keys`.
    list: (usize, usize),
    top_left: Point<f32>,
    bottom_right: Point<f32>,
    tile: usize,
}

impl Run {
    /// Whether the two have the same backdrop and list the same curves.
    fn shades_like(&self, other: &Run, keys: &[u64]) -> bool {
        self.backdrop == other.backdrop
            && keys.get(self.list.0..self.list.1) == keys.get(other.list.0..other.list.1)
    }
}

/// Grows a quad to cover a run's quad.
fn extend_quad(top_left: &mut Point<f32>, bottom_right: &mut Point<f32>, run: &Run) {
    top_left.x = top_left.x.fast_min(run.top_left.x);
    top_left.y = top_left.y.fast_min(run.top_left.y);
    bottom_right.x = bottom_right.x.fast_max(run.bottom_right.x);
    bottom_right.y = bottom_right.y.fast_max(run.bottom_right.y);
}

/// The curves in a range of `keys`.
fn listed<'a>(
    keys: &'a [u64],
    input: &'a [PathCurve],
    list: (usize, usize),
) -> impl Iterator<Item = &'a PathCurve> {
    keys.get(list.0..list.1)
        .unwrap_or_default()
        .iter()
        .filter_map(move |&key| input.get(key as u32 as usize))
}

struct Grid {
    left: f32,
    top: f32,
    cell_size: f32,
    columns: usize,
    rows: usize,
}

impl Grid {
    fn covering(min: Point<f32>, max: Point<f32>, curves: usize, length: f32) -> Option<Self> {
        let left = (min.x - MARGIN).floor();
        let top = (min.y - MARGIN).floor();
        let size = point(max.x + MARGIN - left, max.y + MARGIN - top);
        if !(size.x.is_finite() && size.y.is_finite()) {
            return None;
        }
        let cell_size = cell_size(curves as f32, length, size);
        Some(Self {
            left,
            top,
            cell_size,
            columns: (size.x / cell_size) as usize + 1,
            rows: (size.y / cell_size) as usize + 1,
        })
    }

    fn column_of(&self, x: f32) -> usize {
        let column = (x - self.left) / self.cell_size;
        column.fast_clamp(0.0, (self.columns - 1) as f32) as u32 as usize
    }

    fn row_of(&self, y: f32) -> usize {
        let row = (y - self.top) / self.cell_size;
        row.fast_clamp(0.0, (self.rows - 1) as f32) as u32 as usize
    }

    fn x(&self, column: usize) -> f32 {
        self.left + column as f32 * self.cell_size
    }

    fn y(&self, row: usize) -> f32 {
        self.top + row as f32 * self.cell_size
    }

    fn cell(&self, column: usize, row: usize) -> Cell {
        Cell {
            left: self.x(column),
            top: self.y(row),
            right: self.x(column + 1),
            bottom: self.y(row + 1),
        }
    }
}

#[derive(Clone, Copy)]
struct Cell {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Cell {
    fn top_left(self) -> Point<f32> {
        point(self.left, self.top)
    }

    fn bottom_right(self) -> Point<f32> {
        point(self.right, self.bottom)
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
        if curve.crosses(cell.top) {
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
        let clamp_x = |x: f32| x.fast_clamp(cell.left, cell.right);
        let clamp_y = |y: f32| y.fast_clamp(cell.top, cell.bottom);
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
            cell.right
        } else {
            clamp_x(self.max.x + MARGIN)
        };
        let bottom = if fills(self.bottom_winding, even_odd) {
            cell.bottom
        } else {
            clamp_y(self.max.y + MARGIN)
        };
        (left < right && top < bottom).then(|| (point(left, top), point(right, bottom)))
    }
}

impl PathCurve {
    fn scaled(&self, scale: f32) -> Self {
        let scale_point = |p: Point<f32>| point(p.x * scale, p.y * scale);
        Self {
            p0: scale_point(self.p0),
            p1: scale_point(self.p1),
            a: scale_point(self.a),
            b: scale_point(self.b),
            sign: self.sign,
            pad: 0,
        }
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

    /// Whether the curve crosses the horizontal line at `y`. It's half-open,
    /// so where two curves meet on the line, exactly one of them crosses it.
    fn crosses(&self, y: f32) -> bool {
        self.p0.y <= y && y < self.p1.y
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
            return (self.a.x * t + self.b.x) * t + self.p0.x;
        }
        let t = monotone_quadratic_root(self.a.y, self.b.y, self.p0.y - y, 1.0);
        (self.a.x * t + self.b.x) * t + self.p0.x
    }
}

/// Solves a*t^2 + b*t + c = 0 for the root in [0, 1] on a monotone piece.
fn monotone_quadratic_root(a: f32, b: f32, c: f32, direction: f32) -> f32 {
    let discriminant = (b * b - 4.0 * a * c).fast_max(0.0);
    let denominator = b + direction * discriminant.sqrt();
    if denominator == 0.0 {
        return 0.0;
    }
    (-2.0 * c / denominator).fast_clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ContentMask, Outline, PaintedPath, Path, PathBuilder, Pixels, Scene, black, px, size,
    };

    /// A rectangle spanning many cells is four edge strips, four corners and
    /// one interior, however many cells it covers.
    #[test]
    fn rectangle_is_nine_tiles() {
        let path = square(100., 40., 900., 600.);
        assert_eq!(path.bin(1.25).tiles().len(), 9);
    }

    /// A path far larger than the window is binned coarsely, and painting it
    /// keeps only the tiles its clip shows.
    #[test]
    fn huge_path_paints_only_visible_tiles() {
        let path = square(-20000., -20000., 20000., 20000.);
        let tile_count = path.bin(1.5).tiles().len();
        assert!(tile_count < 10_000, "{tile_count} tiles");

        let mask = Bounds {
            origin: point(px(300.), px(200.)),
            size: size(px(120.), px(90.)),
        };
        let mut scene = Scene::default();
        scene.insert_primitive(PaintedPath::new(
            path,
            1.5,
            ContentMask { bounds: mask },
            black().into(),
        ));
        scene.finish();
        let painted: usize = scene.paths.iter().map(PaintedPath::tile_count).sum();
        assert!(painted > 0 && painted < 20, "{painted} tiles");
    }

    /// A path whose size overflows, at some scale or at any, is skipped.
    #[test]
    fn overflowing_paths_are_skipped() {
        let radius = f32::MAX / 4.0;
        let path = square(-radius, -radius, radius, radius);
        assert!(!path.bin(1.0).tiles().is_empty());
        assert!(path.bin(4.0).tiles().is_empty());
        let radius = f32::MAX * 0.75;
        let path = square(-radius, -radius, radius, radius);
        assert!(path.bin(1.0).tiles().is_empty());
    }

    /// A non-finite point fails the whole path. Dropping only its segments
    /// would leave its contour open.
    #[test]
    fn non_finite_point_fails_path() {
        // Built directly, since lyon asserts its points are finite in debug
        // builds.
        let mut outline = Outline::default();
        outline.move_to(point(0., 0.));
        outline.line_to(point(10., 0.));
        outline.line_to(point(10., 10.));
        outline.move_to(point(20., 0.));
        outline.line_to(point(f32::NAN, 5.));
        outline.line_to(point(30., 10.));
        assert!(outline.into_curves().is_err());
    }

    /// The shader's backdrop corrections need curves that meet to share
    /// bit-identical endpoints. A break shows up as streaks, not as a
    /// failure, so check it directly.
    #[test]
    fn endpoints_balance() {
        let mut curved_fill = PathBuilder::fill();
        curved_fill.move_to(point(px(450.3), px(280.7)));
        curved_fill.curve_to(point(px(530.3), px(230.7)), point(px(450.3), px(230.7)));
        curved_fill.arc_to(
            point(px(30.), px(30.)),
            px(0.),
            false,
            false,
            point(px(590.3), px(230.7)),
        );
        curved_fill.close();

        let mut stroke = PathBuilder::stroke(px(2.));
        stroke.move_to(point(px(80.3), px(500.7)));
        stroke.cubic_bezier_to(
            point(px(900.), px(250.)),
            point(px(300.), px(50.)),
            point(px(700.), px(800.)),
        );

        let mut dashed = PathBuilder::stroke(px(3.)).dash_array(&[px(9.), px(4.)]);
        dashed.move_to(point(px(50.3), px(10.7)));
        dashed.arc_to(
            point(px(40.), px(40.)),
            px(0.),
            false,
            true,
            point(px(90.3), px(50.7)),
        );

        for (name, builder) in [
            ("curved fill", curved_fill),
            ("stroke", stroke),
            ("dashed stroke", dashed),
        ] {
            let (curves, _) = builder.outline().unwrap();
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

    fn square(left: f32, top: f32, right: f32, bottom: f32) -> Path {
        let mut builder = PathBuilder::fill();
        let corners = [(left, top), (right, top), (right, bottom), (left, bottom)];
        let points: Vec<Point<Pixels>> =
            corners.iter().map(|&(x, y)| point(px(x), px(y))).collect();
        builder.add_polygon(&points, true);
        builder.build().unwrap()
    }
}
