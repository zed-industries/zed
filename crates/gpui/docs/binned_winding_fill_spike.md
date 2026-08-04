# Spike directions: binned per-pixel winding fills

Directions for an agent implementing the replacement for the trapezoid
sweep. Self-contained: you do not need the conversation that produced this,
and you do not need to read all of `trapezoid_path_rendering.md` (it is the
design history; leave it in place, untouched).

## Mission

Replace the trapezoid-decomposition fill pipeline (a CPU sweep-line
algorithm) with **CPU-binned, GPU-evaluated per-pixel winding**. The CPU
never compares two curve pieces against each other — it only buckets pieces
into a fixed grid and counts axis-aligned crossings. The GPU fragment
shader computes the exact winding-derived coverage of each pixel with a
short, uniform-bounded loop over the pieces in the pixel's bin, reusing the
exact quadratic area math already in the shader.

Deletion comes **first**. The sweep-line code (crossing resolution, chains,
spans, row snapping, corner patches) must be gone before any new code is
written. Do not preserve it "just in case", do not keep it behind a cfg,
do not port pieces of its topology logic into the new design.

## Ground rules

1. **DirectX only.** The target is `gpui` + `gpui_windows` compiling and
   running on Windows. Metal, blade/wgpu, and any other backend may be left
   broken; do not spend any effort on them, not even stubs, beyond what the
   Windows build itself requires. (`gpui`'s mac/linux platform modules are
   not compiled on Windows targets, so scene-struct changes will not break
   the Windows build.)
2. **No tests.** Do not write unit tests, do not port the old test module
   or the `race_against_lyon` harness. Validation is visual: run the
   examples and Zed itself.
3. **Strokes stay broken.** `PathBuilder` built with `PathStyle::Stroke`
   should return an empty path that renders nothing. The eventual stroke
   design is SDF capsule instances (see `trapezoid_path_rendering.md`
   round two), *not* the winding pipeline — do not write interim code that
   routes strokes through fills; it would be thrown away.
4. Non-negotiable rendering constraints (do not trade these away for any
   simplification): no MSAA, no intermediate textures, no extra render
   passes, single blend per pixel per path, exact quadratic curves
   evaluated in the shader (no flattening of rendered geometry).

## Current data model — supersedes the phase names below

The phases below are the original directions and keep their original
vocabulary (`PathDecomposition`, `PathBins`, `PathBin`, `PathPiece`,
"bins"). The shipped code has since been reorganized around two concrete,
non-generic path types — the generic `Path<Pixels>`/`Path<ScaledPixels>`
is gone — and renamed bin → **tile**, piece → **curve** ("bin" survives
as the verb):

- `Path` is pure logical-space geometry: each segment's xy-monotone
  decomposition (grown *inline* as each segment is pushed — no `Option`,
  no `Arc`, no `ensure_decomposition`, no staleness; `PathDecomposition`
  no longer exists — and no retained `segments` vec either: the
  decomposition *is* the geometry, and the two `is_empty` guards its
  deletion orphaned reduce to `current != start`), `bounds`, and
  `fill_rule`. Paint-time facts are not fields; they arrive as arguments.
- `Path::painted(scale_factor, content_mask, color)` bins once, in
  device space, and returns a `PaintedPath`: `order`, an inline
  `paint: PathPaint` row (bounds, mask, color, `even_odd`), and the tile
  payload (`tiles`/`curves`/`tile_curves`, all public fields). `painted`
  is the only constructor, so a `PaintedPath` is always fully binned.
  Scene replay clones it tiles-and-all; cached elements skip re-binning
  entirely.
- A `PathTile` is 28 bytes: `paint`, `curve_start`, `curve_count`,
  `backdrop`, `corner`, `run`. Its rectangle is **derived in the vertex
  shader** — `run` grid cells rightward from `floor(corner)`, one cell
  tall — and the content-mask clip distances the vertex shader already
  emits trim whatever the mask cuts off; binning only skips tiles wholly
  outside the mask. Each tile's curve list is **sorted by leftmost x**
  at emission so the fragment loop can stop at the first curve entirely
  right of its pixel's column (Slug's sorted-band early-out, mirrored
  for left-integrated winding; safe because such curves fail the
  rightward-leg gate and can't carry a downward-leg booking).
- `Scene.paths` is a plain `Vec<PaintedPath>`, handled exactly like
  `quads`: push on insert, sort by order in `finish`, batch by order. No
  tile-specific logic exists outside `path_winding.rs`, `Scene::finish`,
  and the renderer's four verbatim buffer uploads.
- Tile indices are path-local **only inside `PaintedPath`**.
  `Scene::finish` runs `flatten_paths`, the one place the two index
  domains meet: it concatenates every path's paint row, curves, tiles,
  and entries into flat scene-owned vecs (`path_paints`, `path_curves`,
  `path_tiles`, `path_tile_curves`), globalizing indices as it copies
  (paths with no tiles contribute nothing). Renderers upload those vecs
  verbatim — the `monochrome_sprites` four-line pattern, no per-path
  loops, no stamping, no `curve_base`/`tile_curve_base` fields, no
  shader-side rebasing. `PrimitiveBatch::Paths { range, tile_range }`
  carries the instance range directly; the batch iterator accumulates
  it from `tiles.len()` as it consumes paths, so `draw_paths` is a plain
  `draw_range` like quads.
