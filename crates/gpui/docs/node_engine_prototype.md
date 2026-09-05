# Retained view-node engine

Select the experimental engine when creating a window:

```sh
GPUI_EXPERIMENTAL_NODE_ENGINE=1 cargo run -p gpui --example node_engine_boundaries
```

Use `GPUI_EXPERIMENTAL_NODE_ENGINE=0` for the legacy engine. The boundary lab
exercises keyed state, changed inputs, callbacks, focus, hover, sibling geometry,
mounting, shared dependencies, and deferred painting. Its displayed statistics
refer to the preceding frame. Element traces are not GPU timings.

## Identity and invalidation

Each mounted view occurrence has an `Entity<ViewNode>`. `NodeEngine` owns these
handles and maps a window-local `GlobalElementId` to each node's `EntityId`.
Parent and child edges contain IDs rather than owning handles. The same view can
therefore appear in different mounted scopes without sharing their recordings.

Each node records the entities read while rendering its scope, and the engine keeps
the reverse map from entity to consuming nodes. A parent depends on its children, so
a dirty child dirties its ancestors through the same graph. Reads establish
dirtiness only: `App::notify` still notifies just the entity it is given, marking
windows that read it dirty and running its observers, and the engine expands the
window's dirty set through the consumer map when the next frame begins. Rebuilding
replaces a node's dependencies; unmounting removes them. Entity mutation revisions
also reject reuse after an unnotified update when some other input causes a frame. Notifications emitted during drawing invalidate recordings for the next
requested frame. They do not schedule a draw themselves, preserving the existing focus-lost
fallback behavior.

A dirty scope rebuilds all phases. Clean siblings can reuse their output. This
implementation does not independently render a dirty descendant through a clean
ancestor. Broad `Window::refresh` remains a full rebuild. Built-in hover and scroll
invalidation can target the current node.

## Layout and recordings

Ordinary entity views retain their Taffy layout roots automatically; existing
explicit `.cached()` views remain supported. Frame completion removes unreachable
layout nodes and their measurement closures. When every mounted node is dirty,
old layouts are released before rebuilding; the new tree is retained for later
partial updates. Allocation order and a generational secondary map avoid hashing
every layout node during collection. Text measurement closures own their
state and can survive frames. Public custom measurement callbacks conservatively
make their enclosing scopes rebuild because they may capture frame-arena elements.

Reuse checks dependencies and ambient inputs, including inherited text style,
opacity, clipping, rem size, scale, and image cache identity. Prepaint checks actual
bounds and the computed layout of the subtree again. A changed layout rebuilds
the scope, replaces its Taffy root in the containing tree, and recomputes from
that tree's original available space. Laying the replacement out independently
would change percentage and intrinsic sizing semantics.

Recordings own scene fragments, hitboxes, cursor and tooltip slots, dispatch
nodes, tab-stop operations, window controls, state keys, and text layout leases.
Mouse callbacks and input handlers use shared mutable handles. Replay recreates
frame-local dispatch IDs. Parent scene recordings interleave local fragments and
child node IDs to preserve order without copying every descendant's
primitives into every ancestor. Each node owns its recording directly. Reuse moves
the recording through the frame's element phases and returns it to the node after
paint; descendant recordings remain in their nodes. Local scene fragments retain
paint operations rather than rebuilding GPU lanes and the bounds tree during
capture. Dirty nodes refill their existing recording buffers, retaining capacity
until unmount. Unchanged dependency sets keep their existing graph edges. The
submitted scene remains flat and contiguous. Retained replay does not depend on
the preceding frame's listener or text indices.

Nodes own `use_keyed_state` entities and their subscriptions. State not accessed
on a rebuild is released; removing a mounted subtree releases its local state.
Externally retained state handles can survive removal without retaining their old
render subscriptions. The existing element-state map still serves element APIs.

## Repeatable components

