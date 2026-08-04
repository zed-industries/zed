//! Tiled per-pixel winding fills.
//!
//! A filled path is rendered by evaluating, per pixel, the winding number of
//! the path's boundary around that pixel and mapping it to coverage through
//! the fill rule. The CPU never compares two curves against each other. As a
//! path is built, every segment is split into xy-monotone quadratic curves
//! ([`MonotoneCurve`]) the moment it is appended — decomposition is
//! per-segment independent, so the monotone form can never go stale.
//! [`Path::painted`] then bins those curves
//! into a fixed grid of device-pixel tiles and counts axis-aligned crossings
//! ([`PaintedPath::bin`]), producing a [`PaintedPath`]: a self-contained
//! primitive that renderers upload directly.
//!
//! Each emitted tile carries a *backdrop* — the winding number at the tile's
//! top-left corner, obtained by counting the crossings of the grid row line
//! that lie to the tile's left — plus the list of curves passing through the
//! tile. The fragment shader corrects that backdrop per pixel by the
//! crossings along an L-shaped route from the tile corner (down the tile's
//! left edge, then right to the pixel). Winding is route-independent, so the
//! L-route gives the same answer as a ray cast from infinity, and
//! axis-aligned legs keep every crossing a single closed-form root solve.

use crate::{
    Background, Bounds, ContentMask, PaddedBool32, PaintedPath, Point, ScaledPixels, point,
};
use lyon::geom::QuadraticBezierSegment;
use std::cell::RefCell;

/// Side length of a tile, in device pixels. Larger tiles mean fewer
/// instances and longer per-pixel loops; smaller tiles mean the reverse.
/// Purely a performance knob — coverage is exact at any value. Mirrored in
/// `shaders.hlsl`, where the vertex shader derives each tile's rectangle
/// from its corner and run length.
const TILE_SIZE: f32 = 16.0;

/// Upper bound on the tiles one path may cover, guarding against
/// pathological (but finite) coordinates turning the grid allocation into a
/// denial of service. Comfortably larger than a 4K window's worth of tiles.
const MAX_TILES: usize = 1 << 22;

/// Marks the end of a tile's curve list.
const NO_ENTRY: u32 = u32::MAX;

/// High bit of [`TileCurve::curve`]: the curve crosses this tile's downward
/// leg below the sample point. Booked once during binning, from the same
/// bucketing the backdrop is built on, and uploaded; the shader never
/// re-derives crossing ownership. It encodes a convention, not a geometric
/// fact, which is why it rides alongside the crossing coordinate instead of
/// being derivable from it. Mirrored in `shaders.hlsl`'s
/// `path_tile_fragment`.
const CURVE_DOWNWARD_LEG_FLAG: u32 = 1 << 31;

/// Snap a device-pixel coordinate to the 1/256-pixel lattice the winding
/// bookkeeping operates on. Applied once, in [`MonotoneCurve::scaled`] —
/// device space is where every discrete decision is made, so it is the only
/// space where the lattice buys anything. Exact for |value| < 32768: the
/// scaled value stays within f32's 24-bit integer range, and the scale
/// factors are powers of two.
fn snap(value: f32) -> f32 {
    (value * 256.0).round() * (1.0 / 256.0)
}

/// An xy-monotone quadratic curve of a path's boundary in logical pixels,
/// stored downward (`p0.y <= p1.y`), grown alongside the path's segments as
/// the path is built. [`Self::scaled`] turns it into the device-space
/// [`PathCurve`] the binner and the shader consume.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MonotoneCurve {
    p0: Point<f32>,
    ctrl: Point<f32>,
    p1: Point<f32>,
    /// `+1` if the contour ran downward through this curve, `-1` if upward.
    sign: f32,
}

/// An xy-monotone quadratic Bézier curve of a path's boundary in device
/// pixels, stored so that `p0.y <= p1.y` regardless of which way the contour
/// ran through it, with its polynomial coefficients precomputed:
/// `x(t) = ax·t² + bx·t + p0.x` and `y(t) = ay·t² + by·t + p0.y`.
///
/// Produced by [`MonotoneCurve::scaled`] once per paint and uploaded;
/// `repr(C)` and layout-matched to its HLSL counterpart. The coefficients
/// are computed once here so the CPU binning and the shader consume
/// bit-identical values instead of each deriving them from a control point
/// — the decided-once-and-uploaded principle the leg bookings follow,
/// extended to the arithmetic itself. The endpoints stay stored alongside
/// them: every discrete gate (span clamps, straddle tests) compares against
/// endpoint values, and reconstructing an endpoint as `p0 + a + b` is not
/// exact in general (the sums can leave f32's 24-bit-exact range even on
/// the lattice), which would reopen the tie class the lattice exists to
/// prevent.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PathCurve {
    /// Upper endpoint.
    pub p0: Point<f32>,
    /// Lower endpoint.
    pub p1: Point<f32>,
    /// Quadratic x coefficient. Exactly zero for line curves — set, not
    /// derived, so no rounding can perturb it — keeping every solve against
    /// a line in `monotone_quadratic_root`'s linear branch.
    pub ax: f32,
    /// Linear x coefficient.
    pub bx: f32,
    /// Quadratic y coefficient. Exactly zero for line curves.
    pub ay: f32,
    /// Linear y coefficient.
    pub by: f32,
    /// `+1` if the contour ran downward through this curve, `-1` if upward.
    /// Horizontal curves are never flipped and always carry `+1`.
    pub sign: f32,
    /// `sign(p1.x - p0.x)` in stored orientation; `0` when the curve is
    /// vertical.
    pub sx: f32,
}

// The GPU consumes these layouts verbatim (HLSL structured buffers add no
// hidden padding for these field types, and neither may Rust).
const _: () = assert!(std::mem::size_of::<PathCurve>() == 40);
const _: () = assert!(std::mem::size_of::<TileCurve>() == 8);
const _: () = assert!(std::mem::size_of::<PathTile>() == 28);
const _: () = assert!(std::mem::size_of::<PathPaint>() == 108);

/// Split one quadratic segment into xy-monotone curves and append them.
/// Called as segments are pushed onto a building [`Path`]; non-finite input
/// is dropped.
pub(crate) fn decompose_quadratic(
    curves: &mut Vec<MonotoneCurve>,
    p0: Point<f32>,
    ctrl: Point<f32>,
    p1: Point<f32>,
) {
    let finite = |p: Point<f32>| p.x.is_finite() && p.y.is_finite();
    if !(finite(p0) && finite(ctrl) && finite(p1)) {
        return;
    }
    let curve = QuadraticBezierSegment {
        from: lyon::math::point(p0.x, p0.y),
        ctrl: lyon::math::point(ctrl.x, ctrl.y),
        to: lyon::math::point(p1.x, p1.y),
    };
    // Match both midpoint formulas builders use: `Path::line_to` stores
    // `0.5 * (a + b)`, and other callers may store the lerp form, which can
    // differ in the last bit.
    let is_line = curve.ctrl
        == lyon::math::point(
            0.5 * (curve.from.x + curve.to.x),
            0.5 * (curve.from.y + curve.to.y),
        )
        || curve.ctrl == curve.from.lerp(curve.to, 0.5);
    if is_line {
        // A line is already monotone, and splitting it would only cost
        // curves.
        curves.extend(MonotoneCurve::from_line(p0, p1));
    } else {
        // `for_each_monotonic` rather than the `_range` variant plus
        // `split_range`: it additionally clamps each sub-curve's control
        // point back inside its endpoint box, which finite precision can
        // otherwise push slightly outside. Everything downstream depends on
        // the curves being *actually* monotone, not just monotone in exact
        // arithmetic.
        curve.for_each_monotonic(&mut |monotone| {
            curves.extend(MonotoneCurve::from_curve(*monotone));
        });
    }
}

