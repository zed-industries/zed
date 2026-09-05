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

App maintains forward and reverse render dependencies. Nested entity-access
collectors include descendant reads in their parents, preserving ancestor
invalidation. Entity notifications traverse the same dependency graph for models,
views, local state, and nodes. Rebuilding replaces dependencies; unmounting removes
them. Entity mutation revisions also reject reuse after an unnotified update when
some other input causes a frame. Notifications emitted during drawing invalidate recordings for the next
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
- [ ] Evaluate writing effects directly into retained storage, avoiding the active
  frame-to-recording capture pass while preserving existing frame/replay APIs.
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
  counter is compiled only into GPUI's unit-test executable. Do not use its
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

## Review validation

The retained engine passes the GPUI suite (251 tests, two manual benchmarks
ignored) and the editor suite (899 tests, one ignored). The workspace suite
(238 tests) passed before the final allocation optimizations. The GPUI suite also
passes with the legacy engine. The twelve focused node-engine tests pass across
20 scheduler iterations.

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