`Component::render(&self, window, cx)` complements `RenderOnce`. Mount a component
with `gpui::component(key, value)`. The keyed component instance is an entity;
new parent inputs replace its value and invalidate its output. Stable keys preserve
local state. Props are conservatively treated as changed whenever the parent
supplies them; there is no implicit equality comparison. Existing `RenderOnce`
components continue to render with their parents.

## Validation and remaining boundaries

Run the GPUI tests under both engines:

```sh
GPUI_EXPERIMENTAL_NODE_ENGINE=0 cargo test -p gpui --lib --no-default-features
GPUI_EXPERIMENTAL_NODE_ENGINE=1 cargo test -p gpui --lib --no-default-features
ITERATIONS=20 cargo test -p gpui node_engine --lib --no-default-features
```

Paired legacy/retained windows compare scenes across selected state, geometry,
opacity, ordering, and mount changes. Separate tests cover callback replacement,
local-state disposal, notifications during render, replaced dependencies, skipped
clean siblings, and bounded layout storage across repeated nested reuse. Additional
regressions cover percentage widths and padding under flex/grid constraints,
focus and keyboard dispatch through reuse, moved hit targets, hover, and disposal
of measurement callbacks that capture frame-arena elements. These are
focused differential tests, not a complete semantic oracle for arbitrary Zed UI.

Deferred drawing, prompts, accessibility, and inspector output conservatively
force rebuilds. Debug-selector bounds are recorded and replayed in paint order,
including repeated selectors and removal, without forcing a full redraw.
Global and other ambient invalidation still relies
on existing window refresh paths where precise tracking is unavailable. Full Zed
interaction, IME, and every custom element's side effects need further coverage.

Changed scope bounds are diagnostic only. GPU scene submission and presentation
remain unchanged; the engine does not submit damage regions. Render counts
establish skipped CPU work, not measured battery or GPU savings.

The editor's `test_workspace_rendering_stress` opens six 1,000-line Rust files in
three panes of a real `MultiWorkspace`. It runs 48 tab/focus, selection, edit,
scroll, and resize updates, comparing each completed scene with a forced full
refresh of the same window. This checks cache reuse against rebuilding with the
same engine and resources, not against a separate legacy window. Run it in both
modes:

```sh
GPUI_EXPERIMENTAL_NODE_ENGINE=0 cargo test -p editor test_workspace_rendering_stress --lib -- --nocapture
GPUI_EXPERIMENTAL_NODE_ENGINE=1 cargo test -p editor test_workspace_rendering_stress --lib -- --nocapture
```

Set `GPUI_STRESS_STEPS` to change the update count (at least four). Updates are
grouped by editor so focus changes do not turn every step into a full refresh.
The retained run also requires a nonzero reuse count. Frame statistics report
the reason for full refreshes and the number of scopes whose measurement
captures cannot survive a frame.

This uses production workspace/editor rendering with the headless test platform;
it does not exercise native GPU presentation, OS input, or IME composition.

Run the path-capture microbenchmark with:

```sh
cargo test -p gpui path_recording_capture_benchmark --release --lib --no-default-features -- --ignored --nocapture
```

It compares clearing and reusing vertex buffers for 32 paths, alternating run
order across five batches. Both variants still copy the geometry. Buffer lifetime
regressions independently check that stable path slots preserve allocation and
that replay matches the source scene when geometry and local ranges change.

## Follow-up checklist

Performance and ownership:

- [ ] Give every recorded effect local fragments and child-node references, as scene
  operations already have. Parent recordings currently duplicate descendants'
  hitboxes, dispatch nodes, callback handles, text-layout references, and state
  keys. Preserve phase order and dispatch-ID remapping while removing duplication.
- [ ] Audit path-heavy scenes. `Path::clone` copies its vertex vector; scene
  insertion, capture, and replay can duplicate geometry. Measure copied bytes and
  allocation counts with paths, then reduce payload copies and retain nested
  buffer capacity. The text benchmark's allocation result does not cover paths.