/// Per-path paint data shared by every tile of one path, referenced
/// through [`PathTile::paint`]. `repr(C)` and layout-matched to its HLSL
/// counterpart.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PathPaint {
    /// The path's bounds in device pixels, for gradient evaluation.
    pub bounds: Bounds<ScaledPixels>,
    /// Clip rectangle in device pixels, applied via clip distances in the
    /// vertex shader.
    pub content_mask: ContentMask<ScaledPixels>,
    /// The path's paint.
    pub color: Background,
    /// Whether to map winding to coverage under the even-odd rule rather
    /// than the nonzero rule.
    pub even_odd: PaddedBool32,
}

/// GPU instance for one tile of a filled path: a screen-aligned quad whose
/// every pixel resolves its own winding number from `backdrop` plus the
/// curves in `[curve_start, curve_start + curve_count)`.
///
/// Indices are path-local as binned; [`Scene::finish`](crate::Scene)
/// globalizes them while concatenating every path's tiles into the
/// scene-wide buffers renderers upload verbatim. The tile's rectangle is
/// not stored: the vertex shader derives it from `corner` and `run`, and
/// the clip distances it already emits for the content mask cull whatever
/// the mask would have trimmed. `repr(C)` and layout-matched to its HLSL
/// counterpart.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PathTile {
    /// Index of the path's [`PathPaint`] row within the scene's paint
    /// buffer; zero until `Scene::finish` stamps it.
    pub paint: u32,
    /// First index into the tile-curve list; path-local until
    /// `Scene::finish` globalizes it.
    pub curve_start: u32,
    /// Number of curves passing through this tile. The bound is per
    /// instance, so every pixel of the tile runs the same number of loop
    /// iterations.
    pub curve_count: u32,
    /// Winding number at the tile's sample point (`corner`), counting
    /// crossings strictly left of it, half-open in y.
    pub backdrop: i32,
    /// The tile's top-left corner in device pixels — both the backdrop
    /// sample point and the rasterized rectangle's origin. Boundaries may
    /// pass exactly through it; every discrete gate on both sides is
    /// half-open, so a tie lands on exactly one side.
    pub corner: Point<f32>,
    /// Width of the tile in grid cells: 1 for boundary tiles, the run
    /// length for merged interior runs.
    pub run: u32,
}

/// One element of a tile's curve list: which curve, whether its crossing of
/// the tile's downward leg counts, and where that crossing is. Everything
/// per-(tile, curve) the fragment shader would otherwise re-derive per
/// pixel. `repr(C)` and layout-matched to its HLSL counterpart.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TileCurve {
    /// Low bits index the path's curves (path-local); the high bit is the
    /// tile-specific downward-leg booking ([`CURVE_DOWNWARD_LEG_FLAG`]).
    pub curve: u32,
    /// The y at which the curve crosses the tile's left-edge line
    /// (`x = corner.x`), solved once here. Meaningful — and read by the
    /// shader — only when the curve's x-range straddles that line; zero
    /// otherwise. Carried separately from the booking bit because they
    /// answer different questions: the window split needs the crossing
    /// height even when the leg does not count the crossing (one above the
    /// sample point belongs to the backdrop).
    pub leg_y: f32,
}

thread_local! {
    /// Binning scratch reused across paths and frames. Its working set is
    /// proportional to a path's tile grid, so reusing the allocations
    /// matters, and painting happens on one thread; a thread-local keeps
    /// the reuse without threading a scratch handle through every paint
    /// signature.
    static SCRATCH: RefCell<BinScratch> = RefCell::default();
}

#[derive(Default)]
struct BinScratch {
    /// Per tile, the winding delta contributed by boundary crossings booked
    /// between the previous tile's sample point and this tile's, turned
    /// into the winding at each tile's sample point in place by an
    /// exclusive prefix sum along each row.
    backdrops: Vec<i32>,
    /// Per tile, the first node of its curve list, or [`NO_ENTRY`].
    tile_heads: Vec<u32>,
    /// Arena of curve-list nodes, chained through `next`.
    entries: Vec<ScratchEntry>,
}

#[derive(Clone, Copy)]
struct ScratchEntry {
    entry: TileCurve,
    next: u32,
}

/// The tile grid of one path: origin plus extent, in device pixels.
struct Grid {
    left: f32,
    top: f32,
    columns: usize,
    rows: usize,
}

impl Grid {
    fn column_of(&self, x: f32) -> usize {
        let column = (x - self.left) / TILE_SIZE;
        (column as isize).clamp(0, self.columns as isize - 1) as usize
    }

    fn row_of(&self, y: f32) -> usize {
        let row = (y - self.top) / TILE_SIZE;
        (row as isize).clamp(0, self.rows as isize - 1) as usize
    }

    fn cell(&self, row: usize, column: usize) -> usize {
        row * self.columns + column
    }
}

