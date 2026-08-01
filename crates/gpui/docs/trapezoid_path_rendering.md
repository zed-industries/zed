# Trapezoid Path Rendering

**Status:** Revised across three rounds (July 2026): two Windows
prototyping rounds and one prior-art/analysis round. A working D3D11
implementation exists: fills as specified here; strokes currently as
stamped chains, which the **second-round findings** at the end of this
document replace with distance-coverage instances — the Strokes section
below describes that final design. Round three (WPF MILCore prior art,
the curves-vs-flattening question, and the drag race that decides it)
is folded into Background/Design and logged in its own findings section.
A fresh session should read this document top to bottom; the findings
sections at the end are the authoritative log of what was measured and
decided.
**Scope:** GPUI path rendering on all backends (Metal, D3D11, wgpu)

## Summary

Replace the intermediate-texture path renderer with direct-draw coverage
instances in the main render pass, blended like any other primitive. Two
producers feed **one instance buffer and one pipeline**:

- **Fills** are flattened and decomposed once at build time into
  pixel-row-aligned, y-monotone trapezoids. The instance format still
  carries **monotone quadratic Bézier pieces** (lines as degenerate
  quadratics) because stroke caps need them, but fill input is flattened
  to lines at τ = 0.25 px before the sweep — the drag race (round four)
  retired the curve-carrying fill sweep. The fragment shader computes
  the **exact area integral** of the region over the pixel — true
  box-filter antialiasing, which 4× MSAA only approximates with four
  samples. No offscreen accumulation.
- **Strokes** are dashed and flattened once at build time into centerline
  segments drawn as capsule/box instances; the fragment shader computes
  coverage from **distance to the segment** — the technique GPUI's quads
  and underlines already use. (This is the third stroke design; the
  findings sections record why the first two died.)

This deletes, on every backend: the full-screen path intermediate texture,
the full-screen MSAA texture, the per-batch clear/resolve cycle, all
mid-frame render pass restarts, and the path-sprite composite machinery
(including the disjoint/union bounds logic that is currently triplicated
across the three backends). The frame becomes a single render pass. Fill
rendering becomes flattening-free; stroke centerlines still flatten, but
once at build time at a fixed tolerance in `Pixels`, so all path CPU cost
becomes independent of resolution and DPI for the first time.

## Background

### Current architecture

Paths are tessellated on the CPU (`PathBuilder` → lyon `FillTessellator` /
`StrokeTessellator`). Curves are flattened to line segments at tessellation
tolerance; the resulting triangles carry constant `st = (0, 1)`, so all
curve smoothness today comes from flattening density plus MSAA. (A separate
manual API, `Path::curve_to`, emits true Loop-Blinn curve triangles; see
Open Questions.)

Per `PrimitiveBatch::Paths`, every backend then:

1. Ends the main render pass.
2. Clears a full-screen 4× MSAA texture, rasterizes the batch's triangles
   into it at final screen positions with premultiplied blending, and
   resolves the entire texture into a full-screen intermediate.