- [x] Preserve existing path vertex buffers when overwriting node recordings.
  Regression tests cover changed geometry, shrinking paths, operation-type
  replacement, child-range compaction, and removal. This removes repeated vertex
  allocation during stable capture; it does not remove the vertex copies.
- [ ] Make retained scene storage the primary representation and reduce the
  additional operation buffers, while preserving existing frame/replay APIs.
  Extra recording capacities account for about 93% of the measured pane heap
  increase; capacity reuse alone does not remove this duplication. Avoid the
  active frame-to-recording capture pass where ownership permits.
- [ ] Benchmark deeper nesting, many callbacks, large paths, mixed dirty scopes,
  scrolling, and representative Zed windows. Measure CPU phases, copied bytes,
  allocations, and retained-memory high-water marks across mount/unmount cycles.
- [x] Add live-heap sampling to the original 64-view text microbenchmark on
  macOS. Use `GPUI_BENCH_MEMORY=1 GPUI_BENCH_ENGINE=legacy` or `retained` with
  `node_engine_update_benchmark`; run each engine in a separate process. Add
  `GPUI_BENCH_ALL_DIRTY=1` for dense updates and `GPUI_BENCH_MEMORY_CYCLES=6`
  for repeated mounting/unmounting. Samples include startup, mount, each batch
  of 100 frames, and unmount. `malloc_zone_statistics` reports live heap across
  all malloc zones, including native framework allocations; this excludes GPU
  memory, code pages, and stacks and is distinct from allocation traffic or RSS.
  Paint-operation vector capacities are reported separately for both frame
  buffers and node recordings; these are a partial breakdown, not total heap.
  Add `--features test-memory` to count live requested bytes through Rust's
  global allocator separately from native caches and allocator overhead. The
  counter is selected automatically by GPUI's unit-test executable and by the
  editor benchmark executable when its `test-memory` feature is enabled. Do not use its
  timing output for CPU comparisons: allocation instrumentation adds overhead.
- [ ] Measure repeated root-layout computation when several cached scopes change
  geometry in one frame; avoid repeated whole-tree work where semantics permit.

Correctness and integration before considering default enablement:

- [x] Drive the real editor through the installed platform input handler after
  retained replay. Cover composition updates, UTF-16 selection and replacement
  ranges (including emoji), candidate bounds against a forced refresh, commit,
  unmark, resize/scroll, and removal during composition. Run
  `GPUI_EXPERIMENTAL_NODE_ENGINE=1 cargo test -p editor
  test_ime_platform_handler_across_retained_frames --lib` (use `=0` for legacy).
  This checks platform callbacks, not the native input-method frontend.
- [ ] Exercise native IME composition: marked text, replacement/selection ranges,
  UTF-16 offsets, candidate-window geometry, commit/cancel, and focus changes or
  subtree removal during composition. Include scrolling/resizing during
  composition and native macOS, Linux, and Windows input paths.
- [ ] Run real Zed interaction traces: typing, selection, scrolling, pane resizing,
  tabs, terminal input, menus, popovers, drag-and-drop, and multiple windows.
  Compare legacy and retained behavior through full and partial redraws.
- [x] Add a headless real-workspace rendering stress test with six Rust files,
  three panes, 48 updates, full-refresh scene comparisons, and a retained-reuse
  assertion. Native interaction and the remaining UI surfaces still need coverage.
- [ ] Expand the differential oracle beyond selected scene snapshots: replay
  deterministic event sequences and compare scenes plus hit testing, dispatch,
  focus/tab order, input-handler behavior, and state/resource lifetimes.
- [ ] Extend input tests for overlapping/clipped content, nested scroll regions,
  hover/tooltip/cursor transitions, keyboard propagation, and focused subtree
  movement, replacement, and removal.
- [x] Retain debug-selector bounds instead of forcing full-window rebuilds in
  tests. Cover duplicate selectors, cached replay, reordering, and removal.