impl PaintedPath {
    /// Bin the given logical-space monotone curves, scaled by `scale`, into
    /// this path's `curves`, `tiles`, and `tile_curves`. Called once, from
    /// [`Path::painted`](crate::Path::painted); `closing_curve` closes an
    /// open final contour without the built path having to be mutated.
    pub(crate) fn bin(
        &mut self,
        monotone_curves: &[MonotoneCurve],
        closing_curve: Option<MonotoneCurve>,
        scale: f32,
    ) {
        let mask = self.paint.bounds.intersect(&self.paint.content_mask.bounds);
        if (monotone_curves.is_empty() && closing_curve.is_none()) || mask.is_empty() {
            return;
        }

        // The geometry extent comes from the curves rather than the path's
        // recorded bounds, which are only conservative. Control points are
        // deliberately not consulted: decomposition guarantees each one lies
        // between its curve's endpoints on both axes, so the endpoint box is
        // already the tight box.
        let mut min = point(f32::INFINITY, f32::INFINITY);
        let mut max = point(f32::NEG_INFINITY, f32::NEG_INFINITY);
        for curve in monotone_curves.iter().chain(closing_curve.as_ref()) {
            let device = curve.scaled(scale);
            min.x = min.x.min(device.p0.x.min(device.p1.x));
            max.x = max.x.max(device.p0.x.max(device.p1.x));
            min.y = min.y.min(device.p0.y);
            max.y = max.y.max(device.p1.y);
            self.curves.push(device);
        }
        if !(min.x.is_finite() && min.y.is_finite() && max.x.is_finite() && max.y.is_finite()) {
            self.curves.clear();
            return;
        }

        // The grid covers only the visible extent: the geometry clipped to
        // the mask. Winding at a visible pixel can depend on geometry that
        // is out of view, but only through the grid's edges, where it is
        // already handled: crossings left of the grid clamp into the margin
        // column and prefix-sum into every visible column, while geometry
        // above, below, or right of the grid can't cross a visible sample
        // line or pixel window at all. Sizing by the mask instead of the
        // geometry keeps a huge path clipped to a small viewport from
        // tripping MAX_TILES (and from paying CPU for invisible cells).
        let visible_min_x = min.x.max(mask.origin.x.0);
        let visible_min_y = min.y.max(mask.origin.y.0);
        let visible_max_x = max.x.min(mask.right().0);
        let visible_max_y = max.y.min(mask.bottom().0);
        if visible_max_x <= visible_min_x || visible_max_y <= visible_min_y {
            // Closed contours wind zero around every point outside their
            // bounds, so a mask that misses the geometry shows nothing.
            self.curves.clear();
            return;
        }

        // One column of margin on the left keeps every crossing's booking
        // inside the grid: a boundary left of the first visible column's
        // sample point must land in a cell that exists to be prefix-summed
        // into the visible columns.
        let left = visible_min_x.floor() - TILE_SIZE;
        let top = visible_min_y.floor();
        let grid = Grid {
            left,
            top,
            columns: ((visible_max_x - left) / TILE_SIZE) as usize + 1,
            rows: ((visible_max_y - top) / TILE_SIZE) as usize + 1,
        };
        let cells = grid.rows.saturating_mul(grid.columns);
        if cells > MAX_TILES {
            log::warn!(
                "path with {} x {} device pixels visible needs {} fill tiles; skipping",
                visible_max_x - visible_min_x,
                visible_max_y - visible_min_y,
                cells
            );
            self.curves.clear();
            return;
        }

        SCRATCH.with_borrow_mut(|scratch| {
            scratch.backdrops.clear();
            scratch.backdrops.resize(cells, 0);
            scratch.tile_heads.clear();
            scratch.tile_heads.resize(cells, NO_ENTRY);
            scratch.entries.clear();

            debug_assert!(self.curves.len() < CURVE_DOWNWARD_LEG_FLAG as usize);
            let grid_right = grid.left + grid.columns as f32 * TILE_SIZE;
            let grid_bottom = grid.top + grid.rows as f32 * TILE_SIZE;
            for (index, curve) in self.curves.iter().enumerate() {
                // Geometry that can't cross a sample line or a pixel window
                // inside the grid: above, below, or right of it. Left is
                // different — crossings out there feed the visible backdrops
                // through the margin column.
                if curve.p1.y <= grid.top
                    || curve.p0.y >= grid_bottom
                    || curve.p0.x.min(curve.p1.x) >= grid_right
                {
                    continue;
                }
                add_curve(scratch, &grid, index as u32, curve);
            }

            // An exclusive prefix sum along each row turns the per-tile
            // crossing deltas into the winding number at each tile's sample
            // point.
            for row in 0..grid.rows {
                let mut winding = 0;
                for column in 0..grid.columns {
                    let cell = grid.cell(row, column);
                    let delta = scratch.backdrops[cell];
                    scratch.backdrops[cell] = winding;
                    winding += delta;
                }
            }

            self.emit_tiles(&grid, &mask, scratch);
        });

        if self.tiles.is_empty() {
            self.curves.clear();
            self.tile_curves.clear();
        }
    }

    fn emit_tiles(&mut self, grid: &Grid, mask: &Bounds<ScaledPixels>, scratch: &BinScratch) {
        let mask_left = mask.origin.x.0;
        let mask_top = mask.origin.y.0;
        let mask_right = mask.right().0;
        let mask_bottom = mask.bottom().0;

        for row in 0..grid.rows {
            let tile_top = grid.top + row as f32 * TILE_SIZE;
            // Tiles wholly outside the mask are skipped here; partially
            // masked tiles are emitted whole, and the clip distances the
            // vertex shader already emits for the content mask trim them.
            // Tile rectangles come from the grid partition, so every pixel
            // is blended by exactly one tile.
            if tile_top >= mask_bottom || tile_top + TILE_SIZE <= mask_top {
                continue;
            }
            let mut column = 0;
            while column < grid.columns {
                let cell = grid.cell(row, column);
                let backdrop = scratch.backdrops[cell];
                let mut entry = scratch.tile_heads[cell];

                if entry == NO_ENTRY {
                    // Interior run: consecutive curveless cells necessarily
                    // share their backdrop, because a cell that received a
                    // scatter delta always also lists the curve that
                    // produced it (the crossing at the sample height lies
                    // within that curve's row-slab x-extent, which is what
                    // the trail binning covers). Every pixel of a curveless
                    // cell has winding exactly equal to the backdrop, so a
                    // wholly-outside run emits nothing and a wholly-inside
                    // run emits one wide constant-coverage quad instead of
                    // one instance per tile — for a large filled shape this
                    // collapses the interior's instance count by the column
                    // count. The equality below is checked rather than
                    // relied upon, to stay robust if the scatter changes.
                    let run_start = column;
                    column += 1;
                    while column < grid.columns {
                        let cell = grid.cell(row, column);
                        if scratch.tile_heads[cell] != NO_ENTRY
                            || scratch.backdrops[cell] != backdrop
                        {
                            break;
                        }
                        column += 1;
                    }
                    let inside = if self.paint.even_odd.as_bool() {
                        backdrop % 2 != 0
                    } else {
                        backdrop != 0
                    };
                    if !inside {
                        continue;
                    }
                    let run_left = grid.left + run_start as f32 * TILE_SIZE;
                    let run_right = grid.left + column as f32 * TILE_SIZE;
                    if run_left >= mask_right || run_right <= mask_left {
                        continue;
                    }
                    self.tiles.push(PathTile {
                        paint: 0,
                        curve_start: 0,
                        curve_count: 0,
                        backdrop,
                        // The backdrop is never corrected with an empty
                        // curve list; the run's first sample point keeps
                        // the corner meaningful anyway.
                        corner: point(run_left, tile_top),
                        run: (column - run_start) as u32,
                    });
                    continue;
                }

                let tile_left = grid.left + column as f32 * TILE_SIZE;
                column += 1;
                if tile_left >= mask_right || tile_left + TILE_SIZE <= mask_left {
                    continue;
                }

                let curve_start = self.tile_curves.len() as u32;
                while entry != NO_ENTRY {
                    let node = scratch.entries[entry as usize];
                    self.tile_curves.push(node.entry);
                    entry = node.next;
                }
                // Sort the tile's list so curves whose x-extent starts
                // rightmost come last: the fragment loop stops at the
                // first curve entirely right of its pixel's column, since
                // every later one must be too (Slug's sorted-band
                // early-out, mirrored for our left-integrated winding). A
                // curve right of the column contributes nothing: the
                // rightward-leg gate rejects it, and it cannot carry a
                // downward-leg booking, which requires straddling the
                // tile's left edge.
                let curves = &self.curves;
                self.tile_curves[curve_start as usize..].sort_unstable_by(|a, b| {
                    let min_x = |entry: &TileCurve| {
                        let curve = &curves[(entry.curve & !CURVE_DOWNWARD_LEG_FLAG) as usize];
                        curve.p0.x.min(curve.p1.x)
                    };
                    min_x(a).total_cmp(&min_x(b))
                });
                self.tiles.push(PathTile {
                    paint: 0,
                    curve_start,
                    curve_count: self.tile_curves.len() as u32 - curve_start,
                    backdrop,
                    corner: point(tile_left, tile_top),
                    run: 1,
                });
            }
        }
    }
}