3. Re-opens the main pass with `Load`.
4. Composites the batch's pixels from the intermediate onto the framebuffer:
   one sprite per path when all paths in the batch share a draw order
   (provably disjoint), otherwise one spanning union rect — because blended
   pixels must be composited exactly once (see #35688).

### Costs

At 4K (3840×2160):

| Resource | Metal (Apple GPU) | Metal (Intel Mac) | D3D11 | wgpu |
|---|---|---|---|---|
| Intermediate (4 B/px) | ~33 MB | ~33 MB | ~33 MB | ~33 MB |
| MSAA ×4 (16 B/px) | 0 (memoryless) | ~133 MB | ~133 MB | ~66–133 MB |

Per path batch, regardless of path size: a full-screen clear, a full-screen
resolve (~166 MB read + 33 MB write on immediate-mode GPUs), and a render
pass restart (a full store+load of all attachments on tile-based GPUs;
a binning flush on tiled D3D11 drivers). A 24-pixel spinner pays all of it.

In addition, flattening makes path CPU cost **resolution-scaled**: tolerance
is measured in device pixels, so segment counts grow with both curve radius
(∝ √radius) and display scale (∝ √scale). A 10 px-radius spinner ring is
~60–100 chords today at 2× scale; the same ring is ~20–25 monotone
quadratic pieces in this design, at any scale.

One more per-frame cost, easy to miss because it hides inside a
"cached" path: the current `Path::scale` runs on **every paint** and
re-allocates and rescales the entire vertex vector
(`vertices.iter().map(|v| v.scale(factor)).collect()` in `scene.rs`).
Build-once-paint-many has therefore never been free — today it costs an
O(vertices) multiply-and-allocate per paint, forever. Every conceivable
design has *some* paint-time scale-bound step, because pixels exist only
at paint time; the honest question is its size. This design's residue —
row snapping — is O(instances), allocation-free, and measured at ~1% of
decomposition cost: smaller than the residue it replaces. It is also not
mere bookkeeping: snapping seams to pixel rows is the mechanism that
hides them (see below).

### History (pitfalls this design must not repeat)

- **Pre-2025 per-path atlas**: one atlas tile per path; memory scaled with
  path *count* regardless of pixel area (5000 overlapping stars → 15 GB) and
  atlas textures never shrank. Removed by #34992.
- **#29718** (direct draw + whole-framebuffer MSAA): correct and
  memory-cheap, but MSAA on the main target collapsed Intel GPUs (37 fps on
  an empty window; sluggish typing, #34659). Reverted in #34722.
  **Lesson: never MSAA the framebuffer; be suspicious of per-fragment cost
  on weak integrated GPUs.**
- **#34992** (current): full-screen intermediate, reused serially per batch.
  Memory is O(screen) but the serial reuse forces the per-batch restart.
- **#35688**: composite rects must include the content mask, or "disjoint"
  rects overlap and translucent pixels get composited twice (visible opacity
  doubling).

### The conservation law, and why direct draw escapes it

Any design that stores path pixels in an intermediate must, for N
overlapping translucent path batches z-interleaved with other primitives,
either hold N results simultaneously (memory blowup) or serialize
rasterization mid-frame (pass restarts). The taxes can be shifted but not
eliminated — unless the intermediate itself is eliminated.

Direct draw eliminates it. The keystone is that the `over` operator is
**associative**: compositing paths A and B into an intermediate and then
compositing that result onto the framebuffer equals blending A and then B
directly onto the framebuffer in draw order. Inter-path blending therefore
needs no intermediate at all. The only thing the intermediate was actually
buying is *intra-path* coverage correctness: a path's own coverage must
reach the framebuffer through exactly one blend per pixel. Trapezoids
provide that by construction: they partition the path, each pixel is owned
by one trapezoid (or one corner patch), and that owner computes the pixel's
complete coverage in registers before the single blend.

### Why trapezoids and not triangles; why an exact integral

The primitive and its fragment math are forced by the constraints, not
chosen by taste. Recording the forcing chain, because every reviewer asks:

**Triangles require MSAA.** A tessellated fill is full of interior edges
shared between two triangles. Under MSAA, sample points partition between
the triangles and coverage sums exactly. Under analytic coverage with
ordinary blending, each triangle blends separately: 0.5 over 0.5 gives
0.75 — a grey seam along **every interior edge of every path**.
"Triangles + analytic AA" is not a coherent design point; triangles were
only ever viable because MSAA (and its memory/pass taxes) subsidized
their seams. Deleting MSAA deletes triangles.

**Decomposition seams must hide on the pixel grid.** Any decomposition
has internal seams; a seam is harmless only where no pixel straddles it,
and the only cuts that can be aligned to the pixel grid are horizontal
(or vertical) ones. Cutting a winding region horizontally at pixel-row
boundaries yields exactly one cell shape: two row-aligned horizontal
sides plus the path's true left/right edges — a trapezoid with curved
sides. The trapezoid is not a preference among shapes; it is the
scanline cell, the decomposition whose seams can all be hidden.

**What the sweep hands the shader is structure, not just compression.**
Trapezoids are the run-length encoding of scanline spans — the sweep
already computes per-row fill information while resolving winding, and
a trapezoid is a maximal vertical run of rows with identical span
structure. But two properties of that output matter more than its size.
First, it is **winding-free**: the sweep consumes the winding numbers
and emits non-overlapping cells that tile the region exactly once, so
the GPU needs no stencil pass, no accumulation, and no order dependence
within a path — a global property converted into a local one. Second,
it preserves the **left/right boundary pairing**: the fragment shader
computes `area(R) − area(L)`, and the sweep is what knows which two
edges bound the region at a given pixel. A triangulation of the same
region is equally lossless and similarly compact, but it shreds exactly
that pairing — a triangle knows its own three edges and nothing about
which are silhouette versus scaffolding. Note also that "trapezoid"
here names the *topology* (two y-monotone edges, horizontal top and
bottom), not the shape — the edges stay curved. A straight-sided
trapezoid emitted from curved input would be flattening by another
name.

**Coverage is always an area integral; distance is a shortcut for whole,
symmetric shapes.** Every analytic primitive answers "what fraction of
this pixel does the shape cover". Shapes with the right symmetry admit a
closed-form shortcut via distance — GPUI's rounded quads, shadows,
underlines, and stroke capsules. Interior *pieces* of a region do not
qualify, three times over: (1) a cell composes with the cells above and
below it, so its coverage must be exact or the seams reappear; (2) its
bounding curves have arbitrary orientation, including near-horizontal
(the apex of every rounded corner and circle), where distance ramps
degrade without bound while the integral is orientation-independent;
(3) cells thinner than a pixel are bounded by two *different* curves, so
the composed-ramps weight error has no slab-formula rescue. And the only
robust approximation — true Euclidean distance to a quadratic — needs a
cubic root solve per fragment, costing *more* than the exact integral
(two stable quadratic solves plus a quartic antiderivative). For region
pieces, exact is simultaneously the most robust and roughly the cheapest
option. The integral is not the exotic member of the family; it is the
general case, and every distance-based primitive is a special case.

**Fills are regions; strokes are brushes.** A fill is defined globally by
winding; a stroke is a curve with a brush, local by definition. The old
renderer already had two unrelated algorithms (lyon's `FillTessellator`
and `StrokeTessellator`) unified at an output format (triangles + MSAA).
This design keeps precisely that unification boundary — two producers,
one instance format, one pipeline, one pass — and both prototype rounds
confirmed it empirically: routing strokes through the fill pipeline was a
measured 100× regression and the source of every observed glitch. Do not
attempt to unify the algorithms again.

### Why the industry standardized on triangles (≈2009–2017), and why that ended

Triangle tessellation was not a mistake this design corrects; it was
the right answer under three conditions that have since inverted:

1. **MSAA became free-ish and universal.** MSAA point samples partition
   across shared edges, so the interior-seam problem that forbids
   "triangles + analytic coverage" vanishes under MSAA. Once DX10-class
   desktop hardware (and tile-based mobile GPUs, which resolve on-chip)
   made 4× MSAA cheap, the primitive the hardware is built around
   became available for vector fills. Triangles did not win an
   argument; MSAA dissolved it.
2. **Transform-invariance became a hard requirement.** Compositors,
   pinch zoom, and animated transforms demand geometry that survives
   arbitrary affine transforms with a matrix update. Triangle meshes
   are closed under transform; a y-monotone decomposition is married to
   an axis and must be redone under rotation. For a browser engine that
   is disqualifying. (GPUI's path API offers scale and translation but
   no rotation, so this design does not pay that cost. If paths ever
   grow rotation, decomposition must happen in post-transform space;
   scale alone re-snaps without re-decomposing.)
3. **Shader ALU was scarce; rasterizer and stencil hardware were
   abundant.** Stencil-then-cover (NV_path_rendering; Skia Graphite's
   mainline) outsources winding resolution to the stencil unit — no
   triangulation, no sweep, no CPU geometry processing at all. When
   silicon answers the coverage question for free, spending 30 ALU per
   fragment on an integral is absurd.

Each condition has weakened or reversed: HiDPI made full-screen
MSAA-resolve bandwidth the scarce resource (resolution-scaled ceremony,
exactly the cost class this design deletes); fragment ALU became the
abundant one; and compute shaders let the scanline sweep's descendants
return to the GPU (Vello's per-tile winding accumulation is a
parallelized scanline algorithm; Pathfinder's fills are per-pixel area
integrals). The genealogy is coverage → triangles → coverage; WPF was
the last major pre-triangle coverage renderer and this design is a
post-triangle one. In one line: **triangles won when hardware answered
the coverage question (MSAA, stencil) and lost when the price of asking
became bandwidth instead of ALU.** GPUI's constraint set — no path
rotation, HiDPI targets, a failure history of bandwidth surprises on
weak GPUs, ALU to spare — is point-for-point the post-triangle set.

### Why not Slug?

Slug-style banded per-pixel winding evaluation (now public domain) is the
other attachment-free family member. It is dominated here on both axes:

- **CPU**: Slug's band structures are built offline for static fonts; GPUI
  would rebuild and re-sort them per frame — new work of the same order as
  the trapezoid sweep, which replaces work lyon already does.
- **GPU**: Slug's fragment loops over all curves in a band with root
  *eligibility* logic — the branchy robustness machinery that is the hard
  part of the technique. This design's fragment evaluates exactly two known
  edges with no loop and no eligibility cases, because the CPU pre-split
  every curve into monotone pieces. Monotone pieces have exactly one
  eligible root by construction. **This simplification is affordable only
  because we rebuild per frame anyway** — a static-content renderer cannot
  make this trade, which is why the literature doesn't.

Estimated fragment cost is 5–20× below banded evaluation, on the hardware
class (weak integrated GPUs) that historically vetoes path renderers.

### Prior art: WPF MILCore (read before touching the sweep)

WPF's open-source native layer (`WpfGfx`, in the dotnet/wpf repository)
contains the direct ancestor of this design and of Direct2D's
rasterizer: `core/hw/hwrasterizer.cpp` (1,484 lines) is literally titled
"Trapezoidal anti-aliasing implementation". Same skeleton — active edge
list, vertical sweep, winding resolution, trapezoid output rendered
directly into the main pass with blending, no intermediate texture —
built in 2004 for fixed-function GPUs. Where it differs, each difference
is instructive:

- **Lines only, 28.4 fixed point, integer DDA edges.** All curves are
  flattened first, per paint, at device resolution. Their robustness
  comes from integers: DDA advancement is exact, comparisons never lie,
  no epsilons exist. That exactness is available only to line edges —
  adopting it would mean flattening, which is why this design cannot
  simply borrow their arithmetic (see the flattened-sweep retreat
  position in the Design section).
- **AA as vertex-ramp fringes, and the two-mode fallback it forces.**
  Each trapezoid becomes a tri-strip whose `Diffuse` alpha ramps
  0 → 1 → 1 → 0 across a fringe of half-width `0.5 + |0.5/slope|` — a
  linear-ramp approximation of edge coverage, interpolated by the
  fixed-function unit because 2004 fragment ALU could not afford more.
  Because the fringes are *geometry*, they can collide: near-horizontal
  edges blow the fringe width up (`1/slope`), converging edges overlap,
  and `ComputeTrapezoidsEndScan` spends ~300 lines deciding per slab
  whether trapezoids are safe to emit — falling back, when they are
  not, to a CPU coverage buffer at 8×8 subpixel resolution emitted as
  per-pixel "complex scans". Two full rendering modes, forever. This
  design's coverage lives in the fragment shader inside pixel-row-
  aligned instances: there is no fringe geometry, nothing can overlap,
  and the entire failure taxonomy — overlap decision tree, bail-out,
  fallback rasterizer, near-horizontal blowup (the cause of WPF's
  notoriously soft diagonals) — is structurally absent. That is the
  strongest argument on record for paying ~30 ALU for the exact
  integral.
- **Strokes are widened into fills** — `core/geometry/strokefigure.cpp`
  is 4,930 lines, the largest module in their geometry layer, before
  counting pen scaffolding. This is the posted price of "strokes as
  fills, done properly, in production", and it is the existence proof
  behind this design's stroke pivot (≈130 lines of capsule/box
  instances). Their fill rasterizer tolerates self-intersecting widened
  outlines because the coverage-buffer fallback accumulates winding —
  they paid in a permanent second rasterizer what our first stroke
  attempt paid in a 100× regression.
- **Exactness is quarantined to where there is no fallback.** The
  rasterizer never decides crossings exactly — it uses conservative
  64-bit fraction bounds, annotated "we can be too conservative here",
  because a wrong-but-conservative answer only costs a missed fast
  path. Exact arithmetic exists only in the geometry Booleans
  (`ExactArithmetic.cpp`: 1,073 lines of hand-rolled 192-bit integers;
  `LineSegmentIntersection.cpp`: 2,856 lines), where a wrong sign is
  wrong topology forever and nothing downstream can absorb it.
- **The interior/fringe split is 20 years old.** Their tri-strip is
  fringe / flat-coverage-1 interior / fringe, with `NeedInsideGeometry()`
  to skip interiors entirely — the opaque-core idea in Interior
  optimization, already shipped in 2006.

Three posture rules distilled from that code, adopted for the sweep
(they cost sentences, not mechanisms):

1. **Decisions are conservative, never exact.** Every predicate should
   be asked "might this happen?", not "does this happen?", with the
   cheap side of wrong chosen deliberately. For crossing detection the
   cheap side is *more* splits: a spurious split wastes nanoseconds, a
   missed one corrupts the active list (round-one bug #2).
2. **Degradation is local, never persistent.** WPF bails per slab and
   retries on the next scanline. The active edge list must be
   self-healing: order inversions observed during incremental
   maintenance are repaired locally (re-sort the slab, continue),
   bounding any numerical miss to a one-slab artifact instead of a
   persistent streak.
3. **Exactness is quarantined to where there is no fallback** — and
   because rules 1–2 give this sweep a fallback everywhere, the
   corollary is that we pay for exactness nowhere. The fix for a sweep
   robustness bug is never "better numerics"; it is "cheaper recovery".

## Design

### CPU side: trapezoid emission *(revised: split across build and paint)*

**Original design (corrected):** run everything per path, per frame, in
device pixels. The prototype showed this silently removes an amortization
boundary the old renderer had: a built `Path` carried its tessellation, so
build-once-paint-many callers (common internally and among external users)
paid only a memcpy per frame. Per-frame emission made those callers 100×+
slower on large paths.

**Revised split.** Steps 1–3 below (normalize, monotone split, sweep —
everything expensive) are *scale-independent*: extrema, intersections, and
winding topology are invariant under uniform scaling. They run **once at
`PathBuilder::build`** in `Pixels` space, producing a `PathDecomposition`
(pieces + chains) stored in the `Path` behind an `Arc` and invalidated by
mutation — the same lifecycle as the old design's baked triangles, with no
keyed cache. Only steps 4–6 (pixel-row snapping and instance stamping,
measured at ~1% of emission cost) depend on the display scale; they run
per paint, scaling coordinates on read. A scale change re-snaps but never
re-decomposes — strictly better than the old design, whose cached
tessellations were silently stale across DPI changes.

All coordinates below are in device pixels (`ScaledPixels`) at snap time;
the decomposition itself lives in `Pixels` space.

1. **Normalize** the path into contours of quadratic Bézier segments.
   Lines become degenerate quadratics (control point at the segment
   midpoint). Cubics and arcs are degree-reduced to short quadratic chains
   via `lyon_geom` (`CubicBezierSegment::for_each_quadratic_bezier`,
   `Arc::for_each_quadratic_bezier`) — 2–4 quadratics per cubic; far
   coarser than line flattening, and resolution-independent.
2. **Monotone split**: split every quadratic at its x- and y-extrema
   (`lyon_geom`'s extrema/monotone-range helpers), so each piece is
   xy-monotone. Monotone pieces have endpoint-derived bounding boxes, a
   single root for any horizontal line, and well-ordered behavior under
   the sweep.
3. **Sweep** in y with events at piece endpoints. Maintain the active edge
   list; resolve winding per span using the path's fill rule (nonzero and
   even-odd). Where active edges cross, split both at the intersection —
   intersection numerics come from `lyon_geom`
   (`cubic_bezier_intersections_t` on elevated quadratics, or robust
   bisection on monotone pieces). The sweep's *bookkeeping* is ours; its
   *numerics* are not. This curve-aware sweep is the hardest module in the
   design; see Validation.
4. **Snap slab boundaries to pixel rows.** Event y-coordinates split
   trapezoids at the *pixel row boundary* containing them, not at the
   fractional event y. Consequence: no pixel is vertically straddled by two
   stacked trapezoids of the same path, which eliminates the classic
   "hairline seam at every vertex height" artifact of naïve trapezoid
   renderers. Each trapezoid records its true fractional top/bottom
   (`y_top`, `y_bottom`) in addition to its snapped row range.
   **Never split a trapezoid within a pixel row.**
5. **Classify junctions** where the bounding edge changes identity mid-row:
   - *Tangent-continuous joints* (extrema splits, degree-reduction seams):
     the two pieces are locally collinear; the row is emitted as an
     ordinary trapezoid using either piece, with error far below the
     coverage epsilon. No patch.
   - *Sharp corners* (true path vertices): emit a **corner patch** — the
     same instance format carrying both incident edges for that side, so
     the vertex pixel's coverage is computed exactly in registers. Corner
     patches are part of the design, not an optional rung.
6. **Emit one instance per trapezoid/patch**: snapped row range, fractional
   y-extents, left edge control points, right edge control points, plus the
   path's paint (existing `Background` — solid or gradient), content mask,
   and draw-order metadata.

Scratch-buffer discipline is mandatory: pre-sized arenas reused across
frames (the pattern of `FrameScratch` in the D3D11 renderer), no per-frame
`Vec` churn. Paths are independent, so the emitter must be shaped so
per-path parallelism is a scoped-pool call away, but do not add parallelism
until a measurement asks for it.

### Why the sweep carries curves — ranked honestly, with a designated retreat *(superseded — the race ran and lane B won; see round four)*

**This section is preserved for its reasoning, but the drag race it
specifies was run (round four) and the retreat position is now the
primary design: fills are flattened at τ = 0.25 px in `PathBuilder`
before the unchanged decomposer.**

The fill sweep ran on monotone quadratic pieces rather than flattened
lines. Three reasons, in true order of importance — recorded because the
AA framing ("exact coverage of the true curve") tends to absorb credit
that belongs elsewhere:

1. **Scale-independence** (load-bearing). Flattening requires a
   tolerance, and a tolerance is meaningful only at a scale. Carrying
   curves is what makes the decomposition valid at any DPI and lets a
   scale change re-snap without re-decomposing. Honesty required by the
   survey: no external consumer caches built `Path`s today, so this
   property currently protects internal callers and a caching future,
   not measured workloads.
2. **Segment count** (measured). Flattening multiplies segments ~3–4×
   for curved content, and sweep cost is O(segments + events) — it
   multiplies the most expensive CPU phase, plus instance count and
   snap cost downstream.
3. **AA quality** (real but the weakest). Exact-curve coverage is the
   best possible answer, but exact coverage of a tightly flattened
   polygon is nearly indistinguishable: the error is bounded by the
   flattening tolerance (edge-of-perception at τ = 0.25 px, gone at
   0.1 px), and Vello, Pathfinder, Skia, and Direct2D all flatten —
   "exact coverage of lines" *is* the industry quality bar. The doc's
   headline quality wins (no MSAA quantization, no temporal shimmer)
   come from analytic coverage, not from curves.

What curves cost is equally concrete: curve–curve intersection is the
sweep's hardest numerics (both round-one fill bugs live there — it is
`lyon_geom`'s numerically weakest case), and the fragment integral's
root solves are most of its ~30–40 ALU.

**The flattened-sweep retreat position.** If the posture-rule bug work
(conservative splitting + self-healing; see Prior art) does not cheaply
contain the curve-intersection fragility, the designated fallback is
*not* better numerics and *not* lyon: it is **flatten before the sweep
and keep everything else identical**. Lines are already degenerate
quadratics throughout, so the chains, snapper, instance format, shader
modes, and stroke pipeline all survive unchanged. What it buys:
segment–segment crossing tests with **exact-at-float-speed predicates
from the `robust` crate** (Shewchuk's adaptive-precision predicates —
provably correct topology, the thing WPF spent 1,073 lines of bignums
approximating structurally), and a trapezoid fragment mode that drops
from ~30–40 ALU with root solves to ~10–15 ALU of closed form. What it
gives up: reasons 1 and 2 above — re-decomposition on scale change and
the ~3–4× segment multiplier. Against this project's stated values
(simplicity, library reliance, no in-house computational geometry) the
retreat is *more* aligned than the current design; the current design
holds on performance and scale-independence until measurement says
otherwise.

**The drag race (decides which design is primary).** Lane B costs ~20
lines to stage: flatten fills in `PathBuilder` (kurbo, τ = 0.25 px)
before the unchanged decomposer. It measures flattening's cost side
only — the benefits (robust predicates, the simpler shader) stay
theoretical until committed — so **a near-tie is a win for lane B**.
Rows: rounded-rect selection (dominant internal fill), spinner arc
outline (curvature density), 10-vertex star (pure lines — lanes must
tie; sanity check), chart area fill with `StrokeStyle::Natural`
smoothing (gpui-component's curve-heavy per-frame rebuild), 1,000-point
scribble outline (adversarial event density), and both round-one
bug-repro scenes (does near-tangent corruption even trigger on line
input?). Metrics: decompose time, snap time, instance count, visual
diff at 1× and 4×. **Decision rule, fixed before running:** lane B
within ~1.5× on the realistic rows → flattening becomes the primary
design and the curve sweep is retired; beyond 2× on the chart/scribble
rows → curves stay and the retreat clause stands; in between, argue.
Race before writing the pivot's HLSL: if lane B wins, the trapezoid
mode loses its root solves and the shader work shrinks.

### Strokes and dashes *(revised twice — this section describes the final design)*

**Original design (do not implement):** convert strokes to fills via
`kurbo::stroke` and run them through the fill pipeline ("one pipeline for
all path content"). The prototype measured this at ~100× the old
renderer's per-paint stroke cost (4.4 ms vs 44 µs for a 1,000-point
scribble), because it converts an inherently *local* problem (thicken a
line) into an inherently *global* one (compute the union of a
self-touching 3,000-segment outline). Worse, stroke outlines are the
sweep's most adversarial input class — dense near-tangent self-contacts,
2 px-tall wiggly ribbons with constant span-topology churn — and were the
source of every observed rendering glitch. Every mature 2D stack (lyon,
Skia) separates strokes from fills for exactly these reasons.

**Second revision (built, measured, superseded):** one convex capsule per
flattened centerline segment, each decomposed into chains — first by a
local sweep (1.13 µs/segment: generic sweep machinery on statically-known
answers), then by direct template stamping of rectangle + circle chains
(0.38 µs/segment). Correct, glitch-free, and still ~10× the old
renderer's per-segment constant, because it kept translating a brush into
the *region representation* (pieces, chains, runs, row snapping) so the
fill shader could draw it.

**Final design — strokes never become regions at all.** A stroke is a
polyline plus a radius, and that representation survives to the GPU:

- `PathBuilder::build` applies dashing (`kurbo::dash`) and flattens the
  centerline (`kurbo::flatten`, fixed tolerance in `Pixels`, once at
  build) into segment pairs stored in the path's decomposition — the
  stroke variant of `PathDecomposition` is just `{ segments, radius }`,
  scale-independent like the fill variant.
- Per paint, each segment becomes **one instance** in the *same* instance
  struct, buffer, and pipeline as fill trapezoids (a stroke segment's
  `{p0, p1, radius}` fits with room to spare; a mode flag lives in the
  existing padding field). The fragment shader branches per instance —
  coherent, since all fragments of an instance take the same branch:
  - **mode 0, trapezoid**: the exact area integral (fills, unchanged);
  - **mode 1, capsule**: distance to segment (project + clamp), round
    ends inherent;
  - **mode 2, oriented box**: flat ends, for butt caps.
- Stroke coverage uses the **exact 1D slab formula**, not a boundary
  ramp or smoothstep: both the centerline distance and the radius are
  known, so cross-section coverage is
  `clamp(min(t+r, 0.5) − max(t−r, −0.5), 0, 1)` — exact in the normal
  direction at every width, which keeps sub-pixel hairlines at correct
  weight (the classic "ropey hairline" SDF artifact never appears).

**Caps and joins are data, not geometry.** Round caps: inherent to the
capsule (and now exact circles, not tessellated approximations). Round
joins: consecutive capsules overlap at the shared point — no join
geometry exists. Butt caps (lyon's default, which gpui-component relies
on everywhere): box mode, plus zero-length capsules (= circles) at
interior joints. Square caps: box mode with endpoints extended by `r` on
the CPU — no third shader mode. Miter/bevel joins degrade to round until
a real caller complains. Each dash from `kurbo::dash` is simply one
instance.

The artifact contract is structural, not policy: fills are exact because
their cells are disjoint; strokes double-blend where translucent strokes
overlap themselves (joins, self-crossings) — the contract every prior
design shipped too. No runtime dispatch on paint properties: the
fill/stroke fork is decided by which builder the caller used.

Why this is the terminal stroke design and not a fourth guess: each prior
fix moved strokes further from the region representation, and this is the
fixed point — the representation *is* the brush. It is also how every
shipping UI renderer draws thin strokes, and how GPUI itself already
draws them: `underline_fragment` in `shaders.hlsl` is a gradient-
normalized distance-to-curve stroke renderer, and the dashed-border quad
path ships `dash_alpha()`. The capsule mode brings paths in line with the
renderer's incumbent technique; the only novel fragment math in the
whole design is the fill integral, confined to the one place nothing
else can do the job.

### GPU side: one instanced draw in the main pass

Trapezoids draw inside the main render pass, in draw order, with the
standard premultiplied blend state — exactly like quads. A `Paths` batch
becomes a single instanced draw over its trapezoids. Nothing ends the
encoder. Batch depth uses the same collapsed-viewport-range mechanism as
every other non-quad batch in the depth design.

Because lines are degenerate quadratics, there is **one pipeline and one
instance buffer** for all path content. The fragment shader has one
per-instance mode branch (trapezoid integral / capsule / box — see
Strokes); within the fill mode there is no per-edge branching and no
line/curve permutation.

**Vertex shader:** expands each instance to a screen-space quad:
`y ∈ [row_start, row_end]`, `x ∈ [min_x(L) − pad, max_x(R) + pad]` with
`pad = 1` (edge x-extremes come from endpoints; edges are xy-monotone).
Clipping against the content mask uses the existing clip-distance approach.

**Fragment shader:** for the pixel's covered y-interval
`[y0, y1] = [max(y_top, py), min(y_bottom, py + 1)]`, compute for each edge
the curve parameters `t0, t1` at `y0, y1` — one stable quadratic root solve
each (single root; monotone) — then the exact area between the edges,
clamped to the pixel column:

```
area(E) = ∫ clamp(x_E(y), px, px + 1) dy   over [y0, y1]
        = ∫ clamp(x(t), px, px + 1) · y'(t) dt   over [t0, t1]
cov     = clamp(area(R) - area(L), 0, 1)
color   = background_eval(...) * cov              // premultiplied
```

The unclamped integrand is a quartic polynomial in `t` with closed form;
the clamp is handled by splitting at the (at most two) parameters where
`x(t)` crosses the column boundaries — additional stable root solves
against a monotone piece. Estimated cost ~30–40 ALU per fragment: an order
of magnitude above a flat quad fill, an order of magnitude below banded
winding evaluation, and paid only on trapezoid-covered pixels.

The result is **exact box-filter coverage of the true curve**. Every
artifact of flattening tolerance and MSAA sample quantization is gone,
including the temporal shimmer of slowly moving edges crossing MSAA sample
positions (analytic coverage is continuous in position — visibly smoother
on rotating spinner arcs).

**Corner patches:** same shader with two edges per side; per-side area is
the min/max composition of the incident edges' integrals, combined in
registers before the single blend.

### Interior optimization (two tiers)

Trapezoid rows strictly between fractional extents and away from both
edges have coverage exactly 1. WPF's vertex layout embodies this split
(fringe / flat-1 interior / fringe, with `NeedInsideGeometry()`); ours
comes in two tiers:

**Tier 1 — shader early-out (~4 lines of HLSL, no CPU change; build
with the first shader work).** The vertex shader computes a conservative
interior x-slab per instance — max-x of the left edge, min-x of the
right, from the monotone edges' convex-hull bounds (endpoints + control
point, a few min/max ops across 6 vertices) — passed as a flat
interpolant. The fragment shader tests two comparisons and skips the
integral with `coverage = 1` on interior rows. Wave divergence is the
usual objection and it is benign here: the expensive branch is pure ALU
(no fetches), so a straddling wave pays the integral plus ~3 ops — the
status quo plus noise — while any trapezoid wider than a couple of wave
tiles (≈16–20 px) yields uniform all-interior waves that skip straight
through. The win case (selections, panels, chart fills) is exactly the
large-area content the weak-GPU gate worries about; the lose case does
not exist.

**Tier 2 — CPU opaque-core split (deferred until the weak-GPU gate
demands it).** Split wide interior spans into an opaque core instance
(drawable through the flat opaque pipeline, eligible for depth-write in
the opaque prepass someday) plus thin analytic edge strips. Strictly
stronger — zero divergence, occlusion power — but costs +2 instances
per wide trapezoid and ~40 lines of snapper code. Not required for the
first landing.

## Quality

Relative to today (flattened lyon + 4× MSAA via intermediate), this design
is equal or better on every axis:

- **Straight edges**: 256 coverage levels versus MSAA's 5.
- **Curves**: rendered from true quadratics with exact area coverage —
  strictly better than flattened chords under 4-sample approximation, at
  every scale and DPI.
- **Thin features**: a sub-pixel-wide feature has both edges in the same
  invocation; coverage is exact where MSAA quantizes and fringe methods
  collapse.
- **Sharp corners**: exact via corner patches.
- **Temporal**: continuous coverage; no sample-crossing shimmer.

**Stroke AA (distance model), quantified.** The deviation of the slab
ramp from exact box-filter coverage is a pure function of edge angle:
zero for axis-aligned edges (the ramp *is* the box filter there), maximum
≈4.3% coverage (≈11/256 levels) at 45°, where exact coverage has
quadratic tails to ±0.71 px and the ramp cuts at ±0.5 px. Compare MSAA's
worst case: 12.5% *quantized* error that steps and shimmers. The slab
formula makes sub-pixel stroke widths exact in the normal direction
(critical at 1.25×/1.5× DPI where every 1 px chart line is fractional).
One capsule per segment has **no interior seams at all** — unlike any
chain decomposition of a stroke — and caps are exact circles. Ranking on
strokes: capsule ≥ stamped chains > MSAA. Consequence to state plainly:
the renderer has two AA models, so a stroke laid exactly on an
equal-width fill edge can differ by ≤4% coverage at diagonals — already
true between quads and paths today, and strokes now match quads exactly,
which is the consistency a UI toolkit actually wants.

Known artifact classes, stated honestly:

- **Sub-pixel gaps:** two spans of the same path separated by less than a
  pixel conflate at the shared pixel (thin *holes*, the dual of thin
  features — each side blends separately). Same artifact class exists today
  via MSAA quantization.
- **Abutting separate paths** (shared exact edge, e.g. adjacent chart bars):
  analytic coverage multiplies where MSAA's samples partition, so a
  hairline of background shows through. Inherent to every analytic method
  (including Slug between paths). Shapes within one path cannot seam.
- **Translucent stroke self-overlap** double-blends at joins and
  self-crossings (structural stroke contract; abutting butt-capped
  segments of one stroke instead risk the hairline class above).

## Performance model

**CPU:** the emitter replaces both lyon flattening *and* lyon
triangulation. Segment counts drop ~3–4× for curved content and become
resolution-independent; instance output is smaller than today's triangle
vertices. The expectation is that path CPU cost goes **down** relative to
today. **Kill criterion: if the emitter exceeds 1.2× today's lyon
tessellation cost on the selection-churn and chart benchmark scenes, the
claim is void and the design must be reassessed.** Scaling class is
O(segments + events) — content-scaled.

**GPU:** ~30–40 ALU on trapezoid-covered pixels versus today's per-batch
full-screen clear + 4×-resolve + restart, which are resolution-scaled and
mostly ceremony. Note the deliberate inversion: this design does **more
math per fragment but less work altogether**, and what remains sits in the
column (content-scaled GPU ALU) that pipelines, rather than the columns
(main-thread latency, full-screen bandwidth) that don't.

**Weak-GPU gate:** every prior failure in this subsystem was a fragment- or
bandwidth-cost surprise on weak GPUs. Before commitment: measure on Adreno
(D3D11, binning) and an Intel Mac. This gate is not optional.

### Adversarial scenes (required benchmark rows)

| Scene | Today | This design |
|---|---|---|
| 5000 overlapping stars (one batch) | 1 full-screen clear+resolve+restart | ~5000 × star-area fragments, zero restarts |
| 5000 stars with a quad between each (5000 batches) | 5000 full-screen cycles (~5 fps historically) | same fragments, zero restarts, zero extra memory |
| Full-screen selection | full-screen cycle | mostly interior trapezoids (coverage 1); near-free with the opaque-core optimization |
| Selection churn while typing | flatten + retessellate per frame | emitter per frame (CPU kill criterion applies) |

The quad-interleaved star scene — the scene that defeats both the current
design (restart count) and any atlas design (simultaneous memory) — is
simply N more instanced draws here. This is the design's reason to exist.

## What gets deleted

- Full-screen path intermediate + MSAA textures, all backends
  (−166 MB at 4K on D3D11; −99–166 MB on wgpu; −166 MB on Intel Macs).
- All mid-frame render pass restarts (paths are the only source). The main
  pass becomes a single encoder unconditionally, which makes memoryless
  depth (Clear/DontCare) unconditional on Metal in the depth design, with
  no cross-encoder replay needed.
- Path-sprite composite pipelines and shaders, all backends.
- The disjoint/union composite-rect logic, currently triplicated across
  backends (the #35688 bug class ceases to exist — nothing is composited
  from anywhere, so nothing can be composited twice).
- `PATH_SAMPLE_COUNT` / MSAA pipeline permutations and the Intel MSAA
  performance contingency they encode.
- Curve flattening and its resolution-scaled CPU cost.
- The hand-rolled dash sampler in `path_builder.rs` (replaced by kurbo).
- `lyon_tessellation` as a dependency (we keep `lyon_path`/`lyon_geom`).

## Library leverage

The bespoke core is deliberately small: sweep bookkeeping over non-crossing
monotone pieces, pixel-row snapping, junction classification, instance
emission, and one fragment shader. Everything else leans on maintained
libraries:

| Concern | Library | Status |
|---|---|---|
| Path building/representation | `lyon_path` | already a dependency |
| Degree reduction (cubics, arcs → quadratics) | `lyon_geom` | already a dependency |
| Monotone splitting, extrema, root solving | `lyon_geom` | already a dependency |
| Curve–curve intersection numerics | `lyon_geom` | already a dependency |
| Dashing, centerline flattening (strokes) | `kurbo` | new; small, Linebender-maintained |
| Ground-truth rasterization (tests) | `tiny-skia` + `zeno` | dev-dependencies only |
| Winding pre-resolution (optional) | `flo_curves` | benchmark-gated; default **no** |
| Exact segment orientation predicates | `robust` | only with the flattened-sweep retreat (lines only — no curve equivalent exists) |

Guiding rule: **numerics from libraries, orchestration ours.** The code we
maintain should be readable geometry bookkeeping, not root-finding — the
"did I handle the near-double-root case" category belongs to `lyon_geom`.

`flo_curves` (`path_remove_interior_points`) could pre-resolve
self-intersections curve-preservingly, letting the sweep assume
non-crossing edges and deleting the crossing-split code outright. It is an
option, not the plan: it is a single-maintainer crate built for editing
operations, and its per-frame throughput on the selection-churn benchmark
is unproven. Adopt only if the benchmark says the crossing-split code is
not worth owning.

## Validation plan

Because this design lands whole (no line-only intermediate stage), the
current renderer cannot serve as pixel-exact ground truth — un-flattened
curves legitimately differ from flattened ones. **Ground truth is a pair
of external oracles**: `tiny-skia` (Skia's scan converter, both fill
rules) cross-checked against `zeno` (exact-area accumulation), both as
dev-dependencies, with the comparison epsilon absorbing their internal
flattening tolerance. A ~50-line hand-written exact-area checker covers
micro-cases with closed-form answers (single trapezoid, axis-aligned
shapes), so that when the fuzzer finds a disagreement there is a third
oracle whose every line we understand. The oracle harness is built
*first*, before the emitter.

**Property tests (CPU, the load-bearing suite):**
- Partition validity: emitted trapezoids of a path are pairwise disjoint;
  total area equals the path's analytic area within epsilon.
- Coverage correctness: random paths (including self-intersecting,
  multi-contour, both fill rules) compared per-pixel against the reference
  rasterizer; delta within bound everywhere except declared artifact
  classes (sub-pixel gaps).
- Sweep robustness: the curve-aware sweep (crossing detection, bisection
  splitting) is the highest-risk module; fuzz it with adversarial inputs —
  near-tangent crossings, coincident edges, degenerate quadratics, extrema
  at row boundaries.
- Snapping invariants: no pixel row intersects two stacked trapezoids;
  fractional extents lie within the snapped range; junction classification
  never emits a split within a row.

**Visual tests (`render_to_image`), pinned before any backend work:**
current renderer output for: overlapping translucent same-order paths
(#35688's selection case), mixed-order overlapping paths, gradient fills
(Oklab stops), content-mask clipping, paths at fractional positions,
dashed strokes, and the spinner arc. A/B against the trapezoid renderer
with perceptual thresholds that encode both the artifact budget and the
expected (superior) curve rendering.

**Benchmarks:** the four adversarial scenes above, plus `paths_bench` /
the `painting` example (the historical benchmark this subsystem is judged
by), on: RX 6600 (D3D11), Adreno 690 (D3D11, binning), an M-series Mac,
and an Intel Mac if obtainable.

## Migration

1. **Oracle harness + property tests.** Wire up the `tiny-skia`/`zeno`
   oracles and the exact-area checker. Platform-independent, headless, no
   GPU. This is the foundation everything else is judged against; it lands
   first.
2. **The emitter**, including the curve-aware sweep, developed entirely
   against the property suite. The riskiest code in the project ships with
   the strongest tests and no rendering dependencies.
3. **One backend** (Metal — `render_to_image` gives the cheapest validation
   loop) behind a global env-var switch. The switch is a temporary revert
   lever, not a per-scene fallback: one code path runs at a time, both are
   exercised in CI, and the old path plus the switch are deleted together
   after a release cycle of nightly/preview soak.
4. **Port to wgpu and D3D11** (the backend surface is small: one pipeline,
   one instance buffer; the emitter is shared).
5. **Delete** the intermediate machinery and flattening. Re-run the memory
   ledger and publish before/after in the PR.
6. **Seal `Path`.** The used external API surface (`PathBuilder`
   constructors and segment methods, `build()`, `paint_path`) survives
   this redesign byte-for-byte — the survey found zero external uses of
   anything else. `Path`'s bare `pub` fields predate us and are a
   liability; make them private (accessors for what tests need) while
   the internals are already churning, so the next internal change
   breaks nobody.

This subsystem has been reverted twice (#34722; and #34992 replaced the
atlas). The kill switch, the pinned visual tests, the reference-rasterizer
property suite, and the weak-GPU gate are the institutional memory of those
events. Do not ship without them.

## Open questions

- ~~**kurbo glue and output form.**~~ *Resolved by the prototype:
  `kurbo::stroke` is not used at all — strokes go through the capsule
  synthesizer (see the revised Strokes section). kurbo is used only for
  dashing and centerline flattening; the lyon↔kurbo glue is a leaf.*
- **The manual `Path::curve_to` / `push_triangle` API** predates
  `PathBuilder` and emits Loop-Blinn triangles directly. The trapezoid
  renderer consumes contours, not triangles. Either convert this API to
  build contours internally, or deprecate it in favor of `PathBuilder`.
  *Partially resolved:* the gpui-component survey (second-round findings)
  found zero uses — every call site goes through `PathBuilder`. Other
  external users still unsurveyed.
- **Cubic→quadratic tolerance.** Degree reduction has its own tolerance;
  it is geometric (curve deviation), not pixel-tied, but should be chosen
  so worst-case deviation stays below the coverage epsilon at maximum
  realistic scale. Write the bound down in code comments.
- **Gradient evaluation** in the trapezoid fragment shader should reuse the
  quad shader's `Background`/gradient machinery; verify color-space
  interpolation (Oklab) parity with the current path pipeline, which
  gamma-handles via the intermediate today.
- **Precision:** f32 device-pixel coordinates are exact for the snapped
  integer rows and comfortably precise for edge evaluation at UI scales;
  confirm no issues at 8K + 3× scale (worst realistic case ~7k device
  pixels; f32 has 23 mantissa bits; fine, but write the one-line proof in
  code comments). The quartic integral should be evaluated in a
  pixel-local frame (`x − px`, `y − py`) to keep terms well-conditioned.

## Findings from the Windows prototype, round one (July 2026)

A full D3D11 implementation was built and measured. What follows was
accurate at the time; **the remaining plan at the end of this section is
superseded by round two below**, which replaced the stroke approach
again. The requirements still stand, with requirement 2's "shared
consumer contract" now meaning the shared instance buffer and pipeline
rather than chains.

### What existed at the time (see round two for current branch state)

- `crates/gpui/src/path_trapezoids.rs` — fill decomposer (monotone split,
  grid-broad-phase crossing split, sweep with identity-based span
  matching, chain arena), `PathDecomposition` (scale-independent, computed
  at build), `TrapezoidSnapper` (per-paint row snapping), instance types.
  Includes timing tests (`cargo test -p gpui --release --lib
  path_trapezoids -- --nocapture`).
- `crates/gpui/src/scene.rs` — `Path` stores quadratic contours + an
  `Arc<PathDecomposition>`; `PathVertex`/`push_triangle`/Loop-Blinn
  removed; scene stores `PathTrapezoid` instances batched like quads.
- `crates/gpui/src/path_builder.rs` — no lyon tessellation; fills convert
  to contours; strokes at that point still went through `kurbo::stroke`
  into the fill pipeline (since replaced; see round two).
- `crates/gpui_windows/` — one instanced trapezoid pipeline in the main
  pass; exact-coverage fragment shader; the intermediate texture, MSAA
  texture, pass restarts, and both path pipelines deleted (≈350 lines).
- macOS/Linux/wgpu backends do not compile on this branch.

### Measured results (release, 2× scale)

| Workload | Old renderer (per paint) | Prototype: decompose (build) | Prototype: snap (per paint) |
|---|---|---|---|
| Editor selection (rounded rect) | lyon fill, µs-class | 1.1 µs | 22 ns |
| Spinner arc (stroke) | lyon stroke, µs-class | 1.2 µs | 53 ns |
| 10-vertex star | lyon fill | 1.4 µs | 36 ns (5 instances) |
| 1,000-pt scribble, stroked | 44 µs (`StrokeTessellator`) | 4.4 ms via fill pipeline (the bug); lyon `FillTessellator` on the same outline: 3.3 ms | 24 µs (2,354 instances) |

GPU side validated visually (blending, gradients, fill rules, multi-contour
holes, dashes); the one inherent artifact class observed — hairline between
abutting separate paths — is the declared one. The 2,000-star `paths_bench`
scene renders with zero pass restarts.

### What this design got wrong (corrected above)

1. **Strokes-as-fills** (§Strokes, revised): 100× too slow and the source
   of all observed glitches. Strokes and fills are different questions
   (local thickening vs global winding) and need separate producers
   feeding the shared chain/snapper/shader contract.
2. **Per-frame emission** (§CPU side, revised): the built `Path` is a
   load-bearing amortization boundary. Decomposition is scale-independent
   and belongs at build time; only row snapping (≈1% of cost) is per-paint.
3. **"Deliberately small bespoke core"**: the honest ledger is ≈1,700
   lines today (sweep ≈1,000 incl. tests, chains/snapper ≈300, shader
   ≈200) plus ≈150 for the synthesizer and a dev-only oracle harness.
   Owning this is viable only with the simplicity requirements below.

### Hard-won implementation lessons

- **Span matching must be combinatorial, not geometric.** A span continues
  iff the same two piece IDs bound it (a piece bounds at most one span per
  slab). The original interval-overlap-with-slop matching misfired
  constantly (31,397 chains where the topology had 451) — it was a
  heuristic doing an exact job. Geometry is consulted only for the residue
  at true events.
- **MSAA's real subsidy was seam-free abutment** (sample partitioning).
  Analytic coverage multiplies across abutting instances. Disjointness
  within a chain comes from row snapping; strokes get overlap-by-
  construction instead.
- **Known bugs, with causes** (both exercised almost exclusively by stroke
  outlines; the synthesizer makes that input class extinct, but fills can
  still hit them and the oracle harness must cover them):
  1. Topology-event handoff snaps chain ends to `floor(y)`; for thin
     near-horizontal geometry this amputates coverage (thin/gray
     horizontal runs) and can annihilate sub-row chains entirely
     (dropouts) via the `y_end − y_start < ε` guard in `emit_chains`.
  2. Missed near-tangent crossings corrupt the incrementally-maintained
     active-list order *persistently* (the per-slab re-sort of the naive
     sweep self-healed after one slab), producing long horizontal chords.
- **The ecosystem cannot supply the kernel.** Libraries offer triangles
  (lyon — forces the intermediate/MSAA taxes back), masks (tiny-skia/zeno
  — the removed atlas design), or whole renderers (vello, Skia — a merger,
  not a dependency). Analytic-coverage primitive generation is
  renderer-internal everywhere it exists. Watch item: Linebender's sparse
  strips (`vello_cpu`/`vello_hybrid`) target exactly this split and would
  be the dependency to adopt if it matures.

### Requirements (agreed July 2026)

1. **Simplicity first.** GPUI is not "about" paths and assumes no
   computational-geometry expertise. Numerics come from reputable
   libraries (`lyon_geom`, `kurbo`); only orchestration/bookkeeping lives
   in-tree. Performance within an order of magnitude of the old renderer
   on all workloads — no further optimization of the fill decomposer
   without a measurement demanding it (it is at lyon-fill parity already,
   and genuine fills are µs-class).
2. **Separate stroke and fill producers**, one shared consumer contract
   (chains → snapper → single GPU pipeline).
3. **Decomposition at build, snapping at paint.** The built `Path` owns
   its decomposition; no keyed caches, no eviction machinery.
4. **No runtime policy dispatch.** Artifact contracts are structural
   (disjoint vs overlapping chains), never conditioned on paint
   properties like alpha.
5. **Trust by mechanical verification.** The tiny-skia/zeno oracle
   property harness (dev-only) is the substitute for in-house geometry
   expertise; the sweep is not modified without it.

### Remaining plan *(superseded — see round two)*

1. ~~Stroke synthesizer (capsule chains via local convex sweeps).~~
   Built, measured, and superseded; see below.
2. Oracle harness (dev-only), then fix the two sweep bugs above for the
   fill-only input class. *(Still current.)*
3. Re-validate quality and perf in the `painting` example.
4. Port the backend surface to Metal and wgpu.
5. Survey external users. *(Done for gpui-component; see round two.)*

## Findings, round two: the stroke representation pivot (late July 2026)

Round one's remaining-plan item 1 was implemented **twice** on this
branch, measured, and the whole strokes-as-chains approach was then
superseded by a decision to render strokes as distance-coverage
instances. This section is the authoritative record: what exists on the
branch right now, the measurements that forced the pivot, the external
consumer survey, the decided design, and the implementation plan.

### Branch state (what exists right now)

- The fill pipeline: unchanged from round one, still correct and fast
  (`selection` decompose 1.0–1.1 µs at build, snap 22 ns/paint). The
  fill decomposer (`Decomposer` in `path_trapezoids.rs`) is byte-for-byte
  identical to round one — the stroke work below never touched it.
- `StrokeSynthesizer` in `path_trapezoids.rs`: the *second* stroke
  implementation — no sweep; stamps one rectangle chain per centerline
  segment plus circle chains at polyline points (shared join/cap
  circles), from a screen-axis-aligned unit-circle template built with
  `lyon_geom::Arc`. Correct (tests `stroke_stamp_area`, `emit_long_stroke`,
  `emit_ui_shapes`, `emit_star` pass; fmt/clippy clean), glitch-free, and
  **slated for deletion by the pivot below**.
- `path_builder.rs`: strokes go `kurbo::dash` → `kurbo::flatten` →
  synthesizer (`synthesize_stroke`); the round-one `kurbo::stroke`
  expansion path is deleted. Fills unchanged.
- `scene.rs`: `Path::set_decomposition` (crate-visible) and a
  `#[cfg(test)]` getter; `Path` remains one primitive type.
- Full gpui lib test suite not yet re-run to completion (interrupted);
  path_trapezoids tests + clippy are green.

### The stroke cost trajectory (1,000-point scribble, release, 2×)

| Design | Build | Per paint | Notes |
|---|---|---|---|
| Old renderer (lyon `StrokeTessellator`) | — | 44 µs | rebuilt every paint |
| Strokes as fills (`kurbo::stroke` → global sweep) | 4.4 ms | 24 µs snap | + both glitch classes |
| Capsules via local sweeps | 1.13 ms | 94 µs snap | generic sweep on statically-known answers |
| Stamped rect/circle chains (current branch) | 0.39 ms | 71 µs snap | no sweep; still ≈10× old per segment |
| **Stroke instances (decided, unbuilt)** | **≈30–60 ns/seg** (dash/flatten dominated) | **≈25 ns/seg** | at or below the old renderer on every workload |

The trajectory is the argument: every fix moved strokes further from the
region representation, and the remaining 380 ns/segment had no algorithm
left to remove — it was purely the cost of translating a brush into
pieces/chains/runs so the fill shader could consume it. The fixed point
is to stop translating. Spinner-arc stroke build: 11.3 µs (capsule
sweeps) → 3.1 µs (stamps); expect ≪1 µs as instances.

### External consumer survey: gpui-component

15 `PathBuilder` sites: ~11 stroke, 4 fill. Findings that shaped the
decision:

- **Build-per-paint is universal.** No call site caches a built `Path`;
  charts, separators, and editor overlays rebuild from raw data inside
  `paint()`/`prepaint()`. For this consumer the build constant *is* the
  per-frame cost; the build/paint amortization boundary helps them only
  if they adopt caching (they haven't).
- **Tiny strokes dominate by count**: one path per grid line, per
  candlestick wick, per axis; separator dashes. Per-path fixed overhead
  matters as much as per-segment cost.
- **Charts default to `StrokeStyle::Natural`** (Catmull-Rom cubics) —
  polyline strokes hit the flattener and multiply.
- **lyon's default cap is Butt and they rely on it everywhere.** The
  branch's round-everything silently changed rendering: dashes lengthen
  ≈1 px per end (4–2 dash patterns visibly tighten), and indent guides
  — abutting per-line vertical segments at **alpha 0.85** — would show
  periodic double-blend dots at line boundaries. Butt caps are mandatory,
  and in the instance design they are the *cheapest* mode (a grid line or
  wick = exactly one box instance).
- **Zero uses of the removed `push_triangle`/Loop-Blinn API.**
- Repro assets: `crates/story/examples/brush.rs` is a `painting.rs`
  clone (freehand + per-frame grid); chart stories exercise the rest.

### The decided design (see revised Strokes section for full detail)

Strokes render as **capsule/box coverage instances in the existing
instance struct, buffer, and pipeline** — `Path` stays one primitive
type; no scene or batching changes; backends gain two small fragment
modes, not a pipeline. Key commitments:

1. `PathDecomposition` becomes a two-variant enum: `Fill` (pieces,
   chains, run arena — unchanged) and `Stroke { segments, radius }`
   (flattened, dashed centerline in `Pixels`; scale-independent).
2. Instance mode flag in the existing `pad` field: 0 = trapezoid
   (integral), 1 = capsule, 2 = oriented box. Per-instance branch,
   coherent. ≈15 lines of HLSL per mode; reuses `gradient_color`,
   `distance_from_clip_rect`, and the blend path as-is. In-tree
   precedent: `underline_fragment` (gradient-normalized distance-to-curve
   stroke) and `dash_alpha` (AA'd dash modulation for quad borders).
3. **Exact 1D slab coverage** for strokes —
   `clamp(min(t+r, .5) − max(t−r, −.5), 0, 1)` — never a boundary ramp
   or smoothstep, so hairlines keep exact weight. AA bounds vs box
   filter: 0% axis-aligned, ≤4.3% at 45° (vs MSAA's 12.5% quantized).
4. **Caps honor `StrokeOptions`**: butt (default) = box instances +
   zero-length-capsule circles at interior joints; round = capsules
   (joins free via overlap); square = CPU-extended boxes. Miter/bevel
   joins degrade to round. Each dash = one instance.
5. `Path` records the stroke width it was built with, so
   `ensure_decomposition` after mutation re-synthesizes the stroke
   variant instead of misreading the centerline as fill contours; the
   builder dilates `Path::bounds` by the radius (bounds no longer come
   from outline contour points, which cease to exist).
6. Ledger estimate: delete ≈280 lines (`StrokeSynthesizer` + stamps +
   contour writing), add ≈130 (enum + snapper arm + builder + shader).
   Net ≈ −150; the deleted code is the bespoke-geometry share.

Stroke instance validation does not need the tiny-skia oracle: capsule/
box coverage checks against brute-force supersampling in a dozen lines.

### Pitfalls (round two — tried or considered; do not repeat)

- **Generic sweep per capsule**: built, measured 3× slower than direct
  stamping (1.13 ms vs 0.39 ms) with zero quality benefit on convex
  input. Do not resurrect “reuse the sweep, it's already correct.”
- **Chains for strokes at all**: even optimal stamping is ≈10× the old
  per-segment constant, inherent to producing pieces/chains/runs/snap
  work per segment. The representation was wrong, not the constants.
- **A separate stroke primitive type**: rejected — splitting `Path` (or
  adding a scene primitive) churns scene/batching/backends. Unify at the
  instance level instead; the churn objection dissolves there.
- **Approximating the fill edges** (ramps instead of the integral):
  incoherent for region pieces — near-horizontal edges degrade without
  bound, thin two-curve slivers have no slab rescue, and true distance
  to a Bézier costs more than the exact integral. Recorded in Background.
- **Shader-side dashing**: viable future rung (one instance per dashed
  straight line, `dash_alpha` precedent), deliberately not built —
  per-dash instances are already ≈2–4 µs per separator, and mid-dash
  cuts at polyline joints would grow spurious caps. Wait for a
  measurement that asks.
- **Judging strokes by `painting.rs`/`brush.rs` without context**: they
  rebuild every stroke every frame, so they measure pure build constant
  and hide the GPU-side wins. They are still the honest benchmark — the
  old renderer degrades on them too, just later — and the instance
  design is expected to move degradation onset *past* the old renderer's
  rather than before it.

### Remaining plan *(superseded — see round three)*

1. Stroke-instance pivot on D3D11. *(Still current; see round three
   for sequencing.)*
2. Re-validate `painting.rs`, `brush.rs`, chart stories; re-run the
   full gpui lib suite (was interrupted). *(Still current.)*
3. Oracle harness, then fix the two fill-sweep bugs. *(Reframed by
   round three's posture rules.)*
4. Port to Metal and wgpu. *(Still current.)*
5. Weak-GPU gate and visual pins. *(Still current.)*

## Findings, round three: prior art and the curves question (late July 2026)

No code was written in round three; it was an analysis round prompted
by two questions — "is trapezoid rendering out of style?" and "how much
of WPF's expertise can we take?" — whose answers changed the Background,
the bug-fix strategy, and the plan's sequencing. Everything below is
folded into the sections above; this is the log of what was learned and
decided.

### Sources examined

- **WPF MILCore** (`dotnet/wpf`, `src/Microsoft.DotNet.Wpf/src/WpfGfx`):
  `core/hw/hwrasterizer.cpp` (trapezoidal AA, 1,484 lines),
  `core/hw/HwVertexBuffer.cpp` (`AddTrapezoidStandard` fringe
  tri-strips), `core/geometry/strokefigure.cpp` (widening, 4,930
  lines), `ExactArithmetic.cpp` (192-bit integers, 1,073 lines),
  `LineSegmentIntersection.cpp` (2,856 lines), `scanner.cpp` (Boolean
  sweep, 4,351 lines). Original team design docs sit in-tree
  (`Scanner.doc`, `BezierReconstruction.docx`).
- **Current GPUI** `origin/main` `scene.rs`: `Path::scale` re-allocates
  and rescales the full vertex vector every paint (the "cached paths
  are free today" premise is false; see Costs).

### What changed in this document as a result

1. **Prior art section** (Background): WPF's architecture, the ways it
   differs, and the three posture rules — conservative decisions, local
   degradation, exactness quarantined (= pay for exactness nowhere,
   since rules 1–2 give the sweep a fallback everywhere).
2. **The fill-sweep bug strategy is reframed** by those rules. Bug #2
   (missed near-tangent crossings) is not a numerics bug to be solved
   with better intersection math — that road provably ends at WPF's
   bignum module. The fix is: (a) open the crossing tolerance so
   detection errs toward spurious splits, and (b) make incremental
   active-list maintenance self-healing (repair order inversions
   locally, bounding any miss to a one-slab artifact). Both are edits
   inside existing functions, not mechanisms.
3. **Triangle-epoch history** (Background): why the industry moved to
   triangles and why the conditions inverted — answers the "out of
   style" question (this is a post-triangle design, not a pre-triangle
   one; the technique family is Direct2D's ancestry on one side and
   Vello/Pathfinder's on the other).
4. **Sweep-output structure** (Background): trapezoids as winding-free,
   left/right-paired RLE of scanline spans — why triangulation of the
   same region, though equally lossless, destroys what the coverage
   integral consumes.
5. **Curves rationale ranked + retreat position** (Design): the honest
   ordering is scale-independence > segment count > AA quality; the
   flattened-sweep retreat (flatten + `robust` predicates, all else
   unchanged) is written down with its trigger.
6. **Drag race** (Design): lane B (~20 lines), rows, metrics, and the
   decision rule fixed before running. Race before the pivot's shader
   work, since a lane-B win simplifies the trapezoid fragment mode.
7. **Interior optimization** became two-tiered (vertex-slab shader
   early-out now; CPU opaque-core split deferred), with the divergence
   analysis recorded (benign: ALU-only expensive path).
8. **Costs** gained the `Path::scale` exhibit; **Migration** gained the
   seal-`Path` step (survey evidence: external consumers touch only
   `PathBuilder` + `paint_path`, which survive unchanged).

### Pitfalls (round three — considered; do not repeat)

- **Borrowing WPF's integer/DDA arithmetic** for the curve sweep: its
  exactness is available only to line edges. Adopting it *is* the
  flattened-sweep retreat, not an upgrade to the curve sweep — there is
  no halfway.
- **Chasing exact curve–curve intersection numerics**: the terminus is
  WPF's `ExactArithmetic.cpp`, and even that is line-only. Fixes to
  sweep robustness are recovery-shaped, never precision-shaped.
- **Treating "resolution-independent" as "zero per-paint work"**: every
  design has a paint-time scale-bound step (today's is `Path::scale`'s
  allocate-and-rescale). The claim to defend is that ours is ~1% and
  allocation-free, not that it is zero.
- **Fearing wave divergence on the interior early-out**: the expensive
  branch is fetch-free ALU, so divergence costs ~3 ops over the status
  quo. Measured objections only.

### Remaining plan *(superseded — see round four)*

1. **Drag race** (Design §"Why the sweep carries curves"): stage lane B
   (~20 lines), run the race card, apply the pre-committed decision
   rule. This decides whether the trapezoid fragment mode keeps its
   root solves before any HLSL is written. *(Done — round four.)*
2. **Implement the stroke-instance pivot** on D3D11 (round two, item 1:
   enum, snapper stroke arm, builder change, capsule/box shader modes,
   cap handling per `StrokeOptions`; delete `StrokeSynthesizer`; add
   the 2-point-stroke and dashed-separator benchmark rows and the
   supersampling coverage check). Include the tier-1 interior early-out
   in the same shader work.
3. **Re-validate**: `painting.rs`, gpui-component's `brush.rs` and
   chart stories; expect flat frame cost and stroke build at or below
   the old renderer. Re-run the full gpui lib suite (interrupted in
   round two, still unconfirmed).
4. **Oracle harness** (tiny-skia + zeno, dev-only), then the two
   fill-sweep bugs *via the posture rules*: over-splitting crossing
   tolerance + self-healing active list (and the event-handoff
   truncation fix for bug #1). If this does not cheaply contain the
   fragility — or the drag race already retired the curve sweep — take
   the flattened-sweep retreat instead of hardening numerics.
5. **Port to Metal and wgpu**: one pipeline + one instance buffer each,
   three fragment modes, delete each backend's intermediate/MSAA
   machinery (≈350-line deletion pattern per backend, per round one).
6. **Weak-GPU gate and visual pins** per the Validation/Migration
   sections — unchanged, still mandatory before shipping.

## Findings, round four: the drag race (late July 2026)

The race specified in §"Why the sweep carries curves" was run on
Windows (release, i7-class desktop). Lane A fed the decomposer
curve-carrying quadratics (the design this document originally argued
for); lane B flattened the same input to lines at τ = 0.25 px in
builder space and ran the *identical* decomposer, since lines are
degenerate quadratics throughout. The harness lives in
`path_trapezoids.rs` as the `drag_race` test (run with `--release
--nocapture`); it times both lanes, snaps at 2×, and diffs
4×4-supersampled coverage rasters of both lanes at 1× and 4×, writing
PNGs for eyeballing.

### Results (decompose time, lane B relative to lane A)

| Row | Segments A → lines B | Decompose A | Decompose B | Ratio |
|---|---|---|---|---|
| selection (rounded rect) | 8 → 16 | 1.1 µs | 1.6 µs | 1.4× |
| spinner outline | 26 → 44 | 4.4 µs | 6.5 µs | 1.5× |
| 10-vertex star (lines) | 10 → 10 | 1.1 µs | 1.1 µs | 1.1× (tie, sanity ok) |
| chart area, `Natural` | 196 → 303 | 44 µs | 42 µs | **1.0×** |
| 1,000-pt scribble outline | 3,267 → 3,359 | 4.4 ms | 4.3 ms | **1.0×** |
| thin crescent (bug #1) | 4 → 6 | 4.1 µs | 1.6 µs | 0.4× |
| near-tangent (bug #2) | 4 → 32 | 5.4 µs | 6.4 µs | 1.2× |

Instance counts rise with flattening (selection 3 → 9, spinner 31 → 57,
chart 329 → 459 at 2×) but snap stays proportional (µs-class) and the
instances are trapezoids the GPU was built to eat; the adversarial
scribble row — the one that matters — moved from 2,667 to 2,679.

**Verdict, per the pre-committed rule: lane B wins.** Every realistic
row is at or under 1.5×, and the two rows staged to punish flattening
(chart, scribble) tied. The predicted 3–4× segment multiplier never
appeared: at τ = 0.25 px, UI-scale curves flatten to only ~1.5–2× the
segments, most outline segments were already lines, and sweep cost is
dominated by event processing and crossing tests, not raw segment
count.

### Quality and the bug scenes

- At 1×, the lanes are visually identical on every row (no pixel
  differs by more than 0.25 coverage on any realistic row).
- At 4×, lane B shows bounded tolerance error (max 0.5–0.6 coverage
  diff on a few dozen pixels of 600-px-wide shapes) — the documented
  cost of fixed-tolerance flattening, and the same regime as the old
  lyon renderer, which flattened at fixed tolerance in logical space.
  Tightening τ to 0.0625 restores 4× quality but costs 2–2.8× on
  curve-heavy decompose rows; not worth it at GPUI's 1–3× scales.
- **The near-tangent scene vindicates the retreat.** Lane A drops the
  band entirely mid-span (the dropout class from the `painting`
  screenshots, reproduced in a 4-segment scene), confirmed in the
  rendered rasters, and loses ~20% of its coverage between scales
  (212.8 px² at 1× vs 2,719/16 = 170 px² at 4× — corruption, not
  geometry). Lane B stays continuous, within sampling noise of
  scale-consistent (193.2 vs 2,904/16 = 181.5; the band is ~0.5 px
  tall, where the harness's 4×4 supersampling quantizes). Bug #2's
  input class does not trigger on line input.
- **The thin crescent indicts snap, not curves.** Both lanes show the
  same row-boundary gaps: bug #1 lives in the topology-event handoff
  in `emit_chains`/snapping, unaffected by lane choice, and still
  needs the posture-rule fix (on simpler, line-only input now).

### What changed in code

- `PathBuilder::fill_outline` now flattens quadratics and cubics to
  lines at `FILL_TOLERANCE` (0.25 px) instead of converting cubics to
  quadratics — the production flip is net-negative LOC.
- The `drag_race` harness stays in the test module, building lane A's
  curve-carrying paths directly via `Path::curve_to` (which remains
  curve-preserving) so the retired input class stays measurable.
- The decomposer is untouched: curve support remains because the
  stroke pivot's cap geometry and any direct `Path::curve_to` callers
  still produce quadratic pieces. Its curve–curve intersection paths
  are now cold for `PathBuilder` fills.

### Consequences for the plan

1. The trapezoid fragment mode's **root solves are no longer required
   for `PathBuilder` fills**, but the shader must still evaluate
   quadratic edges while `Path::curve_to` exists and until the stroke
   pivot lands (stroke caps are quadratic pieces today). Decide during
   the pivot's shader work whether to keep one general edge evaluator
   or specialize; do not pre-optimize.
2. The `robust` crate's exact line predicates are now applicable to
   the crossing tests (the flattened-sweep retreat's second half).
   Adopt during the bug-fix step if over-splitting tolerances prove
   insufficient — not before.
3. Scale-independence of the decomposition is formally reduced to
   "decomposed once, snapped per scale, with τ-bounded error growth at
   high scale" — the same contract strokes already had. `Path::scale`
   still carries the decomposition without recomputation.

### Remaining plan (authoritative)

1. **Stroke-instance pivot** on D3D11 (round two item 1 + tier-1
   interior early-out), with the shader-mode decision from consequence
   1 above.
2. **Re-validate**: `painting.rs`, gpui-component's `brush.rs` and
   chart stories. The full gpui lib suite is green as of this round
   (215 tests, including the race).
3. **Oracle harness**, then the bug-fix step — now scoped to bug #1
   (event-handoff truncation, reproduced by the `thin-crescent` row in
   both lanes) plus over-splitting/self-healing hardening on line-only
   input, with `robust` predicates as the escalation.
4. **Port to Metal and wgpu** (unchanged).
5. **Weak-GPU gate and visual pins** (unchanged, mandatory).
6. **Seal `Path`** (unchanged migration step).

## Findings, round five: the library-primitives rewrite (late July 2026)

With the curve sweep retired (round four), the decomposer was rewritten
to lean on library primitives everywhere a library primitive exists,
and the harness was repurposed to race the shipping pipeline against
lyon's `FillTessellator` — the old renderer's kernel — as a permanent
benchmark-plus-visual-diff fixture.

### What changed in code

- **`Path` stores lines, not quadratics.** `PathQuadratic` (degenerate
  midpoint-control-point convention) became `PathLine {p0, p1}`.
  `Path::curve_to` flattens on entry (lyon `for_each_flattened`,
  `PATH_FLATTEN_TOLERANCE` = 0.25 px in `scene.rs` — the single
  flattening seam; `PathBuilder` routes quadratics through it and
  flattens cubics directly at the same constant).
- **The sweep is line-only.** `collect_pieces` no longer splits at
  curve extrema (lines are born monotone); the sweep's x-at-y
  evaluation is a single interpolation; `Piece` lost its `is_line`
  flag. The quadratic-solve helpers (`x_at_y`/`t_at_y`) moved into the
  test module, which still rasterizes stroke-cap curves — production
  CPU code never evaluates a curve anymore.
- **Crossing decisions are exact.** `segments_cross` uses the `robust`
  crate's `orient2d` (Shewchuk adaptive-precision predicates, georust,
  new dependency) for the *decision*; the split parameters derive from
  the same orientation values (`o₀/(o₀−o₁)`), so only coordinates are
  approximate, never topology. `lyon_geom`'s curve–curve intersection
  numerics — the source of both round-one bug classes — are no longer
  called by anything.
- **The instance format is untouched.** `TrapezoidEdge` still carries
  quadratic pieces because `StrokeSynthesizer` stamps true circle-arc
  chains; the shader contract is unchanged.
- Net production LOC went down; the curve-intersection machinery
  (extrema splitting, four-case intersection dispatch, quadratic root
  solving) was deleted outright.

### Ours vs lyon `FillTessellator` (release; build = per-shape geometry work)

| Row | Ours (lines → build) | Lyon (triangles → build) | Ratio |
|---|---|---|---|
| selection | 16 → 1.6 µs | 18 △ → 1.6 µs | 1.0× |
| spinner outline (τ 0.25) | 38 → 6.7 µs | 54 △ → 3.5 µs | 1.9× |
| spinner outline (τ 0.125) | 50 → 5.1 µs | 54 △ → 3.3 µs | 1.5× |
| star | 10 → 1.0 µs | 8 △ → 1.0 µs | 1.0× |
| chart area (`Natural`) | 327 → 46 µs | 498 △ → 32 µs | 1.5× |
| scribble outline | 3,402 → 4.3 ms | 4,340 △ → 3.0 ms | 1.4× |

Reading: our *build-time* cost is 1.0–1.9× lyon's — but lyon's number
was the old renderer's *per-paint* cost (it retessellated every frame
at device scale, then still paid the intermediate-texture/MSAA GPU
taxes), while ours is paid once per built path. Our per-paint cost is
the snap (25 µs on the adversarial scribble, sub-µs on UI shapes),
versus the old pipeline's tessellate-plus-rescale every frame. The
curious τ = 0.125 spinner being *faster* than τ = 0.25 is real and
reproducible: more, shorter lines produce a more uniform crossing grid
and fewer broad-phase candidate pairs.

Visual diffs against lyon's geometry (same supersampler on both):
zero visibly-differing pixels on star and both spinners at 1×;
selection/chart/scribble differ only along edges where analytic
coverage and triangle point-sampling disagree by a subsample. The
`near-tangent` scene — the round-one dropout class — now renders
continuously and matches lyon: with exact predicates the crossing
cannot be missed, so that bug class is structurally gone, not patched.
The `thin-crescent` gaps (bug #1, row-snap handoff) remain, unchanged,
still the oracle-step target.

### Consequences

1. The sweep's bespoke numerics surface is now: exact predicates
   (library), line–line split points (two lerps), and the
   event/span/chain bookkeeping. There is no in-house curve
   intersection code left to harden.
2. Bug #2's fix (self-healing active list) may still be worth doing as
   cheap insurance, but its known trigger (missed crossings) can no
   longer occur; priority accordingly lowered. Bug #1 is unchanged and
   remains the bug-fix step's substance.
3. The race harness (`race_against_lyon`) stays in-tree as the
   standing perf + visual-parity fixture against the old kernel — the
   "poor man's oracle" until the tiny-skia/zeno harness exists.

### Remaining plan *(superseded — see round seven)*

1. **Stroke-instance pivot** on D3D11 (unchanged from round four; the
   trapezoid fragment mode still needs curve evaluation until stroke
   caps move out of it, then may specialize to lines).
2. **Re-validate** `painting.rs` and gpui-component stories
   (unchanged).
3. **Oracle harness, then bug #1** (event-handoff truncation); bug #2
   hardening optional per consequence 2.
4. **Port to Metal and wgpu** (unchanged).
5. **Weak-GPU gate and visual pins** (unchanged, mandatory).
6. **Seal `Path`** (unchanged; note `PathLine` already replaced
   `PathQuadratic` in the public surface — no known external users).

## Round six: "lines decide, curves render" (late July 2026)

**Status: accepted and implemented, together with the i_overlay
adoption — see round seven for what shipped, the measured results,
and the architectural verdict that followed.** The section below is
the original proposal text, kept for the design rationale.

### The two-value conflict this resolves

Rounds four/five made `Path` bake its flattening tolerance at build
time: a built path is no longer resolution-independent (the τ = 0.25 px
error is invisible at 1–2× but facets small-radius curves at 4×).
That collided with two positions both held strongly:

1. Resolution independence is what the word "path" means; giving it up
   is wrong at the contract level.
2. Owning curve geometry code in-house is foolish for a UI framework
   with no computational-geometry background — the round-five state
   (all decisions made by `robust`/libraries) is the right posture.

The conflict dissolves on the observation that "geometry code" is two
separable things. *Decisions* (does this cross that, what is inside)
are where all fragility, all past bugs, and all maintenance risk live
— foolish to own, correctly outsourced in round five. *Evaluation*
(where is this monotone quadratic at row y) is a total function with
one in-range root by construction — frozen, textbook, incapable of
new failure modes, and **already owned forever** because stroke caps
put true quadratic pieces in the instance stream (round two) and the
shader must evaluate them regardless.

### The design

- `Path` stores curves again (quadratics; cubics approximated to
  quadratic chains as before round four). No tolerance is baked into
  the built path.
- At build, each curve is split monotone (`lyon_geom`, a library call,
  never a bug source) and flattened into **proxy lines used only for
  decisions**, each proxy tagged with its source piece and t-range.
- **The CPU pipeline runs on proxy lines exactly as it runs today.**
  Same sweep, same lerp ordering, same exact crossing decisions
  (`robust`, or i_overlay if adopted). The CPU never evaluates a
  quadratic. Round-five performance and robustness are unchanged by
  construction.
- At emission, consecutive runs whose bounding proxies share a source
  piece merge (today they merge on piece identity; the rule becomes
  source identity), and `make_edge` forwards the **original curve
  piece** into `TrapezoidEdge` — which already carries quadratics, into
  a shader that already evaluates them.

Rendered geometry is the true curve at every scale. The proxy is
never visible at any zoom.

### Why this is a principled layering, not a hack

The objection "two representations of the same geometry" was raised
and examined. The proxy is not a second authority that can race the
curve; it is a *conservative abstraction* with a proven bound, used
only for questions it answers correctly:

- Proxy vertices lie **on** the curve (flattening subdivides the
  curve), so every event y, slab boundary, and edge junction is an
  exact curve point.
- Between vertices, proxy and curve differ by ≤ τ. The only decisions
  made inside that band — near-tangent ordering, exact crossing
  placement — are the class where any answer is visually acceptable
  (posture rule 1; every shipping renderer accepts this).
- Crossing splits land within τ of the true crossing, producing at
  worst a sub-pixel notch at a self-intersection under translucent
  paint — the same artifact class as the already-accepted stroke
  overlap contract.

Prior art, because the pattern is twenty years old: **Loop–Blinn
(SIGGRAPH 2005)** triangulates the chord polygon (topology from
lines) and shades exact curves in the fragment shader — lines decide,
curves render, verbatim. Slug makes banding decisions from control
boxes and solves exact roots per pixel. **GPUI's own removed
`push_triangle` API was Loop–Blinn-style curve triangles** — this
repo shipped the pattern for years. Trapezoids with curve edges are
the same division of labor in a better container.

### Where the real risk lives, and its pin

The genuinely new bug class is **provenance bookkeeping** (a wrong
t-range renders the wrong sub-curve) — mechanical, not numerical. It
is pinned by a pre-committed harness invariant: *curve-rendered
output must diff ≤ one subsample (1/16 coverage) from a
finely-flattened reference of the same decomposition, per row, at 1×
and 4×.* This is a property test over the existing race rows, not a
new harness.

### i_overlay compatibility (verified July 2026)

If the i_overlay adoption (round-five option) proceeds, curve
provenance survives it: v7's `EdgeOverlay` API accepts a user payload
per input edge (`InputEdge { a, b, data }`), **copies data through
intersection splits**, and merges collinear edges only when their
data is equal, explicitly to preserve attribute boundaries. Tagging
proxies with (piece index, t-range) and recovering sub-ranges by
projecting output endpoints onto the proxy is plain arithmetic.
Caveats: the provenance API is integer-only today (fine — the float
adapters expose the same grid) and returns per-edge structures
(slightly different plumbing than the plain contour API).

### Costs, stated plainly

- ~120–250 lines of production code back (storage of control points,
  monotone-split call, proxy tags, merge-on-source rule), partially
  reverting round five's deletion. None of it makes a numeric
  decision.
- The fill fragment mode keeps its root solves (~30–40 ALU vs ~10–15
  closed-form) — already mandatory for stroke caps, bounded by the
  weak-GPU gate, mitigated by the tier-1 interior early-out.
- Instance counts *improve* (curve edges merge back: round-four
  measured the spinner at 31 curve instances vs 48 line instances).

### Pre-committed acceptance tests

1. Spinner row at 4×: diff vs a finely-flattened reference ≤ 1/16
   everywhere (the faceting must be gone, exactly).
2. Every race row: decompose time within noise of round-five numbers
   (the CPU path is unchanged; any regression means the design was
   implemented wrong).
3. Instance counts ≤ round-five counts on curve rows.
4. Provenance invariant above, all rows, 1× and 4×.

### If accepted, sequencing

Insert after the stroke pivot and before the Metal/wgpu ports (so
backends are ported once, against the final instance semantics). The
i_overlay decision can be taken independently before or after; the
designs compose. If rejected, delete this section and the round-five
plan stands.

*(In the event, both were accepted and implemented together, before
the stroke pivot — see round seven.)*

## Findings, round seven: round six ships; the two-sweep verdict; round eight decided (late July 2026)

This round covers one long session: the round-six design plus the
i_overlay adoption were implemented and measured; the user then
identified a real architectural flaw in the result ("a sweep on top
of another sweep"); the investigations that followed — into
i_overlay's own source, lyon's own source, and every trapezoid
library that exists — converged on a decided round eight. Read this
section before touching anything.

### What shipped (on the branch, validated)

"Lines decide, curves render" is real code, built on `i_overlay`'s
`EdgeOverlay` payload API:

- **`Path` stores curves again.** `PathQuadratic {p0, ctrl, p1}`
  replaced `PathLine`; lines are degenerate quadratics with the
  control point at the exact midpoint. `Path::curve_to` stores
  exactly — no flattening at build; a built path is
  resolution-independent again. `PathBuilder` converts cubics to
  quadratic chains (`for_each_quadratic_bezier` at
  `PATH_FLATTEN_TOLERANCE`), the only approximation baked into a
  fill path.
- **Decisions went through i_overlay.** `Decomposer::resolve` splits
  each segment xy-monotone (lyon), flattens each piece into proxy
  lines tagged `SourceTag(piece index)`, quantizes to a 1/1024-px
  grid, and feeds `EdgeOverlay<i32, SourceTag>`; the library resolves
  all crossings and the fill rule with exact integer arithmetic and
  returns non-crossing boundary edges with tags preserved across
  splits. The sweep then runs on clean edges with a parity toggle
  (fill rule pre-applied).
- **Provenance needs no t-ranges.** Pieces are y-monotone, so an
  instance's own y-range selects the sub-curve; the shader clamps the
  parameter (it always did). Runs store source-piece tags; `push_run`
  merges on tag equality, so proxies of one curve fuse back into
  single runs. The round-six "provenance bug class" (wrong t-range →
  wrong sub-curve) largely evaporated — there are no t-ranges.
- **Corner patches at former crossings** (two pieces meeting away
  from a shared endpoint) subdivide each piece exactly at the
  junction row via `t_at_y` + `before_split`/`after_split`
  (evaluation-class, de Casteljau).
- **Deleted:** `split_crossings`, `CrossingGrid`, `PieceBounds`,
  `segments_cross`, `orient`, `is_inside`, `Piece.winding`, and the
  `robust` dependency. Net production −143 lines vs round five while
  restoring resolution independence (round six alone was budgeted
  +120–250; the pair beat the estimate).
- **Validated:** `cargo nextest run -p gpui --lib --release` — 215
  passed. Race rerun (release):

| Row | Build | Instances @2x | Notes |
|---|---|---|---|
| selection (8 seg) | 2.28 µs | 3 (was 9) | build +0.7 µs vs round five: EdgeOverlay setup cost |
| spinner outline | 7.7 µs | 19 (was 48) | curves exact at every scale now |
| star | 1.52 µs | 5 | zero visibly-differing pixels vs lyon |
| chart area | 52 µs | 179 (was 507) | |
| scribble outline | 3.10 ms (was 3.96) | 2,567 | snap 34 µs |
| thin-crescent | — | — | bug #1 unchanged, still the oracle target |

  Instance counts collapsed (merge-on-source works); 4× diffs vs lyon
  now partly measure *lyon's* flattening, since ours renders exact
  curves. Test rasterizer switched to half-open row sampling
  (quantized coordinates land exactly on subsample boundaries and
  were double-counted at shared instance edges).

### The verdict: "a sweep on top of another sweep" (accepted)

The user's critique of the shipped architecture is correct and is now
load-bearing: i_overlay internally sorts every edge, resolves every
crossing, and computes side fills — then returns edge soup, and our
sweep rebuilds the ordering it just had in order to extract
trapezoids it never knew we wanted. The composition re-derives
discarded information. Each component is justified; the seam between
them is redundant. This drove three investigations, all against
local sources:

**i_overlay itself (21,806 lines total; ~9k on our call path).**
The `split/` subsystem is 3,059 lines — a 1,586-line grid broad
phase and three narrow-phase strategies behind an adaptive chooser.
But `solver_list.rs`, the naive strategy, is **70 lines**: sort by
x, pruned quadratic pair loop, exact integer cross tests, apply
splits, **fixpoint loop with escalating snap radius** (Hobby's cage
as a `while`). It is selected below `MAX_SPLIT_LIST_COUNT = 4_000`
segments (fill stage: 8,000). Our worst row ever — the scribble,
~3,400 proxies — is under every threshold. **Every benchmark we ever
ran exercised only i_overlay's naive paths.** At GPUI scale,
i_overlay *is* the textbook design, wrapped in escalation armor we
never trigger.

**lyon (the old kernel).** Fill machine = `fill.rs` 3,029 +
`event_queue.rs` 1,019 + `monotone.rs` 406 ≈ 4,450 production lines,
kept honest by ~23,750 test lines. It is the *fused* architecture —
crossings discovered mid-sweep — in `f32`, and the source documents
its own repairs: a precision-hazard clamp in `solve_x_for_y`; a
hardwired-shut `if true ||` around intersection handling (the
adjacent comment: tolerance-collapse "can cause a
non-self-intersecting path to self-intersect"); the
`handle_intersections` confession ("we have to take great care…
manually fixing things up"); a one-ULP `next_after` nudge when a
computed crossing lands above the sweep line; and
`check_remaining_edges`, ~2.5–3% of the profile spent detecting the
sweep's own broken invariants at runtime. Why 3k lines against our
~650 target: five contracts we don't carry — fused crossings,
connected triangle meshes with vertex ids and attribute
interpolation, floats, library-grade generality, eight years of scar
tissue. (Honest asymmetry: we carry ~300 lines of seam-contract
snapping they don't need.) `lyon_geom` (8,794 lines) remains rented
forever — evaluation-class, no decisions. **Gift: `fuzz_tests.rs`,
992 lines of minimized inputs that each broke this production sweep
once. MIT. Import them as oracle rows for round eight.**

**Every trapezoid library on earth** (so this never needs
re-litigating): `makepad-trapezoidator` (Rust UI framework, same
architecture, ~490 lines — no crossing resolution, no provenance, no
chains/seam contract; its size is independent confirmation that our
kernel is market-priced); `triangulate` (Seidel; `Trapezoidation` has
private fields — only consumable as triangles; preconditions forbid
real paths; randomized output); `klayout_geom::fracture` (EDA mask
fracturing — i_overlay for booleans plus its own kernel, our exact
architecture in another domain); cairo's traps compositor (fused
in-house Bentley–Ottmann; Worth's 2006 rewrite post describes the
disease; demoted to glyph fallback by 2017); CGAL (the only true
curve-native vertical decomposition — exact algebraic arithmetic,
C++, not frame-rate). Pattern: trapezoid extraction exists either
fused to an in-house crossing engine, or with pre-resolved input and
no provenance/AA contract. Nobody ships "resolved input → tagged
trapezoid runs"; that layer is always the renderer's own. Forking:
never (a fork is maximum ownership); upstream PR to i_overlay is the
only respectable form, post-seal, ceiling ≈300 lines. A WPF port was
priced and rejected: battle-testedness does not survive translation
— the exposure odometer resets to zero; trust is manufactured by
oracles, not inherited by ancestry.

### Round eight (DECIDED): the textbook edition — one sweep, ours, exact

The standing rule "decisions must come from libraries" is restated as
its true self: **decisions must be exact.** On integer coordinates,
exactness is grade-school arithmetic (`i64` cross products;
coordinates ≤ 2²³ ⇒ products ≤ 2⁴⁶), ownable without
computational-geometry expertise. i_overlay's robustness was never
cleverness; it was integers. We take the lesson and retire the
dependency.

Five verbs, each transparent:

1. **Quantize** proxies to the 1/1024 grid (unchanged).
2. **Split crossings exactly**: x-sort + interval-overlap prune
   (~25 lines); exact integer cross function (~100–120 lines — the
   named dragon: pure crossings, endpoint touches, collinear
   overlaps; caged by lyon's fuzz corpus); splits rounded to the
   grid; fixpoint loop with escalating snap radius, copied as a
   discipline from `solver_list.rs`.
3. **One y-sweep** on non-crossing segments: winding accumulation
   returns (real fill rules, ~6 lines); naive span matching by
   x-overlap per boundary; `Vec<Vec<Run>>`; **no arena, no
   sentinels, no identity maps, no lazy x-eval** — the ~350 lines of
   performance armor stay deleted, and armor returns only by
   measured warrant (a bleeding race row, with the measurement in
   the commit message).
4. **Snap** rows per paint (unchanged — corner patches, row
   ownership).
5. **Shader** integrates the true curves (unchanged).

Keeps: the harness/race (the referee — the new lane races the old
decomposer, lyon, and i_overlay until parity), shader + instance
formats, `Path` curve storage, the snapper (plus a readability pass:
encapsulate run storage, kill `NO_RUN` sentinels at call sites),
`StrokeSynthesizer` until the stroke pivot. Deletes **at the end,
as the final commit**: the i_overlay dependency, `resolve`'s overlay
plumbing, `SourceTag`. Budget ≈650 lines across split modules
(`path_fill.rs` ~450, `path_snap.rs`, `path_stroke.rs` until the
pivot); `path_trapezoids.rs` ceases to exist. Expected performance:
small paths improve (drop ~0.7 µs EdgeOverlay setup); the scribble
may give back some of 3.10 ms — acceptable within the
order-of-magnitude bar.

**Process gates (pre-committed, mechanical):**

1. **Skeleton first.** The new fill module's opening doc — the
   invariant and worked example below — plus a complete table of
   contents: every function, a one-line contract, a line budget
   (~40 lines total). The user approves the skeleton **before any
   function bodies are written**; the finished file must match its
   approved skeleton's table of contents, rejection by diff, not by
   taste.
2. **Cold-reader test.** A GPUI teammate with no geometry background
   reads `path_fill.rs` top to bottom in one sitting and can explain
   each stage. Not done until this passes.
3. **Parity before demolition.** 215 tests green; race rows within
   noise (small paths may improve); star pixel-identical to lyon;
   instance counts unchanged; lyon's imported fuzz corpus passes
   against the oracle rasterizer. Only then the deletion commit.

### The corner-patch explainer (module-doc source, user-derived)

The plain-language chain for the worked example, produced this
session and better than anything previously written here:

1. A fragment can't fill half a pixel: partial coverage becomes
   transparent ink over the whole pixel.
2. Two coats can't make one: 50% over 50% is 75%, never 100% — the
   blend unit cannot distinguish "my sibling made it gray" from "the
   background was gray." Therefore **never cut a shape mid-pixel;
   cut only between pixel rows.**
3. The shape's corners are the caller's `line_to` points —
   immovable input. A trapezoid stores no corners: it is two edge
   references plus a y-window; its apparent corners are where the
   window crops the lines (crop marks, not data).
4. The invariant, in the user's words: *"Every handoff between
   stacked trapezoids gets floored/ceiled apart, and a one-row bent
   trapezoid owns the row in between. Interior seams are always
   integers. Every pixel row has exactly one owner. No number the
   caller gave us ever changes."* (Chain-outermost tops/bottoms stay
   fractional — they abut nothing; handoffs already on a row
   boundary need no patch.)
5. Derivation as three refusals: refuse geometry distortion (scale
   invariance), refuse double shading (the ink arithmetic), refuse
   the accumulator texture (the frame budget that started this
   project). The design is the unique remainder.
6. Two loops, two times: the sweep runs per **event** at build (who
   are the bounding edges in this y-interval); the shader runs per
   **pixel** at paint (how much ink). A corner patch is the active
   edge table's answer for one row, computed at build and frozen
   into instance data, because the GPU cannot stop to ask.

Worked example (the diamond): T (50, 10.3), R (80, 50.7),
B (50, 91.1), L (20, 50.7) → three instances:
`[10.3→50.0] TL/TR` · `[50.0→51.0] corner patch, both sides swap at
50.7` · `[51.0→91.1] LB/RB`.

### Remaining plan (authoritative)

1. **Round-eight skeleton** → user approval → implementation raced
   to parity → deletion commit (i_overlay and the old decomposer
   out). Readability split lands as part of this.
2. **Stroke pivot** (deletes `StrokeSynthesizer` ~270 lines; capsule
   and box fragment modes; tier-1 interior early-out).
3. **Oracle harness** (tiny-skia/zeno) + lyon fuzz corpus; then
   **bug #1** (thin-crescent row-snap handoff truncation).
4. Re-validate `painting.rs` and gpui-component stories.
5. Metal and wgpu ports; weak-GPU gate and visual pins; seal `Path`.
6. Post-seal, optional: propose payload-carrying trapezoid
   extraction upstream to i_overlay (the only respectable fork).

### Pitfalls (round seven — do not repeat)

- Don't re-litigate "isn't there a library for this" — the survey
  above is the answer; the extraction layer is always the renderer's
  own. Point at this section.
- Trust is not portable. Porting battle-tested code resets its
  exposure odometer to zero. Trust is manufactured by oracles and
  fuzzing, not inherited through translation.
- Don't infer a wrapped library's complexity is necessary at your
  scale — check its own escalation thresholds first. (i_overlay runs
  70-line naive paths for everything GPUI-sized; the other ~8,900
  lines never executed for us.)
- Readability is fixed by structure, vocabulary, and encapsulation —
  never by deleting requirements. Each requirement's deletion has a
  named, user-visible artifact (seams, faceting, dropped shapes).
- Don't judge the plan by condemned code (`stamp_rect` is
  superseded-design residue awaiting the stroke pivot).
- Skeleton-first is mandatory for round eight. A file that drifts
  from its approved skeleton is rejected by definition — that gate
  exists precisely because rounds one through six were exploration
  and accreted, and round eight is transcription and must not.