- [ ] Verify accessibility, prompts, deferred drawing, and inspector
  fallbacks, including entering and leaving each mode with cached content present.
- [ ] Audit custom-element side effects and measurement captures for frame-arena
  lifetimes, including callbacks that disappear or change during a rebuild.
- [ ] Test theme/font/global-state changes, display-scale changes, window
  activation, asynchronous image loading, atlas eviction, and other ambient
  inputs; check both targeted invalidation and full-refresh fallbacks.
- [ ] Extend lifetime tests for repeated mounts/reorders, one entity mounted in
  multiple places or windows, conditional keyed state, subscriptions, and window
  closure. Verify stale recordings and handlers cannot keep resources alive.
- [ ] Run the validation matrix on supported native backends. Current measured
  performance and local validation are from macOS.

GPU validation:

- [x] Compare offscreen Metal pixels after retained replay with a forced full
  refresh, including text, paths, clipping, opacity, reorder, resize, and removal.
  `retained_scene_matches_full_refresh_pixels` passes under both engines on macOS.
  Run with `GPUI_EXPERIMENTAL_NODE_ENGINE=1 cargo test -p gpui_platform
  --features test-support,font-kit retained_scene_matches_full_refresh_pixels
  -- --ignored --nocapture` (use `=0` for legacy). This uses an offscreen Metal
  target and readback; it does not exercise the window compositor.
- [ ] Measure GPU work and presentation cost on representative Zed workloads.
  CPU frame timings do not establish GPU or battery gains.

Damage tracking (deferred to a follow-up):

- [ ] Compute conservative old/new damage extents for movement, removal, reorder,
  clipping, shadows, paths/antialiasing, opacity, and overlapping content.
- [ ] Integrate damage with renderer submission and presentation, including
  backend buffer-age/preservation requirements; validate against full rendering.
- [ ] Measure battery impact after backend damage integration.

## Editor and memory measurements

On the Apple M4 Max, three release runs per engine (order legacy, retained,
retained, legacy, legacy, retained) produced these medians of Criterion point
estimates. Each run used 20 samples, one second of warmup, and three seconds of
measurement. Allocation instrumentation was disabled.

| Workload | Legacy | Retained | Difference |
| --- | ---: | ---: | ---: |
| Existing `editor_render` benchmark | 0.763 ms | 0.783 ms | +2.6% |
| One editor pane, 1,000 lines | 1.128 ms | 1.170 ms | +3.7% |
| Three editor panes, one cursor moving | 3.323 ms | 1.761 ms | 47.0% less time |

The pane benchmark uses real Editor/MultiBuffer instances, native text shaping,
and offscreen Metal encoding and submission. Eight setup updates settle initial
invalidations; the three-pane retained case then rebuilds two scopes and reuses
two inactive editor scopes. It excludes workspace chrome, project services,
GPU completion time, compositor presentation, and vsync. These measurements do
not establish full-workspace performance or battery savings.

Build with `CARGO_PROFILE_BENCH_DEBUG=0 cargo bench -p benchmarks --bench
editor_render --no-run`. Run the resulting executable with
`GPUI_EXPERIMENTAL_NODE_ENGINE=0` or `1` and arguments
`'^(editor_render|Editor panes)' --bench --sample-size 20 --warm-up-time 1
--measurement-time 3 --noplot`.

The original 64-view, eight-text-rows-per-view microbenchmark was also measured
in release mode with `--features test-memory`. The following are total live
requested Rust heap bytes, expressed in MiB, including the shared test runtime
(2.016 MiB before mounting). Warm figures are medians of 15 snapshots: five
100-frame batches in each of three mount/unmount cycles in a single process per
engine/workload. Native allocations and allocator rounding are excluded.

| Memory stage | Legacy | Retained |
| --- | ---: | ---: |
| First mount | 7.250 MiB | 8.183 MiB |
| Warm, one leaf dirty | 9.662 MiB | 9.065 MiB |
| Warm, all leaves dirty | 9.656 MiB | 9.041 MiB |