/// Bucket one device-space curve into every tile its monotone trail touches
/// — booking its downward-leg crossings into the entries' flag bits — and
/// scatter the winding deltas it contributes at the tiles' sample heights.
fn add_curve(scratch: &mut BinScratch, grid: &Grid, index: u32, curve: &PathCurve) {
    let horizontal = curve.p1.y <= curve.p0.y;
    let delta = if curve.sign < 0.0 { -1 } else { 1 };
    let min_x = curve.p0.x.min(curve.p1.x);
    let max_x = curve.p0.x.max(curve.p1.x);
    let first_row = grid.row_of(curve.p0.y);
    let last_row = grid.row_of(curve.p1.y);
    for row in first_row..=last_row {
        let row_top = grid.top + row as f32 * TILE_SIZE;
        // Samples sit exactly on the tile corners; boundaries pass through
        // them freely. Every discrete decision — here and in the shader —
        // evaluates the same one-sided limit, "sample at corner + 0⁺", so
        // a tie lands on the same side everywhere: half-open span and
        // straddle tests, ceil-minus-one column bucketing (a crossing
        // exactly on a column boundary is strictly left of that column's
        // perturbed sample), and the shader's at-or-left-is-dead gates.
        let sample_y = row_top;

        // Where the curve sits at this row's sample height, clamped to its
        // span and bucketed against the sample points' x positions. This
        // one value is the booking that both consumers below act on — the
        // backdrop deltas and the downward-leg flags — so they cannot
        // disagree about who owns a crossing, no matter how the solve
        // rounds. Horizontal curves never span a sample height and their
        // booking is only read through the straddle test, where either
        // endpoint gives the same answer, so they skip the degenerate
        // solve.
        let booked = if horizontal {
            curve.p0.x
        } else {
            curve.x_at_y(sample_y)
        };
        let booked = ((booked - grid.left) / TILE_SIZE).ceil() as isize - 1;

        // Count the crossing for every tile whose backdrop ray passes it:
        // counted iff booked strictly left of that tile's column. The span
        // test compares stored endpoint fields against the sample height,
        // half-open: a curve starting exactly at the sample height counts,
        // one ending there does not, so a shared contour endpoint on the
        // line counts exactly once.
        if !horizontal && curve.p0.y <= sample_y && sample_y < curve.p1.y {
            let column = booked.clamp(0, grid.columns as isize - 1) as usize;
            scratch.backdrops[grid.cell(row, column)] += delta;
        }

        // Monotonicity makes the curve's x-range over a y-slab exactly the
        // interval between the two boundary evaluations, so the trail is a
        // staircase of O(rows + columns) tiles rather than the bounding
        // box's rows * columns. No inclusion slack is needed: the trail
        // and the booking bucket the same row-top evaluation, so a tile
        // whose backdrop excludes a crossing always lists the curve that
        // owns it. (A crossing exactly on a column boundary books one
        // cell further left — outside the trail — which only means that
        // cell's run breaks on the backdrop-equality check in
        // `emit_tiles`; the crossing itself is backdrop-owned there and
        // curve-owned in the trail, on opposite sides of the boundary.) A
        // curve wrongly excluded from a tile it merely grazes loses only
        // an ulp-sized continuous sliver of area.
        let (xa, xb) = if horizontal {
            (curve.p0.x, curve.p1.x)
        } else {
            let ya = curve.p0.y.max(row_top);
            let yb = curve.p1.y.min(row_top + TILE_SIZE);
            (curve.x_at_y(ya), curve.x_at_y(yb))
        };
        let low = grid.column_of(xa.min(xb));
        let high = grid.column_of(xa.max(xb));
        for column in low..=high {
            // The downward-leg booking for this (tile, curve) pair: the
            // the curve crosses the tile's left-edge leg below the sample iff
            // it straddles that edge, extends below the sample height, and
            // at the sample height sits on the opposite side of the edge
            // from where it is heading. `booked < column` is exactly `x at
            // sample height <= corner.x` under the ceil-minus-one
            // bucketing — the `corner + 0⁺` limit again: a crossing
            // exactly on the edge is left of the perturbed leg. The
            // straddle test is half-open the same way: a curve whose
            // leftmost point only touches the edge is crossed by the
            // perturbed leg, one whose rightmost point touches it is not.
            // When the curve straddles the edge at all, the crossing
            // height is solved here, once, whether or not the leg counts
            // it: the shader's window split needs it either way.
            let corner_x = grid.left + column as f32 * TILE_SIZE;
            let straddles = corner_x >= min_x && corner_x < max_x;
            let leg_y = if straddles {
                curve.y_at_x(corner_x)
            } else {
                0.0
            };
            let crosses_leg = straddles
                && curve.sx != 0.0
                && curve.p1.y > sample_y
                && if curve.sx > 0.0 {
                    booked < column as isize
                } else {
                    booked >= column as isize
                };
            let cell = grid.cell(row, column);
            scratch.entries.push(ScratchEntry {
                entry: TileCurve {
                    curve: index
                        | if crosses_leg {
                            CURVE_DOWNWARD_LEG_FLAG
                        } else {
                            0
                        },
                    leg_y,
                },
                next: scratch.tile_heads[cell],
            });
            scratch.tile_heads[cell] = scratch.entries.len() as u32 - 1;
        }
    }
}

impl MonotoneCurve {
    /// Store an xy-monotone curve downward, recording which way the contour
    /// ran through it. Coordinates stay in logical pixels; snapping onto the
    /// winding lattice happens once, in [`Self::scaled`]. Returns `None` for
    /// degenerate curves.
    ///
    /// Horizontal curves compare equal here and so keep `sign = 1`. They must
    /// survive decomposition: they never cross a rightward route, but the
    /// shader's downward leg genuinely crosses them, and dropping them
    /// corrupts the winding of every tile whose left edge passes through one
    /// — the interior of every wide rectangle.
    fn from_curve(curve: QuadraticBezierSegment<f32>) -> Option<MonotoneCurve> {
        let from = point(curve.from.x, curve.from.y);
        let ctrl = point(curve.ctrl.x, curve.ctrl.y);
        let to = point(curve.to.x, curve.to.y);
        if from == to {
            return None;
        }
        let (p0, p1, sign) = if from.y > to.y {
            (to, from, -1.0)
        } else {
            (from, to, 1.0)
        };
        // `for_each_monotonic` clamps sub-curve control points, but keep
        // the guarantee local; everything downstream depends on the stored
        // curve being actually monotone.
        let ctrl = point(
            ctrl.x.clamp(p0.x.min(p1.x), p0.x.max(p1.x)),
            ctrl.y.clamp(p0.y, p1.y),
        );
        Some(MonotoneCurve { p0, ctrl, p1, sign })
    }

    /// Store a line downward with its control point at the midpoint of its
    /// endpoints. Returns `None` for degenerate lines.
    ///
    /// The midpoint is computed with the same expression [`Self::scaled`]
    /// tests for, which is how a line stays recognizable as a line at scale
    /// time, where its quadratic coefficients are set to exact zeros. The
    /// control point of a line carries no boundary geometry, so the
    /// convention costs nothing.
    pub(crate) fn from_line(from: Point<f32>, to: Point<f32>) -> Option<MonotoneCurve> {
        if from == to {
            return None;
        }
        let (p0, p1, sign) = if from.y > to.y {
            (to, from, -1.0)
        } else {
            (from, to, 1.0)
        };
        let ctrl = point(0.5 * (p0.x + p1.x), 0.5 * (p0.y + p1.y));
        Some(MonotoneCurve { p0, ctrl, p1, sign })
    }