- Binning scratch (backdrop grid, tile heads, entry arena) is a
  thread-local inside `path_winding.rs`, reused across paths and frames.

Name map for reading the phases: `PathDecomposition::compute` → inline
`decompose_quadratic`; `MonotonePiece` → `MonotoneCurve`; `PathPiece` →
`PathCurve`; `PathBin` → `PathTile` (no `order`/`even_odd` fields);
`PathPieceEntry` → `TileCurve`; `PathBins` → gone; shader entry points
`path_bin_*` → `path_tile_*`, `piece_winding` → `curve_winding`,
`piece_column_area` → `curve_column_area`.

Public-API migration (for the merge changelog): `Path<Pixels>` → `Path`
(delete the parameter); `Path::scale` → `Path::painted(scale, mask,
color)`; the built path's `color`/`content_mask`/`order` fields are gone
(they were overwritten by `Window::paint_path` anyway — pass color to
`paint_path`, which is unchanged for callers); `Primitive::Path` and
`Scene.paths` now carry `PaintedPath`; `PathQuadratic` and
`Path.segments` are gone (the built path exposes no geometry — build it,
paint it).

## Phase 0 — demolition

Salvage these before deleting anything (copy them out; they are reused):

- `monotone_pieces()` — `crates/gpui/src/path_fill.rs` (~L175). The
  monotone split via lyon. This is the only CPU geometry that survives.
- `t_at_y()` — `crates/gpui/src/path_snap.rs` (~L280). Stable
  monotone-quadratic root solve; the binner needs the same evaluation.
- From `crates/gpui_windows/src/shaders.hlsl`: `monotone_quadratic_root`,
  `piece_column_area`, and its helper `coverage_integral`. Keep these three
  functions; delete the rest of the "Path Trapezoids" section.

Then delete:

- `crates/gpui/src/path_fill.rs` — entire file. This is the sweep:
  `split_crossings`, `segment_crossing`, `apply_splits`, `merge_coincident`,
  `sweep`, `Span`, `match_spans`, `Chain`, `Run`, `GridPoint`, `orient`,
  `crossing_point`, `strictly_inside`, `div_round`, the grid constants, and
  the whole `mod tests`.
- `crates/gpui/src/path_snap.rs` — entire file: `TrapezoidSnapper`,
  `PathTrapezoid`, `TrapezoidEdge`, `TrapezoidPaint`, `make_edge`,
  `push_cut`.
- `crates/gpui/src/path_trapezoids.rs` — entire file (the retired
  `i_overlay` referee, `#[cfg(test)]`-only).