Retained adds 0.933 MiB at initial mount; warmed live Rust heap is about 6% lower
in this fixture. This does not mean that recordings are free: both engines keep
672 KiB of frame paint-operation vector capacity, and retained adds another
336 KiB in node recordings. These figures exclude typed primitive lanes and
other metadata. Native malloc-zone totals fluctuate with framework caches and
are not used for the percentage comparison.
The element arena reserves 1 MiB in both engines. The net reduction has not
been attributed to individual allocation sites; it should not be extrapolated
to other workloads.

After the third unmount, live requested bytes return to 2,182,846 (legacy) and
2,256,672 (retained) for sparse updates; dense updates return to 2,183,962 and
2,245,296. The final two unmount totals match within each run. This bounds growth
in this fixture; it does not establish memory behavior for path-heavy scenes,
deep nesting, native GPU resources, or a full Zed workspace.

## Busy workbench validation and measurement

`Workbench/update/{row,editor,mixed,full}` embeds a real 1,000-line Editor in a
1600-by-1000 window with four custom GPUI panels and 48 independently owned row
entities. These panels exercise nested view boundaries, text, clipping, and click
handler recordings; they are not the production project/search/diagnostics panels.
The fixture has 54 retained scopes. Local row updates invalidate their containing
panel and ancestors, editor updates leave the panels reusable, and full updates
invalidate every scope. Mixed updates also reorder rows every eight updates and
resize the window every twelve. Eight setup updates settle initial invalidations.

`GPUI_BENCH_VALIDATE=1` with Criterion `--test` performs 24 updates and compares
both the final scene and offscreen Metal pixels with a forced refresh after each
update. It also checks for nontrivial scene and pixel output and reports actual
subtree reuse. `GPUI_BENCH_SCREENSHOT=/tmp/workbench.png` saves the first incremental
frame. This oracle detects stale cached output; refreshing the same engine is not
an independent implementation of all rendering semantics. Input callback dispatch,
native IME, compositor presentation, and GPU damage remain separate coverage.

Build the memory executable with `CARGO_PROFILE_BENCH_DEBUG=0 cargo bench -p
benchmarks --features test-memory --bench editor_render --no-run`. Run the emitted
executable in a fresh process for each exact benchmark filter, with
`GPUI_BENCH_MEMORY=1 GPUI_EXPERIMENTAL_NODE_ENGINE=0` or `1` and `--test`.
It reports live requested Rust bytes before setup, after mounting, every 100
updates through 500, and after closing the window and settling pending work.
Profiler collection is disabled during these samples. This counts retained heap
across all Rust threads, including shared application state; it excludes native
font/Metal allocations and allocator overhead. Closing one window is a cleanup
sample, not proof of leak freedom over repeated mount/unmount cycles.

Use the executable built without `test-memory` for CPU timing, with filter
`'^(editor_render|Editor panes|Workbench)' --bench --sample-size 20
--warm-up-time 1 --measurement-time 3 --noplot`. Do not combine the memory and
validation modes or interpret their empty Criterion callbacks as CPU timings.

After `f7e66a3192`, three release timing runs per engine/workload gave these
medians of Criterion point estimates on 2026-09-05. Every workload ran in a fresh
process, with engine order legacy, retained, retained, legacy, legacy, retained;
each used 20 samples, one second of warmup, and three seconds of measurement.
Allocation instrumentation was disabled. Runs interrupted by other compilation
or heavy CPU activity were discarded and retried. Exact benchmark-name filters
(e.g. `'^Workbench/update/full$'`) reproduce the process isolation.