    /// Scale into device pixels, snap onto the winding lattice, and
    /// precompute the polynomial coefficients.
    ///
    /// This is the one place geometry is snapped, and it is snapped in
    /// device space, where every discrete decision — backdrop scatter,
    /// leg booking, the shader's gates — is made. Boundaries may land
    /// exactly on tile corners and sample lines (integer-coordinate
    /// rectangles do constantly); that is fine, because every discrete
    /// gate on both sides is half-open in a consistent direction, so a
    /// tie lands on exactly one side. The lattice is not there to avoid
    /// ties — it conditions the geometry: path construction can emit
    /// slivers thinner than f32's spacing at screen magnitudes (lyon has
    /// returned an arc endpoint at `y = 255.00002` for a requested `255`,
    /// making a 96-pixel-wide edge with a 2e-5 y-extent), and counting
    /// crossings inside such a sliver needs more precision than the
    /// numbers themselves carry, so no convention can get it right. On
    /// the lattice every coordinate is exactly representable and at least
    /// 1/256 apart — comfortably above f32's 1.5e-5 spacing near 255 — so
    /// near-degenerate slivers collapse to the exactly-horizontal or
    /// exactly-vertical cases the conventions handle, and boundary
    /// comparisons are equalities on exact values, not ulp lotteries.
    fn scaled(&self, scale: f32) -> PathCurve {
        let p0 = point(snap(self.p0.x * scale), snap(self.p0.y * scale));
        let p1 = point(snap(self.p1.x * scale), snap(self.p1.y * scale));
        // Snapping can collapse a short curve to exactly horizontal;
        // restore the convention that horizontal curves are stored
        // unflipped with sign +1 (see [`Self::from_curve`]).
        let (p0, p1, sign) = if p0.y == p1.y && self.sign < 0.0 {
            (p1, p0, 1.0)
        } else {
            (p0, p1, self.sign)
        };
        // A line (marked by [`Self::from_line`]'s midpoint control point)
        // gets its coefficients set rather than derived: the quadratic
        // terms are exactly zero — no rounding can say otherwise — and the
        // linear terms are endpoint differences, exact on the lattice.
        // Every solve against a line therefore takes
        // `monotone_quadratic_root`'s linear branch: no square root, no
        // near-singular leading coefficient.
        let is_line = self.ctrl.x == 0.5 * (self.p0.x + self.p1.x)
            && self.ctrl.y == 0.5 * (self.p0.y + self.p1.y);
        let (ax, bx, ay, by) = if is_line {
            (0.0, p1.x - p0.x, 0.0, p1.y - p0.y)
        } else {
            // Snapping moves each coordinate independently, so re-clamp
            // the control point into the endpoint box before deriving;
            // everything downstream depends on the curve being actually
            // monotone.
            let ctrl = point(
                snap(self.ctrl.x * scale).clamp(p0.x.min(p1.x), p0.x.max(p1.x)),
                snap(self.ctrl.y * scale).clamp(p0.y.min(p1.y), p0.y.max(p1.y)),
            );
            (
                p0.x - 2.0 * ctrl.x + p1.x,
                2.0 * (ctrl.x - p0.x),
                p0.y - 2.0 * ctrl.y + p1.y,
                2.0 * (ctrl.y - p0.y),
            )
        };
        let dx = p1.x - p0.x;
        PathCurve {
            p0,
            p1,
            ax,
            bx,
            ay,
            by,
            sign,
            sx: if dx == 0.0 { 0.0 } else { dx.signum() },
        }
    }
}

impl PathCurve {
    /// The x coordinate at which this downward-monotone curve reaches `y`.
    /// Monotonicity guarantees a single in-range root.
    fn x_at_y(&self, y: f32) -> f32 {
        let t = monotone_quadratic_root(self.ay, self.by, self.p0.y - y);
        (self.ax * t + self.bx) * t + self.p0.x
    }

    /// The y coordinate at which this x-monotone curve reaches `x`. Only
    /// meaningful when `x` lies within the curve's x-range.
    fn y_at_x(&self, x: f32) -> f32 {
        let t = monotone_quadratic_root(self.ax, self.bx, self.p0.x - x);
        (self.ay * t + self.by) * t + self.p0.y
    }
}