- `crates/gpui/src/path_stroke.rs` — entire file (`StrokeSynthesizer`
  produces sweep-shaped output; the future stroke design won't reuse it).

And clean up the references:

- `crates/gpui/Cargo.toml`: remove the `i_overlay` and `kurbo`
  dependencies.
- `crates/gpui/src/gpui.rs`: remove the `path_fill` / `path_snap` /
  `path_stroke` / `path_trapezoids` module declarations and their
  re-exports; declare the new module (Phase 1).
- `crates/gpui/src/path_builder.rs`: delete `synthesize_stroke`,
  `to_kurbo_point`, `from_kurbo_point`, `STROKE_TOLERANCE`, and the kurbo
  imports. `PathStyle::Stroke` in `build()` returns
  `Ok(Path::new(Point::default()))` with a comment that strokes are
  pending their own instance design. Keep `fill_outline` unchanged.
- `crates/gpui/src/scene.rs`: `Path` keeps `segments`, `fill_rule`,
  `decomposition`, `ensure_decomposition` (its body shrinks in Phase 1);
  delete `set_decomposition` (stroke-only). `Scene`'s `path_trapezoids`
  vector and `trapezoid_snapper` field are replaced in Phase 1.
- `crates/gpui_windows/src/directx_renderer.rs` and `shaders.hlsl`: the
  trapezoid pipeline is replaced in Phase 3.

**Checkpoint:** `cargo check -p gpui -p gpui_windows` passes with paths
simply not rendering (e.g. `Scene::insert_primitive` for `Primitive::Path`
temporarily emits nothing). Run one example to confirm no crash. Commit
mentally here: the sweep is gone.

## Phase 1 — build-time decomposition (scale-independent, cached)

New module, e.g. `crates/gpui/src/path_winding.rs`. `PathDecomposition`
becomes trivially small:

```rust
pub struct PathDecomposition {
    pieces: Vec<Piece>,      // xy-monotone quadratics, stored downward
    fill_rule: FillRule,
}

struct MonotonePiece {
    // lyon QuadraticBezierSegment<f32> or equivalent, in Pixels space,
    // reoriented so p0.y <= p1.y; ctrl doubles as the line marker
    // (from_line's exact midpoint convention)
    p0: Point<f32>, ctrl: Point<f32>, p1: Point<f32>,
    sign: f32,   // +1 if the contour ran downward here, -1 if upward
}
// scaled() turns this into the uploaded device-space PathPiece:
// p0, p1, ax, bx, ay, by (coefficients precomputed once), sign,
// sx = sign(p1.x - p0.x) in stored orientation (0 if vertical)
```

Computed by the salvaged `monotone_pieces` (split every `PathQuadratic` at
its x/y extrema, filter non-finite input), then per piece: record
`sign = ±1` from the original direction and flip upward pieces downward.
**Keep horizontal pieces** (zero y-extent — the top and bottom edges of
every rectangle). They are invisible to horizontal routes — the backdrop
and the horizontal leg skip them automatically via the half-open y-span
test — but the fragment shader's *vertical* leg genuinely crosses them;
dropping them corrupts the winding of every bin whose left edge passes
through one (e.g. the interior of every wide rectangle). A horizontal
piece is never flipped: set `sign = +1` and `sx = sign(p1.x - p0.x)` as
stored, so the uniform vertical-leg delta `-sign * sx` comes out as
`-sign(dx)`, the correct crossing sign for a downward route.
`Path::ensure_decomposition` calls this; still cached in the `Arc`, still
invalidated on mutation, exactly like today's lifecycle. That's the entire
build stage — no proxies, no crossings, no chains.

## Phase 2 — per-paint binning + backdrop (CPU)

Runs in `Scene::insert_primitive` (where `trapezoid_snapper.snap` used to
run), via a `PathBinner` struct with reusable scratch buffers held by
`Scene`. All work in **device pixels**: scale piece coordinates by
`path.scale_factor` on read.

**Grid.** Per path: a fixed grid of `BIN_SIZE = 16.0` device px (a plain
constant; a pure performance knob, never a correctness knob), anchored at
`floor()` of the top-left of the path's device bounds **clipped to the
content mask** (plus the one-column left margin below). Sizing the grid by
the geometry instead of the visible intersection is not just wasteful —
it made huge clipped paths (a zoomed canvas rectangle spanning ~40k px)
trip `MAX_BINS` and vanish entirely instead of rendering their visible
portion (`huge_path_clipped_to_small_mask` is the regression test).
Out-of-view geometry stays correct through the grid's edges: crossings
left of the grid clamp into the margin column and prefix-sum into every
visible column, while pieces wholly above, below, or right of the grid
cannot cross a visible sample line or pixel window and are skipped.

**Binning (piece lists).** For each piece, walk its *monotone trail* —
never its bounding box:

```text
for each bin row the piece's y-span touches:
    ya = max(piece.p0.y, row_top);  yb = min(piece.p1.y, row_bottom)
    xa = x_at_y(piece, ya);  xb = x_at_y(piece, yb)   // t_at_y + eval
    push piece index into bins[row][col] for col spanning [min,max](xa,xb)
```

Because pieces are monotone in both axes, the x-range over any y-slab is
exactly the interval between its endpoint evaluations, and the trail is a
staircase of O(rows + cols) bins — never rows × cols. Horizontal pieces
skip the trail walk (their `t_at_y` is degenerate): bin them into the
single row `floor((y - grid_top) / BIN_SIZE)`, columns spanning their
x-extent. They contribute no backdrop deltas — the half-open span test is
empty — with no special case needed there. Inclusion must be
conservative: a piece listed in a bin it barely misses costs one wasted
loop iteration (safe); a piece missing from a bin it touches silently
corrupts coverage there (never acceptable). When in doubt, widen by one
column.

**Backdrop.** For each grid-row top line `y = grid_top + r * BIN_SIZE`,
for each piece whose stored y-span contains it half-open
(`p0.y <= y < p1.y`): solve the crossing x (`t_at_y`, then evaluate x),
compute `col = floor((x - grid_left) / BIN_SIZE)`, and accumulate
`delta[r][col] += piece.sign`. Then per row, an **exclusive** prefix sum:
`backdrop[r][c] = Σ delta[r][c'] for c' < c` — the winding number at bin
(r, c)'s top-left corner, since nothing is left of the grid. This is a
scatter plus a prefix sum; no piece ever looks at another piece.

**Instance emission**, appended to `Scene` (four flat vectors replacing
`path_trapezoids`, all indexed absolutely so that sorting `path_bins` into
draw order in `Scene::finish` invalidates nothing):

- `bins: Vec<PathBin>` (48 bytes) — one instance per bin that intersects
  the mask-clipped bounds and is *interesting*:
  - non-empty piece list → loop instance;
  - empty list, backdrop "inside" (nonzero: `backdrop != 0`; even-odd:
    `backdrop` odd) → solid instance with `piece_count = 0` (adjacent
    equal ones merged into one wide instance per run);
  - empty list, backdrop "outside" → **no instance at all**.
  Carries only bin-varying data: `order`, a `paint` index, `piece_start`,
  `piece_count`, `backdrop: i32`, a fill-rule flag, the bin's top-left
  **corner** (the route origin — kept even when the drawn quad is clamped
  by the mask), and the quad rect.
- `piece_entries: Vec<PathPieceEntry>` (8 bytes) — concatenated per-bin
  piece lists.
- `pieces: Vec<PathPiece>` (40 bytes) — the frame-global device-space
  pieces in coefficient form (`p0, p1, ax, bx, ay, by, sign, sx`),
  appended once per path.
- `paints: Vec<PathPaint>` (104 bytes) — `bounds`, `content_mask`, and
  `color: Background`, appended **once per path** and shared by all its
  bins through `PathBin::paint`. The indirection is native to this
  pipeline (bins already index entries and pieces); the fragment shader
  still evaluates an intact `Background` through the shared
  `gradient_color`, so no per-pipeline paint special-casing exists.

Struct sizes are locked by `const` asserts next to the struct definitions
in `path_winding.rs` — HLSL structured buffers add no hidden padding for
these field types, and the asserts prove Rust doesn't either.

## Phase 3 — D3D11 pipeline + fragment shader

Rename the pipeline: `ShaderModule::PathTrapezoid` → e.g.
`ShaderModule::PathBin` ("path_bin"), entry points `path_bin_vertex` /
`path_bin_fragment`; update `gpui_windows/build.rs`, the `ShaderModule`
match arms, `DirectXRenderPipelines`, `upload_scene_buffers`, and
`draw_path_trapezoids` → `draw_path_bins`. The existing `PipelineState`
assumes one instance StructuredBuffer per pipeline; this pipeline needs
three (`PathBin` at t1, `uint` indices at t2, `PathPiece` at t3) — extend
`PipelineState` or hand-roll the extra two buffers for this pipeline only.

**Vertex shader:** same shape as the old `path_trapezoid_vertex` — 4-vertex
strip quad over the instance's rect, `to_device_position_impl`, gradient
prep, clip distances from `content_mask`.

**Fragment shader.** All math in device px; pixel box is
`[X0, X0+1] × [Y0, Y0+1]` with `X0 = floor(input.position.x)` etc. Let
`(Xb, Yb)` be the bin corner from the instance. The pixel's mean winding:

```text
w = float(bin.backdrop)
[loop] for i in 0..bin.piece_count:
    piece = path_pieces[path_piece_indices[bin.piece_start + i]]
    w += piece.sign * horizontal_term(piece)
    w += piece.sign * (-piece.sx) * vertical_term(piece)
coverage = fill-rule map of w      // below
return float4(color.rgb, color.a * coverage)
```

The loop bound is per-instance, so every pixel in the bin does identical
work: no divergence.

*Sign convention* (must match the CPU backdrop, which counted `+sign` per
crossing of a rightward line): a route-leg crossing counts with the sign of
`cross(leg_direction, curve_tangent)`. For rightward legs (backdrop line
and the horizontal leg) that is `sign(dy)` = `piece.sign` in stored
orientation; for the downward leg it is `-sign(dx)` = `-piece.sx * piece.sign`.

*Horizontal term* — the box-filter integral of "the piece crosses the
horizontal leg from `(Xb, y)` to the pixel at height `y`", which is exactly
an area the existing `piece_column_area` computes:

1. Window: `[ya, yb]` = pixel y-range ∩ piece y-span. **Always clamp the
   window to the piece's y-span before calling `piece_column_area`** — that
   function extends the boundary with constant x outside the span
   (trapezoid-edge semantics), but a winding crossing only exists inside
   the span. With the window pre-clamped, its extension terms are zero.
2. Gate on the bin's left edge: a crossing at `x_c(y) < Xb` is not on the
   leg and must contribute **zero for that y** (not "full pixel width" —
   naively skipping this gate is the most likely correctness bug in the
   whole shader). x-monotonicity means `x_c(y)` crosses `Xb` at most once:
   one `monotone_quadratic_root` solve on x splits `[ya, yb]` into a live
   sub-window (where `x_c >= Xb`) and a dead one. Pieces entirely right of
   `Xb` (the common case) skip the solve.
3. On the live window: `term = (yb - ya) - piece_column_area(p0, c, p1, ya, yb, X0)`,
   using the identity `clamp(X0+1 - x_c, 0, 1) = 1 - clamp(x_c - X0, 0, 1)`.

*Vertical term* — the piece crosses the bin's left edge `x = Xb` at most
once (x-monotone): solve `t` where `x(t) = Xb`, evaluate `y_c = y(t)`. The
crossing is on the leg only if `y_c > Yb`, **strictly**: the backdrop's
half-open convention (`p0.y <= y < p1.y`) means the corner's winding is
effectively sampled just *below* the row line, so a crossing exactly on
the line is already inside the backdrop, and counting it on the leg too
double-counts. This is not a rare edge case — an integer-coordinate
rectangle puts its top edge exactly on a row line whenever the grid
anchor lands there. No upper gate is needed: the box average
`clamp(Y0 + 1 - y_c, 0, 1)` already zeroes crossings below the pixel.
Pieces with `sx == 0` or an x-range not straddling `Xb` skip all of this.
Horizontal pieces flow through this same code with no special-casing —
`monotone_quadratic_root`'s linear branch solves their crossing and `y_c`
is their constant y — and their horizontal term is automatically zero
because the window clamp empties their y-span.

*Fill-rule map* (per-instance flag, uniform branch):

```hlsl
// nonzero: distance from 0, clamped
coverage = min(abs(w), 1.0);
// even-odd: distance to the nearest even integer (FreeType's fold)
coverage = abs(w - 2.0 * round(w * 0.5));
```

## Phase 4 — run it

- `cargo run -p gpui --example painting`, `paths_bench`, `gradient`
  (from `crates/gpui`). Stroked content is expected to be invisible;
  filled content must be correct.
- Run Zed itself: editor selections, the git panel graph, circular
  progress indicators are the real fill call sites.
- Visual checklist: a diamond or star whose apex sits exactly on a bin
  column line (e.g. a 32-px-wide diamond — the grid anchors at the shape's
  min-x, so its apex always does), solid under both fill rules; an
  axis-aligned, integer-coordinate rectangle taller than one bin — its top
  edge lies exactly on a bin row line — solid under nonzero *and* even-odd (the acid test for the horizontal-piece and
  leg-gate conventions; nonzero's clamp can mask a double count that
  even-odd folds into a visible hole); a self-intersecting star under both
  fill rules (the pentagram core is filled under nonzero, hollow under
  even-odd); a translucent fill showing no seams or double-dark rows
  anywhere; edges staying smooth across zoom / display-scale changes;
  nothing bleeding outside content masks.

## Accepted limitations — do not burn time fixing these

- CPU (backdrop) and GPU (legs) evaluate crossings with separate float
  arithmetic; a crossing landing within float error of a bin boundary can
  mis-tally one bin's winding for a frame. Local, rare, accepted for the
  spike.
- Sum-then-map coverage is exact wherever a pixel sees at most two
  adjacent winding values; pixels where several boundary arcs collide
  (self-intersection points, sub-pixel slivers, coincident or
  overlapping contours) get a bounded local approximation — the shader
  folds the pixel's *mean* winding through the fill rule, and
  `fill_rule(mean(w)) != mean(fill_rule(w))` when the boundary crosses a
  pixel more than once. Example: two coincident same-orientation contours
  covering fraction A of an edge pixel yield nonzero coverage min(2A, 1)
  instead of A; under even-odd, an edge pixel can show up to full
  coverage where exact evaluation gives zero. Interiors are always
  correct (the fold of an integer winding is exact) — only the one-pixel
  AA fringe conflates. This is the standard "conflation artifact" of
  every analytic-coverage renderer (FreeType, stb_truetype, Skia's
  analytic AA, Vello); fixing it requires per-sample fill-rule
  evaluation (MSAA/supersampling) or exact CPU face decomposition, both
  ruled out by the ground rules. Accepted permanently, not just for the
  spike.
- Strokes render nothing (deliberate, see ground rules).
- Performance is unmeasured; `BIN_SIZE` is untuned. Get it correct first.

## Optimization ledger — standing decisions, do not re-propose without new evidence

Done:

- Extent-based early-out in the fragment loop; root/parameter reuse in
  `piece_column_area`; exact-zero line coefficients; uploaded `leg_y`;
  run-length merging of interior bins; `sort_unstable_by_key` for bins.
- Zero-coverage early return before paint evaluation
  (`path_bin_fragment`): wholly-exterior pixels of boundary bins land on
  exactly zero winding (the untouched integer backdrop) and skip gradient
  direction math, Oklab conversion, and dithering. Straight-alpha
  blending makes the all-zero output a no-op.
- Per-path paint rows (`PathPaint`, `PathBin::paint`): bins carry a paint
  index instead of duplicated `bounds`/`content_mask`/`color`, dropping
  `PathBin` from 152 to 48 bytes. The acceptable form of the previously
  declined "paint split": a fourth indexed buffer in a pipeline already
  built on indexed side buffers, with `Background` kept intact for the
  shared `gradient_color`. The declined form remains declined:
  disassembling `Background` into per-pipeline interpolators.
- Coefficient-form pieces: `PathPiece` (device) stores `ax/bx/ay/by`
  precomputed on the CPU in `MonotonePiece::scaled`, alongside — not
  instead of — both endpoints. CPU binning and the shader consume
  bit-identical coefficient values (decision-upload extended to
  arithmetic), line pieces carry exact-zero quadratic terms *set* rather
  than derived, and the per-fragment derivation preamble is gone. The
  endpoints must stay stored: every discrete gate compares against them,
  and reconstructing `p1` as `p0 + a + b` is not exact in general (sums
  can leave f32's 24-bit-exact range even on the lattice), which would
  reopen the tie class. 40 bytes/piece; piece count scales with path
  complexity, not area, so the growth is immaterial.
- Path-owned tiles (see "Current data model"): tiles/curves live on the
  `PaintedPath` itself, produced once in `Path::painted`, so
  `Scene::replay` of cached elements clones instead of re-binning, and
  the scene/batching layers carry no tile-specific logic. The eager-inline
  decomposition also removed the `Arc`/`Option` cache — a path painted
  fresh every frame re-decomposes every frame, which is strictly
  dominated by the binning it already pays for.
- Scene-flattened GPU buffers (`Scene::flatten_paths`, run by `finish`):
  the per-path concatenate-and-rebase loop moved out of the renderer into
  one named scene phase producing flat `path_paints`/`path_curves`/
  `path_tiles`/`path_tile_curves` vecs with globalized indices. The
  renderer's path uploads are now verbatim `update_buffer`/`write` calls
  (the `monochrome_sprites` pattern), `PathPaint` lost its
  `curve_base`/`tile_curve_base` fields, the shader lost its base adds,
  and the `MappedBuffer`/`write_with` upload machinery was deleted. The
  copy is the same copy the renderer already made every frame, relocated
  where all three backends can share it; `PrimitiveBatch::Paths` carries
  the tile instance range, accumulated by the batch iterator.
- Derived tile rectangles: `PathTile` no longer stores its clipped quad —
  the vertex shader derives the rectangle from `corner` and a `run` cell
  count (`PATH_TILE_SIZE` mirrored in HLSL), and the content-mask clip
  distances trim partially masked tiles that binning used to pre-clip.
  40 → 28 bytes per tile; binning still skips wholly-invisible tiles.
- Sorted-tile early-out (Slug's sorted bands, mirrored): each tile's
  curve list is sorted by leftmost x at emission, and the fragment loop
  breaks at the first curve entirely right of its pixel's column — safe
  because such curves fail the rightward-leg gate and cannot carry a
  downward-leg booking (that requires straddling the tile's left edge,
  which is left of every pixel's right edge). Costs one comparison per
  iteration and a tiny CPU sort per boundary tile; trades the old
  uniform-trip-count lockstep claim for skipped root solves in
  curve-dense tiles.
- Deleted `Path.segments`/`PathQuadratic`: the retained as-built segment
  vec had no readers — the inline monotone decomposition is the geometry
  — and its two `is_empty` guards reduce to `current != start` (only
  segment-pushing calls separate them).

Deferred pending measurement (the solid-color `paths_bench` A/B, then a
GPU capture, decide priority — in that order):

- Gradient/paint optimization (`gradient_color`): per-instance
  precomputation must ride the existing vertex-to-fragment interpolator
  pattern (`background_solid` et al.) uniformly across all pipelines, and
  the two `sin`-based dither hashes are candidates for a cheaper hash.
  Benefits quads as much as paths; do not fold it into path work.
- Vertical merging of equal pieceless runs (currently one instance per
  bin row): instance count has not been the measured bottleneck anywhere.
- Sign-bit crossing convention (Slug's `CalcRootCode` principle,
  degenerate monotone form: a crossing exists iff
  `signbit(y0 - sample) != signbit(y1 - sample)`): would replace the
  half-open comparison conventions and possibly the leg bookings with a
  decision CPU and GPU cannot disagree on, and IEEE `x - x = +0.0` makes
  exact ties deterministic for free. Deferred, not declined — two real
  obstacles. First, the lattice's *other* job survives it: snapping also
  collapses sub-f32-spacing slivers (lyon has emitted `y = 255.00002`
  for a requested `255`) into the exactly-degenerate cases the
  conventions handle, which sign bits do nothing about, so the lattice
  cannot simply be deleted. Second, our discrete decisions consume
  crossing *positions* (backdrop scatter buckets, `leg_y`), not just
  crossing counts — Slug's ramps are continuous in the position, ours
  are not. Needs its own session with the tie tests as the referee;
  the current lattice + shared-booking design passes them today.

Declined (re-open only with a capture proving the premise):

- Per-entry row-slab x-range upload to pre-reject full-left/full-right
  lanes: grows every entry to save solves that feed the area integral
  anyway in the columns that pay.

For the depth-buffer project (not actionable until it exists):

- Opaque interior runs are exact-coverage-one quads. Once the depth
  buffer lands, pieceless bins with opaque paint qualify for phase-1
  front-to-back opaque drawing with depth write, and early-z then
  rejects exactly the redundant overdraw that `paths_bench` punishes —
  dense opaque overlap gets cheaper without touching the winding math.
  Boundary bins keep blending in phase 2.

## Slug reference code — read before writing the shader

Eric Lengyel dedicated the Slug patent (#10,373,352) to the public domain
on 2026-03-17 and published reference shaders under MIT/Apache (credit
required if code is copied): https://github.com/EricLengyel/Slug — see
also https://terathon.com/blog/decade-slug.html. Closest shipped relative
of this design's fragment loop. Specifically:

- **Rejected after implementation review: line encoding `{p1, p2, p2}`.**
  Originally listed here as a steal; the review killed it, twice over.
  First, the `abs(a) < eps` branch is not a line branch — it catches
  *component-linear* pieces (control point at one coordinate's midpoint,
  e.g. `(0,0), (1,0), (2,1)` has `ax == 0` yet is a genuine, already
  monotone curve), so the branch stays load-bearing under any line
  encoding. Second, the swap costs accuracy: midpoint-encoded lines are
  *exactly* component-linear, making the linear solve their true root,
  while `{p1,p2,p2}` lines have merely tiny coefficients — one that trips
  the epsilon gets `s/2` where the true root is `1 - sqrt(1-s)`, a
  crossing misplaced by up to a quarter of the piece's extent, landing on
  the backdrop scatter. The tip is correct in Slug's habitat (fp16 curve
  texture where a stored midpoint isn't exact anyway; contour packing
  where the duplicated endpoint is free; no monotone split) and wrong in
  ours. **Keep midpoint encoding.**
- **Steal: clamped-discriminant style.** `sqrt(max(b*b - a*c, 0.0))` with
  reciprocal-multiply roots; tangent/imaginary pairs collapse to a double
  root whose two opposite-signed contributions cancel exactly. Branchless
  tangency handling; crib the formulation when touching the root solve.
- **Confirmation only: even-odd fold.** His
  `1.0 - abs(1.0 - frac(w * 0.5) * 2.0)` is equivalent to the fill-rule
  map above (`abs(w - 2.0 * round(w * 0.5))`), shipping behind a
  per-glyph flag. Ours was derived independently from the FreeType
  precedent, not copied, so no attribution obligation attaches — this
  citation is corroboration, not compliance.
- **Do not take: `CalcRootCode` sign-bit tables** (the monotone split
  already reduces crossing eligibility to endpoint window tests),
  **sorted bands with early-out breaks** (divergence; contradicts the
  uniform-loop principle, and he deleted his own fancier variant), or
  **dynamic dilation** (pixel-aligned screen-space bins have nothing to
  dilate).
- His 1D-coverage dual-ray weighted blend (`CalcCoverage`) is the
  production version of the cheaper tier-3 fallback — relevant only if
  measurements force us off the exact area integral. Not spike work.
- If any code or formula is copied, add attribution per his license in a
  NOTICE/comment.

## Measured datapoints

- Intel HD 4600 (i7-4770, the target weak-GPU tier), `painting` example
  (real mixed content, three path batches interleaved with quads and
  text): accumulate-resolve 8,700 µs/frame with 7,536 µs of path work;
  binned winding 1,555 µs/frame with 394 µs of path work — 19x on path
  work, 5.6x on the frame (quads control identical to 0.2% in both
  captures). The decisive detail: the old design paid ~2,000 µs of
  ResolveSubresource plus ~105 µs of intermediate clear per path batch
  *regardless of content* — its third batch drew 69 instances in 124 µs
  wrapped in 2,245 µs of overhead. The toll scaled with batch count, not
  ink; real UI has dozens of path batches. This measurement is the
  project's founding bet, confirmed on target hardware.
- Intel HD 4600, `paths_bench`: accumulate-resolve 81.6 ms, binned
  winding 100.9 ms (1.24x, consistent with the desktop ratio below);
  both unusable at that workload, and per-path cost (~50 µs/star) is
  far under the pre-registered ~2 ms dense-fill abort criterion.
- Desktop Radeon, `paths_bench` (2000 identical opaque stars, fully
  overlapping, one draw-order batch): accumulate-resolve 6.2 ms, binned
  winding 8.2 ms (41M fragment invocations across 160k bin instances).
  This bench is the old design's best case and the new design's worst
  case at once: a single batch means the old pipeline pays its
  clear/resolve/pass-restart toll exactly once, while 2000x overdraw
  makes the new pipeline re-derive identical per-pixel winding 2000
  times. It measures the axis the design deliberately traded away and
  gives zero weight to the axis it bought (no mid-frame passes — the
  same machine measures the mixed-content `painting` example at 233 µs
  old vs 56 µs new). First lever for this workload class: `BIN_SIZE` —
  boundary-bin fragment cost scales roughly linearly with bin side
  (work ≈ perimeter × bin height × pieces per bin), and the bench is
  nearly all boundary bins. The harness passes at 8/16/32, so sweeping
  is safe.
- Desktop Radeon, `paths_bench` at BIN_SIZE 8: 7.8 ms vs 8.2 ms at 16
  (instances 160k to 350k). The linear boundary-scaling model predicted
  roughly half and was wrong: total fragment count is bin-size-invariant
  (bins tile the same shape at any size), and each fragment pays
  bin-size-independent costs — instance fetch, loop setup, winding fold,
  and above all the paint. This bench's stars are Oklab gradients, whose
  per-pixel cost (gradient direction math, Oklab conversion, a
  two-`sin()` dither hash) rivals a boundary bin's whole winding loop
  and runs on every fragment in either pipeline; the old design paid
  the identical paint bill, which is part of why it wins this bench.
  Smaller quads also fill waves less efficiently. Verdict: BIN_SIZE
  stays 16; it is a weak lever on paint-heavy content. To decompose the
  bench further, A/B it with a solid fill — if the time collapses, the
  residual story is expensive paint times 2000x overdraw, which no
  binning choice addresses (and the per-pixel dither hash becomes a
  cross-pipeline optimization target).

## Pitfalls — hard-won, do not re-litigate

- No bounding-box binning: a long diagonal piece's bbox covers
  quadratically more bins than its actual staircase trail.
- Never geometrically clip/split pieces to bins. Pieces are shared whole
  via the index buffer; clamped evaluation does the clipping.
- Never omit a piece from a bin it might touch (silent corruption);
  over-inclusion is merely slow.
- Do not extend winding contributions past a piece's y-span (mind
  `piece_column_area`'s extension semantics — clamp the window first).
- Do not drop horizontal pieces: they contribute crossings to the
  downward legs, and dropping them corrupts the interior of every wide
  rectangle. Winding-count bugs generally are masked by nonzero's
  `min(abs(w), 1)` (a double count clamps back to 1) while even-odd
  exposes them as holes — lyon's default fill rule is even-odd, so real
  callers hit the exposing rule. Validate under both.
- Do not adjudicate discrete crossing-ownership decisions with tie-break
  conventions. A whole season of bin-sized holes (star vertex, dome curve
  endpoint, pie arc endpoint, every integer rectangle's vertical edge —
  all at 100% display scale, where integer geometry sits exactly on bin
  lines; fractional scales hide the class) was patched case by case with
  half-open gates, heading-dependent column shifts, and a per-row
  baseline, each fix creating the next. All of it was replaced by three
  structural rules that make the ties unrepresentable or irrelevant
  instead of adjudicated:
  1. Geometry snaps to the 1/256 px lattice exactly once, **in device
     space**, when pieces are scaled for binning (`MonotonePiece::scaled`).
     Path construction emits slivers thinner than f32's resolution at
     screen magnitudes (lyon returned an arc endpoint at y = 255.00002;
     f32 spacing at 255 is 1.5e-5), and no convention can count what the
     arithmetic cannot represent. Device space is the only place the
     lattice buys anything: snapping logical coordinates instead does
     not survive scaling — a non-power-of-two scale carries logical
     lattice values onto arbitrary reals, some of which land exactly on
     a sample point (logical 410/256 at Windows' 125% scale is device
     2 + 1/512), where the CPU backdrop and the shader's leg gates each
     assume the other owns the crossing and the bin loses a winding step
     (`fractional_scale_sample_collision` is the regression test). A
     logical-space snap on top of the device one is pure vestige; it was
     tried first, proven insufficient, and deleted.
  2. Each bin's backdrop sample point sits at corner + 1/512, off the
     lattice, with one column of grid margin on the left. No snapped
     boundary can pass through a sample, so every left-or-right decision
     has a guaranteed gap that dwarfs float noise on both processors —
     CPU and GPU agree without having to round identically. This
     guarantee lives in device space, which is why rule 1 re-snaps there.
  3. Crossing ownership is decided exactly once, on the CPU, and
     uploaded. During binning, one bucketing per (piece, row) — where the
     piece sits at the row's sample height — feeds both the backdrop
     deltas and a per-(bin, piece) downward-leg booking. Each piece-list
     entry carries that booking in the high bit of its index plus the
     crossing height of the piece with the bin's left-edge line, solved
     once. The two are not redundant: the coordinate is a geometric fact
     (the window split needs it even when the leg does not count the
     crossing — one above the sample point belongs to the backdrop), and
     the bit is a convention no arithmetic could recover from the
     coordinate without reopening CPU/GPU disagreement — including on
     platforms whose fractional scale factors take device-space geometry
     off the lattice (Wayland scales are multiples of 1/120, not dyadic).
     Uploading both deletes every per-pixel root solve whose answer is
     constant over the bin. The window split clips only the
     crossing-count height by the uploaded coordinate and leaves the area
     integral on the full window: over the clipped-away part the piece is
     left of the corner, where the column integrand is zero anyway (short
     of 1/512 on the bin's leftmost pixel column, far below visibility),
     so the already-solved parameter bounds stay valid. Bookings are also
     self-correcting: even an ill-conditioned crossing solve is harmless,
     because backdrop and leg act on the same booking rather than each
     estimating the truth.
- Line pieces must reach the solves with *exactly zero* quadratic
  coefficients, so every solve against a line — the dominant piece
  population — takes `monotone_quadratic_root`'s linear branch: no square
  root, no near-singular leading coefficient. With coefficients uploaded,
  this is enforced by *setting* them in `MonotonePiece::scaled` (`ax = ay
  = 0`, and `bx/by` as endpoint differences, exact on the lattice) rather
  than deriving them from any control point. Lines are recognized there
  by `from_line`'s midpoint-control-point convention — the logical
  control point exists only as that marker. Do not "unify" the line and
  curve branches by deriving line coefficients from a midpoint: deriving
  reintroduces rounding (scaling is not distributive; a derived cancel
  can miss by an ulp at fractional scale factors), which historically
  sent every line down the quadratic path silently.
- The fragment loop classifies each (piece, pixel) by the piece's exact
  x-extent over the pixel's row window (monotonicity makes the extent the
  interval between the two window-end evaluations, already computed):
  entirely right of the pixel's column contributes zero, entirely left
  contributes the whole window, and only the column(s) the piece actually
  passes through pay for the exact area integral, which also reuses the
  window-end roots instead of re-solving. Do not "simplify" by calling
  the integral unconditionally; for a 16-pixel bin that reintroduces the
  expensive path on roughly fifteen of sixteen pixels per piece.
- Per-path paint data lives in `PathBins::paints`, indexed by
  `PathBin::paint` (see the optimization ledger for history — an earlier
  form of this split was declined, then adopted once framed as a fourth
  indexed buffer in a pipeline already built on indexed side buffers).
  The constraint that survives: the fragment shader must keep evaluating
  an intact `Background` through the shared `gradient_color`. Do not
  disassemble `Background` into per-pipeline interpolators reassembled
  per pixel — that variant stays declined. Independent of layout, the
  answer to interior *instance count* is run-length merging in `emit`:
  consecutive pieceless cells share their backdrop — provably, since a
  scatter delta's cell always also lists the piece that produced it — so
  a wholly-inside run emits one wide constant-coverage quad instead of
  one instance per bin, collapsing a full-screen path's interior from
  thousands of instances to one per row.
- Keep the Rust shader transcription in `path_winding.rs` tests in
  lockstep with `shaders.hlsl` — it is the only guard for this bug class.
  The harness rasterizes acid shapes against a supersampled reference at
  several scale factors including 1.0; its shape list must include every
  lattice-aligned boundary archetype a UI draws (integer rectangle,
  45-degree diamond, circles with axis tangents, self-intersecting star,
  arc wedges). A regression shipped because the list lacked an integer
  rectangle.
- No diagonal or "direct" per-pixel routes: axis-aligned legs are what
  makes each piece a single closed-form root solve. Route-independence of
  winding guarantees the L-route equals the naive ray cast.
- No intermediate textures, no extra passes, no MSAA, no stencil, no
  UAV/atomic accumulation, no per-backend algorithm forks. All previously
  explored and rejected; see `trapezoid_path_rendering.md` for why.
- Do not resurrect any sweep concept (crossing detection, span matching,
  chains, corner patches, row snapping). If a problem seems to need one,
  the design is being misapplied — stop and reconsider against this
  document instead.