| Workload | Legacy | Retained | Retained change |
| --- | ---: | ---: | ---: |
| Existing editor benchmark | 0.761 ms | 0.773 ms | +1.5% |
| One editor pane | 1.093 ms | 1.153 ms | +5.5% |
| Three editor panes | 3.218 ms | 1.681 ms | 47.8% less time (1.91×) |
| Workbench: row | 0.852 ms | 0.436 ms | 48.8% less time (1.95×) |
| Workbench: editor | 1.725 ms | 1.204 ms | 30.2% less time (1.43×) |
| Workbench: mixed | 1.343 ms | 1.033 ms | 23.1% less time (1.30×) |
| Workbench: full | 1.775 ms | 1.903 ms | +7.2% (+0.128 ms) |

The full-workbench point estimates range from 1.771–1.836 ms legacy and
1.865–1.947 ms retained. The existing editor benchmark's ranges overlap
(0.747–0.765 ms legacy, 0.754–0.776 ms retained), so its small median difference
should not be interpreted as a precise regression estimate. Earlier runs that
combined all workloads in one process, during intermittent CPU contention,
produced an approximately 10 ms retained full-workbench outlier; it did not
reproduce in isolated runs. This measurement does not attribute that outlier to
the engine or prove that CPU contention was its only cause.

On 2026-09-05, after integrating the notification fix in `f7e66a3192`, three
fresh processes per engine/workload on the M4 Max produced the following live
Rust heap medians. Each warm value combines nine samples (at 300, 400, and 500
updates in each process). These are whole-executable totals, including shared
runtime/application allocations, not just scene storage.

| Workload | Legacy warm | Retained warm | Extra retained heap |
| --- | ---: | ---: | ---: |
| Existing editor benchmark | 4.847 MiB | 5.037 MiB | 0.190 MiB (+3.9%) |
| One editor pane | 6.258 MiB | 6.963 MiB | 0.705 MiB (+11.3%) |
| Three editor panes | 9.580 MiB | 11.706 MiB | 2.126 MiB (+22.2%) |
| Workbench: row | 6.917 MiB | 7.793 MiB | 0.877 MiB (+12.7%) |
| Workbench: editor | 7.309 MiB | 8.242 MiB | 0.933 MiB (+12.8%) |
| Workbench: mixed | 7.175 MiB | 8.099 MiB | 0.925 MiB (+12.9%) |
| Workbench: full | 7.307 MiB | 8.497 MiB | 1.190 MiB (+16.3%) |

The workbench row samples are nearly flat: 6.916–6.925 MiB legacy and
7.793–7.794 MiB retained. Workloads with editor activity still vary across the
last three samples; the largest within-process increase from update 300 to 500
is about 0.189 MiB in each engine. This run does not establish a steady-state
plateau for those workloads. Three-pane warm ranges are 9.550–9.597 MiB legacy
and 11.686–11.722 MiB retained, so their extra heap is larger than the observed
sampling variation. The notification fix did not materially change these heap
costs relative to the preceding measurement.

After closing the windows, median Rust heap is 3.895/3.894 MiB for the existing
editor benchmark, 3.098/3.556 MiB for one pane, and 3.307/2.961 MiB for three panes
(legacy/retained). Workbench medians range from 2.850–3.403 MiB legacy and
2.952–3.377 MiB retained. Cleanup samples vary and include shared caches; they
should not be interpreted as a precise leak measurement. The editor/workbench
results demonstrate a memory cost unlike the original text microbenchmark's net
savings. Attribution to scene recordings, layout contexts, text caches, and other
allocation sites remains outstanding.

A subsequent capacity audit makes part of the memory cost explicit. Memory mode
also prints `paint-operation buffer bytes`, distinguishing the two flat frames
from retained node recordings. One fresh process per engine/workload sampled
capacities after each 100 updates through 500; all five capacities matched in
each run:

| Workload | Both flat frames, either engine | Additional node recordings |
| --- | ---: | ---: |
| Existing editor benchmark | 0.328 MiB | 0.164 MiB |
| One editor pane | 1.3125 MiB | 0.65625 MiB |
| Three editor panes | 2.625 MiB | 1.96875 MiB |
| Workbench, each update mode | 1.3125 MiB | 0.60242 MiB |