/// Stable quadratic solve for `a t^2 + b t + c = 0`, returning the root
/// within `[0, 1]`. Deliberately identical to the shader's
/// `monotone_quadratic_root` so that the CPU's backdrop and the shader's
/// route legs agree on where a curve crosses a line.
fn monotone_quadratic_root(a: f32, b: f32, c: f32) -> f32 {
    if a.abs() < 1e-6 {
        return if b.abs() < 1e-12 {
            0.0
        } else {
            (-c / b).clamp(0.0, 1.0)
        };
    }
    let sqrt_discriminant = (b * b - 4.0 * a * c).max(0.0).sqrt();
    let q = if b >= 0.0 {
        -0.5 * (b + sqrt_discriminant)
    } else {
        -0.5 * (b - sqrt_discriminant)
    };
    let root0 = q / a;
    let root1 = if q.abs() > 1e-12 { c / q } else { root0 };
    let root = if (-1e-4..=1.0001).contains(&root0) {
        root0
    } else {
        root1
    };
    root.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PathBuilder, Pixels, px, size};
    use lyon::path::FillRule;

    /// Rasterize every shape from the `painting` example at scale 1.0 (the
    /// integer-lattice case that puts crossings exactly on tile lines) and
    /// compare the emulated shader output against a brute-force winding
    /// reference at every pixel that is unambiguously inside or outside.
    #[test]
    fn painting_example_shapes_match_reference() {
        let mut failures = Vec::new();
        failures.extend(check_path("rectangle", rectangle()));
        failures.extend(check_path("diamond", diamond()));
        failures.extend(check_path("star", star()));
        failures.extend(check_path("dome", dome()));
        failures.extend(check_path("bolt", bolt()));
        failures.extend(check_path("circle", circle(530.0)));
        failures.extend(check_path("circle2", circle(570.0)));
        let pie = [
            ("pie blue", (871.0, 255.0), (747.0, 163.0)),
            ("pie red", (747.0, 163.0), (679.0, 263.0)),
            ("pie blue2", (679.0, 263.0), (754.0, 349.0)),
            ("pie green", (754.0, 349.0), (854.0, 310.0)),
            ("pie yellow", (854.0, 310.0), (871.0, 255.0)),
        ];
        for (name, start, end) in pie {
            failures.extend(check_path(name, pie_wedge(start, end)));
        }
        assert!(
            failures.is_empty(),
            "{} wrong pixels:\n{}",
            failures.len(),
            failures[..failures.len().min(60)].join("\n"),
        );
    }

    /// Interior tiles merge into one instance per run: after binning, no
    /// row may contain two horizontally adjacent curveless tiles.
    #[test]
    fn interior_tiles_merge_into_runs() {
        let device = bin_path(&rectangle(), FillRule::NonZero, huge_mask(), 4.0);
        // A 320x320 rectangle spans many tile columns at any tile size;
        // without merging, its interior would be full of horizontally
        // adjacent curveless instances, so their absence is the invariant
        // (and stays true across `TILE_SIZE` sweeps, unlike any count
        // bound).
        assert!(
            device.tiles.iter().any(|tile| tile.curve_count == 0),
            "expected at least one merged interior run",
        );
        for pair in device.tiles.windows(2) {
            let (left, right) = (&pair[0], &pair[1]);
            let (left_rect, right_rect) = (tile_rect(left), tile_rect(right));
            let adjacent = left_rect.origin.y == right_rect.origin.y
                && left_rect.origin.x.0 + left_rect.size.width.0 == right_rect.origin.x.0;
            assert!(
                !(adjacent && left.curve_count == 0 && right.curve_count == 0),
                "unmerged curveless neighbors at ({}, {})",
                left_rect.origin.x.0,
                left_rect.origin.y.0,
            );
        }
    }

    /// Logical `410/256` is a lattice value that scale 1.25 carries exactly
    /// onto a tile sample point (device `2 + 1/512`). Before device-space
    /// re-snapping, the CPU backdrop and the shader's leg gates each assumed
    /// the other owned that crossing, and the rectangle lost its first
    /// interior tile. The top edge sits on the same value to cover the row
    /// analog.
    #[test]
    fn fractional_scale_sample_collision() {
        let edge = px(410.0 / 256.0);
        let mut builder = PathBuilder::fill();
        builder.move_to(point(edge, edge));
        builder.line_to(point(px(60.0), edge));
        builder.line_to(point(px(60.0), px(40.0)));
        builder.line_to(point(edge, px(40.0)));
        builder.close();
        let failures = check_path("collision rectangle", builder.build().unwrap());
        assert!(
            failures.is_empty(),
            "{} wrong pixels:\n{}",
            failures.len(),
            failures[..failures.len().min(60)].join("\n"),
        );
    }

    /// A rectangle whose every edge lies exactly on tile boundaries, so
    /// crossings sit exactly on backdrop sample points and rows of samples
    /// — the ubiquitous tie case (any integer-coordinate rectangle at
    /// integer scale). The half-open conventions must give every tie to
    /// exactly one side; a gap or double-count loses or doubles a whole
    /// winding step across a tile.
    #[test]
    fn tile_aligned_edges_land_on_samples() {
        for (left, top, right, bottom) in [
            (0.0, 0.0, 32.0, 32.0),
            (16.0, 16.0, 64.0, 48.0),
            (-32.0, -16.0, 16.0, 16.0),
        ] {
            let mut builder = PathBuilder::fill();
            builder.move_to(point(px(left), px(top)));
            builder.line_to(point(px(right), px(top)));
            builder.line_to(point(px(right), px(bottom)));
            builder.line_to(point(px(left), px(bottom)));
            builder.close();
            let failures = check_path(
                &format!("aligned rectangle ({left}, {top})"),
                builder.build().unwrap(),
            );
            assert!(
                failures.is_empty(),
                "{} wrong pixels:\n{}",
                failures.len(),
                failures[..failures.len().min(60)].join("\n"),
            );
        }
    }

    /// A path whose full extent would need more than `MAX_TILES` tiles must
    /// still render the portion inside a small mask: the grid is sized by
    /// the mask intersection, with out-of-view crossings folded into the
    /// margin column.
    #[test]
    fn huge_path_clipped_to_small_mask() {
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(-20000.0), px(-20000.0)));
        builder.line_to(point(px(20000.0), px(-20000.0)));
        builder.line_to(point(px(20000.0), px(20000.0)));
        builder.line_to(point(px(-20000.0), px(20000.0)));
        builder.close();
        let path = builder.build().unwrap();
        let mask = Bounds {
            origin: point(px(300.0), px(200.0)),
            size: size(px(120.0), px(90.0)),
        };
        let device = bin_path(&path, FillRule::NonZero, mask, 1.0);
        assert!(!device.tiles.is_empty(), "visible portion must emit tiles");
        // Tiles wholly outside the mask are never emitted; partially
        // masked ones are, and the content-mask clip distances trim them.
        for tile in &device.tiles {
            let rect = tile_rect(tile);
            assert!(
                rect.origin.x.0 < 420.0
                    && rect.origin.y.0 < 290.0
                    && rect.origin.x.0 + rect.size.width.0 > 300.0
                    && rect.origin.y.0 + rect.size.height.0 > 200.0,
                "tile invisible under the mask: {rect:?}",
            );
        }
        // Every masked pixel is deep inside the rectangle; pixels just
        // outside the mask are clipped even where a tile overhangs it.
        for pixel_y in [200, 245, 289] {
            for pixel_x in [300, 360, 419] {
                let coverage = emulated_coverage(&device, pixel_x, pixel_y);
                assert!(
                    (coverage - 1.0).abs() < 1e-3,
                    "pixel ({pixel_x}, {pixel_y}) coverage {coverage}",
                );
            }
        }
        for (pixel_x, pixel_y) in [(299, 245), (360, 199), (420, 245), (360, 290)] {
            let coverage = emulated_coverage(&device, pixel_x, pixel_y);
            assert!(
                coverage == 0.0,
                "pixel ({pixel_x}, {pixel_y}) outside the mask has coverage {coverage}",
            );
        }
    }

    /// A mask so large it never clips the test shapes.
    fn huge_mask() -> Bounds<Pixels> {
        Bounds {
            origin: point(px(-16384.0), px(-16384.0)),
            size: size(px(65536.0), px(65536.0)),
        }
    }

    /// Paint (and thereby bin) a built path under the given fill rule and
    /// content mask — the wiring `Window::paint_path` performs.
    fn bin_path(
        path: &crate::Path,
        fill_rule: FillRule,
        mask: Bounds<Pixels>,
        scale: f32,
    ) -> PaintedPath {
        let mut path = path.clone();
        path.fill_rule = fill_rule;
        path.painted(scale, ContentMask { bounds: mask }, Background::default())
    }

    fn check_path(name: &str, path: crate::Path) -> Vec<String> {
        let mut failures = Vec::new();
        // Every shape runs under both fill rules: the even-odd fold is a
        // separate branch of the shader's coverage mapping, and the
        // self-intersecting shapes (the star) give it a non-trivial
        // interior to disagree about.
        for fill_rule in [FillRule::NonZero, FillRule::EvenOdd] {
            for scale in [1.0f32, 1.25, 1.5, 2.0] {
                let device = bin_path(&path, fill_rule, huge_mask(), scale);
                // The device-space curves: exactly the buffer the GPU would
                // read.
                let curves = &device.curves;
                if curves.is_empty() {
                    continue;
                }

                let mut min = point(f32::INFINITY, f32::INFINITY);
                let mut max = point(f32::NEG_INFINITY, f32::NEG_INFINITY);
                for curve in curves {
                    min.x = min.x.min(curve.p0.x.min(curve.p1.x));
                    max.x = max.x.max(curve.p0.x.max(curve.p1.x));
                    min.y = min.y.min(curve.p0.y);
                    max.y = max.y.max(curve.p1.y);
                }

                for pixel_y in (min.y.floor() as i32 - 2)..=(max.y.ceil() as i32 + 2) {
                    for pixel_x in (min.x.floor() as i32 - 2)..=(max.x.ceil() as i32 + 2) {
                        let actual = emulated_coverage(&device, pixel_x, pixel_y);
                        // Fast path: a pixel matching its single center sample is
                        // interior or exterior; only boundary pixels need the
                        // supersampled reference.
                        let center_winding =
                            reference_winding(curves, pixel_x as f32 + 0.5, pixel_y as f32 + 0.5);
                        let center_filled = match fill_rule {
                            FillRule::EvenOdd => center_winding.rem_euclid(2) != 0,
                            FillRule::NonZero => center_winding != 0,
                        };
                        if (actual - center_filled as i32 as f32).abs() <= 0.35 {
                            continue;
                        }
                        let expected = reference_coverage(curves, fill_rule, pixel_x, pixel_y);
                        // 8x8 supersampling quantizes coverage in steps of 1/64
                        // and point sampling shifts an edge by up to half a
                        // sample step, so allow a generous band; real failures
                        // are whole-pixel.
                        if (actual - expected).abs() > 0.35 {
                            failures.push(format!(
                                "{name} ({fill_rule:?}) at scale {scale}: \
                            pixel ({pixel_x}, {pixel_y}) \
                            expected {expected} actual {actual}\n{}",
                                describe_pixel(&device, pixel_x, pixel_y),
                            ));
                        }
                    }
                }
            }
        }
        failures
    }

    /// The rectangle a tile rasterizes, derived exactly as the vertex
    /// shader derives it: `run` grid cells rightward from the corner, one
    /// cell tall.
    fn tile_rect(tile: &PathTile) -> Bounds<ScaledPixels> {
        Bounds {
            origin: point(ScaledPixels(tile.corner.x), ScaledPixels(tile.corner.y)),
            size: size(
                ScaledPixels(TILE_SIZE * tile.run as f32),
                ScaledPixels(TILE_SIZE),
            ),
        }
    }

    /// Whether a fragment at the pixel's center survives rasterization of
    /// the tile: inside the derived rectangle and not culled by the
    /// content-mask clip distances.
    fn tile_rasterizes(path: &PaintedPath, tile: &PathTile, center_x: f32, center_y: f32) -> bool {
        let rect = tile_rect(tile);
        let mask = path.paint.content_mask.bounds;
        center_x >= rect.origin.x.0
            && center_x < rect.origin.x.0 + rect.size.width.0
            && center_y >= rect.origin.y.0
            && center_y < rect.origin.y.0 + rect.size.height.0
            && center_x >= mask.origin.x.0
            && center_x < mask.right().0
            && center_y >= mask.origin.y.0
            && center_y < mask.bottom().0
    }

    /// Every term of the emulated fragment computation for one pixel, for
    /// failure diagnostics.
    fn describe_pixel(path: &PaintedPath, pixel_x: i32, pixel_y: i32) -> String {
        let center_x = pixel_x as f32 + 0.5;
        let center_y = pixel_y as f32 + 0.5;
        let Some(tile) = path
            .tiles
            .iter()
            .find(|tile| tile_rasterizes(path, tile, center_x, center_y))
        else {
            return "  no tile".to_string();
        };
        let pixel = point(pixel_x as f32, pixel_y as f32);
        let mut description = format!(
            "  tile corner ({}, {}) backdrop {} curves {}\n",
            tile.corner.x, tile.corner.y, tile.backdrop, tile.curve_count
        );
        for i in tile.curve_start..tile.curve_start + tile.curve_count {
            let entry = path.tile_curves[i as usize];
            let curve = &path.curves[(entry.curve & !CURVE_DOWNWARD_LEG_FLAG) as usize];
            let crosses_downward_leg = entry.curve & CURVE_DOWNWARD_LEG_FLAG != 0;
            let contribution = emulated_curve_winding(
                curve,
                tile.corner,
                pixel,
                crosses_downward_leg,
                entry.leg_y,
            );
            description.push_str(&format!(
                "  curve p0 ({}, {}) p1 ({}, {}) a ({}, {}) b ({}, {}) \
                sign {} sx {} leg {} -> {}\n",
                curve.p0.x,
                curve.p0.y,
                curve.p1.x,
                curve.p1.y,
                curve.ax,
                curve.ay,
                curve.bx,
                curve.by,
                curve.sign,
                curve.sx,
                crosses_downward_leg,
                contribution,
            ));
        }
        description
    }

    /// Ground-truth box-filtered coverage of a pixel: 8x8 supersampled
    /// point-in-path tests, each a leftward ray cast over the curves.
    fn reference_coverage(
        curves: &[PathCurve],
        fill_rule: FillRule,
        pixel_x: i32,
        pixel_y: i32,
    ) -> f32 {
        let mut inside = 0;
        for sample_y in 0..8 {
            for sample_x in 0..8 {
                let x = pixel_x as f32 + (sample_x as f32 + 0.5) / 8.0;
                let y = pixel_y as f32 + (sample_y as f32 + 0.5) / 8.0;
                let winding = reference_winding(curves, x, y);
                let filled = match fill_rule {
                    FillRule::EvenOdd => winding.rem_euclid(2) != 0,
                    FillRule::NonZero => winding != 0,
                };
                inside += filled as i32;
            }
        }
        inside as f32 / 64.0
    }

    /// Winding number at a point: a leftward ray cast over the curves,
    /// counting row crossings half-open in y exactly like the backdrop.
    fn reference_winding(curves: &[PathCurve], x: f32, y: f32) -> i32 {
        let mut winding = 0;
        for curve in curves {
            if curve.p0.y <= y && y < curve.p1.y && curve.x_at_y(y) < x {
                winding += if curve.sign < 0.0 { -1 } else { 1 };
            }
        }
        winding
    }

    /// The fragment shader, transcribed: find the tile whose quad rasterizes
    /// the pixel and run `path_tile_fragment`'s winding loop and fill-rule
    /// fold. Pixels no tile covers have coverage zero.
    fn emulated_coverage(path: &PaintedPath, pixel_x: i32, pixel_y: i32) -> f32 {
        let center_x = pixel_x as f32 + 0.5;
        let center_y = pixel_y as f32 + 0.5;
        let Some(tile) = path
            .tiles
            .iter()
            .find(|tile| tile_rasterizes(path, tile, center_x, center_y))
        else {
            return 0.0;
        };
        let pixel = point(pixel_x as f32, pixel_y as f32);
        let mut winding = tile.backdrop as f32;
        for i in tile.curve_start..tile.curve_start + tile.curve_count {
            let entry = path.tile_curves[i as usize];
            let curve = &path.curves[(entry.curve & !CURVE_DOWNWARD_LEG_FLAG) as usize];
            // The shader's sorted early-out, mirrored.
            if curve.p0.x.min(curve.p1.x) >= pixel.x + 1.0 {
                break;
            }
            let crosses_downward_leg = entry.curve & CURVE_DOWNWARD_LEG_FLAG != 0;
            winding += emulated_curve_winding(
                curve,
                tile.corner,
                pixel,
                crosses_downward_leg,
                entry.leg_y,
            );
        }
        if path.paint.even_odd.as_bool() {
            (winding - 2.0 * (winding * 0.5).round()).abs()
        } else {
            winding.abs().min(1.0)
        }
    }

    /// `curve_winding` from shaders.hlsl, transcribed. HLSL `clamp`/`saturate`
    /// are `min(max(..))`, which never panics on an inverted range, so the
    /// transcription uses `max().min()` rather than Rust's checked `clamp`.
    fn emulated_curve_winding(
        curve: &PathCurve,
        corner: Point<f32>,
        pixel: Point<f32>,
        crosses_downward_leg: bool,
        leg_y: f32,
    ) -> f32 {
        let ax = curve.ax;
        let bx = curve.bx;
        let ay = curve.ay;
        let by = curve.by;
        let mut winding = 0.0;

        let mut ya = pixel.y.max(curve.p0.y);
        let mut yb = (pixel.y + 1.0).min(curve.p1.y);
        if yb > ya {
            let ta = monotone_quadratic_root(ay, by, curve.p0.y - ya);
            let tb = monotone_quadratic_root(ay, by, curve.p0.y - yb);
            let xa = (ax * ta + bx) * ta + curve.p0.x;
            let xb = (ax * tb + bx) * tb + curve.p0.x;

            if xa.min(xb) < pixel.x + 1.0 {
                let mut live = true;
                if xa.max(xb) <= corner.x {
                    live = false;
                } else if xa.min(xb) < corner.x {
                    let y_c = leg_y.max(ya).min(yb);
                    if xa < corner.x {
                        ya = y_c;
                    } else {
                        yb = y_c;
                    }
                    live = yb > ya;
                }
                if live {
                    if xa.max(xb) <= pixel.x {
                        winding += curve.sign * (yb - ya);
                    } else {
                        winding += curve.sign
                            * ((yb - ya)
                                - emulated_curve_column_area(
                                    ax,
                                    bx,
                                    curve.p0.x - pixel.x,
                                    ay,
                                    by,
                                    curve.p0.y,
                                    ta,
                                    tb,
                                    xa - pixel.x,
                                    xb - pixel.x,
                                ));
                    }
                }
            }
        }

        if crosses_downward_leg {
            winding -= curve.sign * curve.sx * (pixel.y + 1.0 - leg_y).clamp(0.0, 1.0);
        }

        winding
    }

    /// `curve_column_area` from shaders.hlsl, transcribed. The parameter
    /// list mirrors the HLSL signature one-to-one on purpose; bundling them
    /// into a struct would make the lockstep comparison harder to eyeball.
    #[allow(clippy::too_many_arguments)]
    fn emulated_curve_column_area(
        ax: f32,
        bx: f32,
        cx: f32,
        ay: f32,
        by: f32,
        p0y: f32,
        ta: f32,
        tb: f32,
        xa: f32,
        xb: f32,
    ) -> f32 {
        let integral = |t: f32| {
            let c3 = 0.5 * ax * ay;
            let c2 = (ax * by + 2.0 * bx * ay) / 3.0;
            let c1 = 0.5 * (bx * by + 2.0 * cx * ay);
            let c0 = cx * by;
            (((c3 * t + c2) * t + c1) * t + c0) * t
        };

        if xb >= xa {
            let s0 = if xa >= 0.0 {
                ta
            } else if xb <= 0.0 {
                tb
            } else {
                monotone_quadratic_root(ax, bx, cx).max(ta).min(tb)
            };
            let s1 = if xb <= 1.0 {
                tb
            } else if xa >= 1.0 {
                ta
            } else {
                monotone_quadratic_root(ax, bx, cx - 1.0).max(ta).min(tb)
            };
            let y_s1 = (ay * s1 + by) * s1 + p0y;
            let y_tb = (ay * tb + by) * tb + p0y;
            (y_tb - y_s1) + integral(s1) - integral(s0)
        } else {
            let s1 = if xa <= 1.0 {
                ta
            } else if xb >= 1.0 {
                tb
            } else {
                monotone_quadratic_root(ax, bx, cx - 1.0).max(ta).min(tb)
            };
            let s0 = if xb >= 0.0 {
                tb
            } else if xa <= 0.0 {
                ta
            } else {
                monotone_quadratic_root(ax, bx, cx).max(ta).min(tb)
            };
            let y_s1 = (ay * s1 + by) * s1 + p0y;
            let y_ta = (ay * ta + by) * ta + p0y;
            (y_s1 - y_ta) + integral(s0) - integral(s1)
        }
    }

    /// Axis-aligned integer rectangle: every edge lies exactly on the
    /// geometry lattice, the left edge on the grid's leftmost line.
    fn rectangle() -> crate::Path {
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(450.), px(60.)));
        builder.line_to(point(px(530.), px(60.)));
        builder.line_to(point(px(530.), px(140.)));
        builder.line_to(point(px(450.), px(140.)));
        builder.close();
        builder.build().unwrap()
    }

    /// 45-degree edges through integer vertices: the diagonal resonance case
    /// for any sample-offset convention.
    fn diamond() -> crate::Path {
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(100.), px(68.)));
        builder.line_to(point(px(132.), px(100.)));
        builder.line_to(point(px(100.), px(132.)));
        builder.line_to(point(px(68.), px(100.)));
        builder.close();
        builder.build().unwrap()
    }

    fn star() -> crate::Path {
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(350.), px(200.)));
        for (x, y) in [
            (370., 260.),
            (430., 260.),
            (380., 300.),
            (400., 360.),
            (350., 320.),
            (300., 360.),
            (320., 300.),
            (270., 260.),
            (330., 260.),
            (350., 200.),
        ] {
            builder.line_to(point(px(x), px(y)));
        }
        builder.build().unwrap()
    }

    fn dome() -> crate::Path {
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(450.), px(280.)));
        builder.curve_to(point(px(530.), px(230.)), point(px(450.), px(230.)));
        builder.line_to(point(px(570.), px(230.)));
        builder.curve_to(point(px(650.), px(280.)), point(px(650.), px(230.)));
        builder.line_to(point(px(450.), px(280.)));
        builder.build().unwrap()
    }

    fn bolt() -> crate::Path {
        let mut builder = PathBuilder::fill();
        builder.add_polygon(
            &[
                point(px(150.), px(300.)),
                point(px(200.), px(225.)),
                point(px(200.), px(275.)),
                point(px(250.), px(200.)),
            ],
            false,
        );
        builder.build().unwrap()
    }

    fn circle(center_x: f32) -> crate::Path {
        let mut builder = PathBuilder::fill();
        let radius = px(30.);
        let center = point(px(center_x), px(85.));
        builder.move_to(point(center.x + radius, center.y));
        builder.arc_to(
            point(radius, radius),
            px(0.),
            false,
            false,
            point(center.x - radius, center.y),
        );
        builder.arc_to(
            point(radius, radius),
            px(0.),
            false,
            false,
            point(center.x + radius, center.y),
        );
        builder.close();
        builder.build().unwrap()
    }

    fn pie_wedge(start: (f32, f32), end: (f32, f32)) -> crate::Path {
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(start.0), px(start.1)));
        builder.arc_to(
            point(px(96.), px(96.)),
            px(0.),
            false,
            false,
            point(px(end.0), px(end.1)),
        );
        builder.line_to(point(px(775.), px(255.)));
        builder.close();
        builder.build().unwrap()
    }
}