These are requested vector capacities, not a full heap census; path vertex
buffers, typed primitive lanes, and other metadata are excluded. The flat-frame
operation capacities are identical between engines. Node operation buffers alone
equal about 93% of the extra warm heap in the pane fixtures, and about 51–69% in
the workbench fixtures. They scale with retained scene content and capacity growth,
not just the number of node entities. This is additional storage alongside the
flat frames, rather than the same allocation moving to a different owner.

A three-second macOS CPU sample of each fully dirty workbench engine also shows
retained recording capture, including operation and dispatch-node copying, in
the retained stacks. The sample includes benchmark activity and inlined code;
it does not reliably allocate the 0.128 ms timing gap among individual costs.
It identifies copying as a target, not proof that removing it eliminates the
whole penalty. This audit changes diagnostics only and claims no optimization.

All seven workloads pass 24 scene and Metal pixel comparisons under each engine
(336 comparisons of each kind in total). The retained run records reuse in the
three-pane and row/editor/mixed workbench cases; the single-editor and fully dirty
cases legitimately rebuild everything. Mixed-workbench resizing calls
`bounds_changed` after `resize`, because test windows do not dispatch the native
resize callback. `./script/clippy -p gpui -p benchmarks` passes, including the
dependency audit.

## Review validation

After the notification fix in `f7e66a3192`, the GPUI suite passes under both
engines (252 tests, two manual benchmarks ignored), and thirteen focused
node-engine tests pass across 20 scheduler iterations. The earlier editor suite
run passed 899 tests with one ignored; it has not been repeated after that fix.
The workspace suite (238 tests) passed before the final allocation optimizations.

The workspace stress test passes its 48 scene comparisons under both engines.
Its initial version exposed a test-only full-refresh fallback for debug selectors
on workspace tabs. Retaining that metadata allowed the grouped workload to
exercise reuse; the first four sampled frames rebuilt 18 scopes and reused two
subtrees each, with 20 live scopes and 38 retained layout nodes.

The frame timings below predate the path-buffer and debug-selector follow-up;
they have not been remeasured for it. The path-capture microbenchmark produced
variable timings without a clear CPU-speedup conclusion. Its allocation-reuse
regressions pass, but geometry still gets copied into the recording.

The editor suite exposed a focus scheduling regression that the small scene
fixtures did not catch. Repeated notifications from focus-lost fallback handlers
must not schedule new frames. The retained engine now preserves that behavior
while keeping recordings invalidated for a later requested frame.

Run the release benchmark with:

```sh
cargo test -p gpui node_engine_update_benchmark --release --lib --no-default-features -- --ignored --nocapture
GPUI_BENCH_ALL_DIRTY=1 cargo test -p gpui node_engine_update_benchmark --release --lib --no-default-features -- --ignored --nocapture
```

The fixture has 64 entity views, each with eight text rows. Each engine processes
five batches of 100 updates, alternating which engine runs first. The default
case changes one leaf per update; the second command changes every leaf. Paired
runs compare the final scenes after each batch and assert stable retained layout
counts. Timings include updates and headless frame processing, excluding scene
comparison and logging. They do not include GPU submission or presentation.

For process memory measurement, set `GPUI_BENCH_ENGINE=legacy` or `retained` and
run the already-built test executable shown by Cargo under `/usr/bin/time -l`.
Single-engine runs omit the paired scene comparison. Maximum resident size is a
whole-process peak, not an allocation census of the node engine.

Measured on 2026-09-04 on an Apple M4 Max with 64 GiB RAM, using Rust 1.97.1
and the release test executable, after compilation and other test runs finished.
Medians combine 15 batches across three separate runs per workload:

| Workload | Legacy median per update | Retained median per update | Change |
| --- | ---: | ---: | ---: |
| One of 64 leaves dirty | 1.623 ms | 0.323 ms | 5.02× faster |
| All 64 leaves dirty | 1.631 ms | 1.723 ms | 5.7% slower |

Both workloads kept 65 mounted scopes and 1,089 retained Taffy nodes across the
500 updates. Sparse updates rebuilt two scopes and reused 63 subtrees. Fully
dirty updates rebuilt all 65 scopes. Earlier implementations measured 32.1%, then
17.9% all-dirty overhead. The latest pass reuses recording buffers, keeps unchanged
dependency edges, uses generational layout metadata, removes obsolete parents
before their children where allocation order permits, and bulk-clears old layouts
when every mounted node is dirty. A regression alternates full and partial redraws,
compares scenes against legacy, and checks that clean siblings resume reuse.

Before the bulk-clear optimization, diagnostic all-dirty builds measured an
approximately 194 microsecond gap. Omitting recording capture reduced that gap by
44 microseconds; using legacy layout cleanup reduced it by 74 microseconds;
omitting dependency registration reduced it by 16 microseconds. Omitting all three
left about 61 microseconds of node traversal and bookkeeping overhead. These
interacting differences are approximate, not additive cost guarantees. Those
variants were restricted to fully dirty benchmark runs and removed afterward;
they are not valid general-purpose engines.

Phase timers also showed that text-measurement destruction shifts from layout
creation in legacy to cleanup in retained rendering. Comparing cleanup durations
alone therefore overstates additional work. Whole-update timing decides whether
an optimization helps.

Before the allocation audit, the remaining overhead was about 144 microseconds.
A one-percent budget for this fixture is about 16 microseconds. Current rendering still builds the
active frame and then copies effects into node recordings; those recordings also
require dependency and lifetime maintenance. Making retained storage the primary
output destination could remove duplicate capture, but preserving all frame APIs
and reducing the remaining bookkeeping needs further work. These measurements do
not establish a fundamental cost of entities or a lower bound for a node engine.

A warmed foreground Rust-allocation audit subsequently found 566 extra allocator
requests per fully dirty update (479 allocations and 87 reallocations) relative
to legacy. Reusing reconciliation/traversal vectors, dependency sets and revision
buffers, and notification scratch storage reduced that to 132 allocations. The
remaining 130 per-view allocations came from constructing default font features
twice per scope while building cache keys. A private shared default text style
removes these without changing public `TextStyle::default()` behavior. Dispatch
recordings also reuse listener and key-context buffers nested inside their nodes;
child-scene ordering uses an in-place unstable sort to avoid sorting allocations.

In the final measured warmed batch of 100 fully dirty updates, legacy made 887,100
allocations and 16,001 reallocations; retained made 887,300 allocations and the
same 16,001 reallocations. That is two extra allocation calls per update. The
instrumented capture, store, node-entry, layout-retention and child-reconciliation
paths each recorded zero allocations in that batch. This does not make a full
redraw allocation-free: both engines still construct elements and text each
frame. Counts cover foreground Rust allocator requests, not native allocations,
other threads, or live memory. Temporary allocator instrumentation was removed
before CPU timing and final validation. Reused scratch containers retain their
high-water capacity; dependency-set pools contain IDs, not entity ownership.
After the allocation cleanup, uninstrumented timing measured about 93 microseconds
of full-redraw overhead (5.7%) and a 5.02× sparse-update speedup, as shown above.

Before the recording allocation optimizations, three separate single-engine
sparse runs measured whole-process peak RSS of
37.9–48.5 MiB for legacy (median 40.6 MiB), and 26.4–38.3 MiB for retained
(median 31.5 MiB). These variable, overlapping process peaks do not establish
a general memory advantage. The stable scope/layout counts provide the stronger
lifetime check for this fixture; heap attribution remains a separate measurement.

`./script/clippy -p gpui` passes, including all-target/all-feature checks and the
dependency audit. The boundary demo was built during the preceding correctness
validation. This is evidence for reviewing an experimental engine, not for enabling it by default
or claiming GPU/battery improvements.
