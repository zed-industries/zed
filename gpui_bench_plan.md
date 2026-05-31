# `#[gpui::bench]` + `BenchAppContext` plan

## Motivation

Zed has no first-class way to benchmark and regression-gate the work that actually
drops frames: synchronous main-thread cost and the fan-out a single state change
triggers. We currently reach for `criterion` + `TestAppContext`, then fall back to
`xctrace` + `dsymutil` for any real attribution. That workflow is slow, manual, and
its numbers are misleading.

This came out of the agent `edit_file_tool` performance work. The pathological case was
applying each `CharOperation` as its own `buffer.edit` transaction, so one edit
fanned out into hundreds of `BufferEvent::Edited` events, each triggering a
tree-sitter reparse, an LSP `didChange`, the action-log diff, and the editor's
on-edit observers (matching brackets, bracket colorization, code actions, outline).
We only found it by profiling after the fact. We want to catch this class of bug in
CI, before it ships.

### What's wrong with the current tooling

- **`TestScheduler` is single-threaded.** Foreground and background runnables share
  one queue on one thread (`crates/scheduler/src/test_scheduler.rs`,
  `schedule_background_with_priority`). So a benchmark cannot separate "blocked the
  main thread" from "ran off-thread." Our `large_multi_edit` number (~919 ms) mixes
  both, even though the diff and LSP work run off-thread in production.
- **No per-update timing.** To see where time went we had to `dsymutil` a 145 MB
  binary and parse a Time Profiler XML export by hand.
- **No cascade visibility.** Nothing reports "this edit emitted N events and fired M
  observers" — the exact metric that would have caught the footgun deterministically.
- **No frame metric.** "Would this drop a frame" had to be computed offline from
  miniprof spans.
- **Harness artifacts skew absolute numbers.** Maximized window lays out far more
  lines than a real pane, the headless harness repaints per edit (the real app
  coalesces to one paint per frame), and per-iteration setup (project/editor/LSP
  construction) pollutes the samples.

The current bench is good for **relative** before/after comparisons, but its absolute
breakdown overstates editor rendering and conflates threads.

## Goals

- Measure **foreground-thread blocking time** truthfully (separate from offloaded work).
- Make the **update/effect cascade observable and assertable** (events, observers,
  transactions, reparses, re-renders, allocations).
- Produce a **frame-drop metric** against a budget.
- Lean on `criterion` for statistics, sampling, baselines, and reporting rather than
  reimplementing them.
- Emit **profiles viewable in Tracy / Perfetto** so drill-down doesn't require
  `xctrace` + `dsymutil`.
- Enable **deterministic regression gates** in CI.

## Non-goals

- Replacing `criterion`'s statistics engine.
- Generating standalone Instruments `.trace` files (not a writable format; see below).
- A general-purpose APM/tracing framework. This is a test/bench harness.

## Proposed solution

A `#[gpui::bench]` macro that provides a `BenchAppContext` and a
`BenchBencher`, layered on top of `criterion`. `BenchAppContext` owns app,
window, scheduler, recorder, and teardown state. `BenchBencher` stays close to
Criterion's `Bencher` API while adding GPUI-specific helpers for rendering
frames, measuring foreground time, and running mounted views.

The core design is **one backend-agnostic span + counter recorder**, with the
measurement frontends (Criterion) and the trace exporters (Tracy/Perfetto) both
reading from it. Capture once, surface many ways. Criterion still owns sampling,
statistics, baselines, and reporting for a single scalar measurement at a time;
GPUI sidecar reports and traces carry the richer multi-metric data.

## Capabilities

### Criterion-backed measurements

- **Wall time first.** The initial version should use Criterion's normal wall-clock
  measurement so the macro feels familiar and the PoC remains small.
- **Foreground time and missed frames next.** Phase 2 should add foreground busy time
  and frame-drop / missed-frame measurements, probably via `iter_custom` before a
  custom `Measurement` implementation.
- **Multiple requested measurements become multiple Criterion benchmarks.** Criterion
  models one scalar per benchmark result, so a single GPUI workload requesting
  multiple measurements should expand to distinct benchmark IDs such as
  `render_button/wall_time`, `render_button/foreground_time`, and
  `render_button/missed_frames`.
- **Rich reports stay out of Criterion's scalar model.** A `BenchReport` can contain
  wall time, foreground time, missed frames, spans, counters, allocation stats, and
  render details, but Criterion should analyze one chosen scalar per run.
- **Later measurement backends.** Allocation bytes, longest foreground span,
  renderer-blocking time, CPU cycles, instructions, cache misses, and branch misses
  can be added later where platform support exists.

### GPUI recorder

- **Backend-agnostic spans and counters.** Record once, then feed Criterion,
  Perfetto, miniprof/Tracy, and sidecar JSON/Markdown reports.
- **Per-update timing.** Reuse the miniprof hook (`TaskTiming` /
  `GLOBAL_THREAD_TIMINGS`, recorded in dispatcher trampolines) and extend it to
  `App::update`, entity updates, window updates, and `flush_effects`, attributed by
  `#[track_caller]` call site.
- **Update/effect cascade counts.** Per outer update / `flush_effects` cycle: update
  calls, entity updates, nested update depth, effects queued/flushed, events emitted,
  observer/subscription callbacks fired, entities notified, global notifications,
  windows invalidated, re-renders triggered, deferred callbacks, and action dispatches.
- **Notify attribution.** Track explicit `Context::notify()` separately from total
  `App::notify(...)` calls. Some paths, such as `GpuiBorrow` drop, notify implicitly
  without going through `Context::notify`; the report should make that distinction
  visible.
- **Extensible crate-specific counters.** GPUI owns generic app/render/frame metrics.
  Editor, terminal, language, project, and agent crates should add domain-specific
  counters through a generic span/counter API instead of baking Zed-specific metrics
  into GPUI.
- **Low overhead when disabled.** Hooks sit on hot paths, so instrumentation must be
  feature-gated and effectively zero-cost outside benchmark/instrumentation builds.

### Frame and render instrumentation

- **Foreground-blocking / inter-frame time / frame drops.** With a real scheduler and
  a modeled frame cadence, bucket foreground occupancy into frames against a budget
  (16.67 ms / 8.33 ms) and report longest contiguous span, renderer-blocking time,
  missed frames, and worst frame delay.
- **Realistic frame loop + window sizing.** Model normal pane/window sizes and
  coalesce draws to a vsync cadence, removing maximized-window and per-edit-repaint
  artifacts.
- **Render pipeline spans.** Measure layout, prepaint, paint, scene construction, and
  later backend-specific renderer work. Attribute these spans to windows, entities,
  and call sites where possible.
- **Render cache/reuse metrics.** Track view cache reuse, dirty subtree size, layout
  cache hits/misses, text/glyph/image/SVG/path cache hits/misses, shaped text runs,
  paths built/tessellated, scene command count, and unchanged subtree skips as the
  render pipeline instrumentation matures.
- **First frame vs steady state.** Make cold first-frame render, warmed steady-state
  render, and incremental-update render distinct benchmark modes.

### First-class render benchmark API

- **Mounted views.** Provide an ergonomic `cx.mount_view(...)` / `cx.render_entity(...)`
  API that mounts an `Entity<T>` where `T: Render` in a benchmark window and returns a
  `MountedView<T>`.
- **Renderer iteration helpers.** `BenchBencher` should provide helpers such as
  `bench_renderer(&mut MountedView<T>, ...)` that run user code between frames, flush
  effects, render one or more frames, and record layout/prepaint/paint/frame metrics.
- **Actions and updates between frames.** Mounted views should support dispatching
  actions, updating the mounted entity, resizing the window, warming caches, rendering
  a single frame, and rendering every frame in a modeled frame loop.

### Trace export and artifacts

- **Trace export.** Emit miniprof JSON for Tracy import and Chrome/Perfetto JSON for
  timeline drill-down.
- **Artifact layout.** Store GPUI artifacts next to Criterion output so local runs and
  CI uploads are easy to find: trace files, sidecar `BenchReport` JSON, and a concise
  Markdown summary.
- **Regression gates.** Provide count-based and frame-count assertion helpers for
  deterministic CI gates; use Criterion baselines for tracked latency trends, not hard
  pass/fail wall-clock gates.

### Optional platform counters

- **Hardware counters.** On platforms that support it, add optional measurements for
  cycles, instructions, cache misses, and branch misses. Linux can use perf counters;
  macOS should initially rely on Instruments / `xctrace` rather than first-class
  portable hardware-counter support.

## Criterion integration

`criterion` only models **one scalar per iteration**, but it gives us a lot we should
not reimplement: warmup, adaptive sampling, outlier detection, summary statistics,
baseline save/compare with change detection (p-values), CLI/HTML reporting, and
profile-mode execution.

The GPUI API should stay close to Criterion so existing Criterion users can onboard
quickly:

```rust
#[gpui::bench(sample_size = 20, measurement = wall_time)]
fn render_button(bencher: &mut BenchBencher<'_>, cx: &mut BenchAppContext) {
    let mut view = cx.mount_view(|window, cx| ButtonView::new(window, cx));

    bencher.bench_renderer(&mut view, |view, window, cx| {
        window.dispatch_action(...);
        view.update(cx, |button, cx| button.click(cx));
    });
}
```

### Initial version: normal Criterion wall time

The first implementation should inject `BenchAppContext` and `BenchBencher`, then let
`BenchBencher::iter` delegate to Criterion's normal `Bencher::iter`. This keeps the
PoC simple and produces ordinary Criterion output.

### Phase 2 seam: `iter_custom`

`iter` / `iter_batched` time wall-clock of the whole closure, which conflates
foreground work, background work, and harness work. `iter_custom` lets the harness
return the scalar Criterion should analyze. Phase 2 can use it for foreground time
and missed-frame measurements before committing to custom `Measurement` plumbing:

```rust
b.iter_custom(|iters| {
    cx.iter_foreground_time(iters, |cx| {
        run_workload(cx);
    })
});
```

Criterion then runs its statistics on the right number.

### Later: custom `Measurement`

`criterion` is generic over a `Measurement` trait (default `WallTime`; third-party
impls exist for CPU cycles / perf counters). Implementing measurements such as
`ForegroundTime`, `MissedFrames`, or `AllocatedBytes` makes Criterion report those
units natively. A Criterion group has one `Measurement`, so multiple requested GPUI
measurements should generate multiple Criterion benchmark IDs or groups.

A rich `BenchReport` may contain every metric from a run, but Criterion still reduces
one selected measurement to `f64` for statistical analysis. Use sidecar reports and
traces for the full multi-metric data.

### Later: Criterion `Profiler` / `--profile-time`

Criterion supports `--profile-time N`, which runs each benchmark workload for about
`N` seconds without normal sampling/statistical analysis so an external or in-process
profiler can collect data. Its `Profiler` trait has `start_profiling` and
`stop_profiling` hooks with the benchmark ID and output directory. A later GPUI
integration should use this to enable the recorder for profile-mode runs and write
Perfetto/miniprof/Tracy artifacts next to Criterion's output.

This is a later-phase integration because `Profiler` hooks do not receive
`BenchAppContext`; we need a clean bridge between Criterion's profile lifecycle and
GPUI's active recorder. Early versions can use a simpler non-timed trace pass.

### What stays out of Criterion

- **Counts** (events, transactions, reparses, notifies, observer callbacks) →
  deterministic assertion helpers / tests. These are less flaky as CI gates than
  time-based measurements.
- **Frame-drop lists / per-update breakdowns / traces** → `BenchReport` sidecars and
  trace artifacts, not Criterion's primary scalar result.

### Caveats

- A real thread pool widens Criterion's CIs and triggers more outlier warnings.
  Manage with more samples; fine for tracked trends.
- **CI gating on Criterion wall-clock is flaky** on shared runners. Gate CI on the
  **counts**; use Criterion's baseline/change-detection for tracked latency trends,
  not hard pass/fail.

## Trace export (Tracy / Perfetto / Instruments)

Once the harness captures spans + counters, exporting a profile is just a serializer.
Use one in-memory model with pluggable exporters.

- **Tracy — partly pre-built.** Zed already ships `tracy-import-miniprofiler`
  (`docs/src/performance.md`) that converts `*.miniprof.json` → a Tracy capture, plus
  a `tracy` feature (`ztracing/tracy`). If `BenchAppContext` emits the same miniprof
  JSON schema, the existing importer and the analysis tooling work on bench output
  for free. Richer Tracy use (zones/frames/plots) needs a small importer extension.
- **Chrome Trace / Perfetto — best portable default.** A plain JSON array of
  `{name, ph, ts, dur, pid, tid, args}` opens directly in `ui.perfetto.dev` with no
  importer and no macOS dependency. It maps cleanly onto what we care about:
  - nested duration events → the update / `flush_effects` / layout / prepaint /
    paint hierarchy,
  - **flow events (`ph: "s"/"f"`) → the cascade chain** (edit → arrows to each
    triggered effect), the unique thing Criterion can't express,
  - **counter events (`ph: "C"`) → the count metrics** as area-graph tracks.
- **Criterion profile-mode artifacts — later.** A polished implementation should use
  Criterion's `Profiler` hooks during `--profile-time` runs to write GPUI artifacts
  into the benchmark output directory. This is not required for the initial wall-time
  PoC because bridging Criterion's profile lifecycle to the active `BenchAppContext`
  recorder needs additional design.
- **Instruments — qualified.** You can't synthesize a `.trace` bundle from data.
  Realistic options: record the bench binary under `xcrun xctrace record` (works
  today; what we did), or emit `os_signpost` intervals that show in Instruments
  _during a recording_. "Generate a file to open in Instruments" is not practical.

## The determinism principle

Real concurrency trades away deterministic ordering, which is what makes wall-clock
perf gates flaky. Split the two:

- **Times** (foreground-blocking, frame timings) → advisory + tracked trends via
  Criterion; do not hard-gate CI on them.
- **Counts** (events, observers, transactions, reparses, allocations) → deterministic
  even with a real pool; these are the actual CI gates.

The headline value is not "faster timing"; it is **making the fan-out observable and
assertable.**

## Phased plan

### Phase 1 — Foundation: Criterion wall-time PoC

Establish the harness scaffolding so every later metric is an incremental add, not a
rewrite. Keep this phase small and Criterion-like.

- `BenchAppContext` as a benchmark-specific app context, distinct from
  `TestAppContext`, owning app/window/scheduler setup and teardown outside the timed
  region.
- `BenchBencher` as a Criterion-like wrapper around `criterion::Bencher`, initially
  delegating to normal wall-time `iter` / `iter_batched` APIs.
- A `#[gpui::bench]` macro that expands to Criterion `bench_function` plumbing,
  injects `BenchAppContext` and `BenchBencher`, and supports basic Criterion-like
  options such as `sample_size` over time.
- Initial measurement: **wall time only**.
- First consumer: port a small existing benchmark to prove one command builds, runs,
  and prints normal Criterion stats.

Outcome: GPUI benchmarks look familiar to Criterion users and can run with a real
`BenchAppContext`, but no deep GPUI metrics are required yet.

### Phase 2 — Mounted render benchmarks + foreground/frame measurements

Make rendering a specific entity first-class and add the first GPUI-specific scalar
measurements.

- `MountedView<T>` for an `Entity<T>` where `T: Render`, mounted in a benchmark
  window with controlled size/theme/font setup.
- `cx.mount_view(...)` / `cx.render_entity(...)` helpers for quickly creating render
  benchmarks without manually wiring windows.
- `BenchBencher::bench_renderer(&mut MountedView<T>, ...)`, rendering one frame per
  iteration after running user-provided code between frames.
- Helpers for dispatching actions, updating the mounted entity, resizing the window,
  rendering a single frame, warming caches, and rendering a modeled frame loop.
- `BenchScheduler` or equivalent foreground/background separation sufficient to
  compute foreground busy time.
- `foreground_time` and `missed_frames` measurements, initially via `iter_custom`.
- Basic frame model: frame cadence, longest foreground span, missed frames, and worst
  frame delay.

Outcome: agent panel, terminal, editor, and simple GPUI component render benchmarks
can be written ergonomically, and numbers begin to reflect main-thread blocking and
frame impact rather than only wall-clock time.

### Phase 3 — Cascade instrumentation + regression gates (highest value)

Add deterministic fan-out visibility and assertion helpers.

- Per-outer-update / `flush_effects` counters: update calls, entity updates, nested
  depth, effects queued/flushed, events emitted, observer/subscription callbacks,
  explicit `Context::notify()` calls, total `App::notify(...)` calls, implicit notify
  paths such as `GpuiBorrow` drop, window invalidations, and re-renders.
- Generic span/counter API for domain-specific metrics from editor, terminal,
  language, project, and agent crates.
- Allocation counting.
- Count-based `assert!` helpers and the first regression tests (e.g. "an N-op edit
  emits one `Edited` per chunk, not per op").
- Surface counters as Perfetto counter tracks / Tracy plots.

Outcome: the fan-out footgun class is caught deterministically in CI.

### Phase 4 — Render pipeline instrumentation

Add the detail needed to improve GPUI's renderer itself.

- Layout, prepaint, paint, scene construction, and render-backend spans.
- Entity/window/callsite attribution for render pipeline spans.
- View cache reuse, dirty subtree size, layout cache hits/misses, text/glyph/image /
  SVG/path cache hits/misses, shaped text runs, paths built/tessellated, scene command
  count, and unchanged subtree skips.
- Distinct cold first-frame, warmed steady-state, and incremental-update render modes.

Outcome: GPUI render pipeline changes can be benchmarked directly and attributed to
specific phases and cache behavior.

### Phase 5 — Trace/export polish and advanced measurements

- Trace exporters: **miniprof JSON** (reuse `tracy-import-miniprofiler`) and
  **Chrome/Perfetto JSON** with spans, flows, and counters.
- Criterion `Profiler` integration for seamless `--profile-time` trace artifacts.
- Macro ergonomics and Criterion-like parameters: `sample_size`, `warm_up_time`,
  `measurement_time`, `app = fresh_per_sample`, `drain = after_iteration`, window
  size, frame budget, pool size, and seed.
- Optional custom Criterion `Measurement` implementations for foreground time,
  missed frames, allocation bytes, renderer-blocking time, and platform hardware
  counters.
- CI integration: count gates as required checks; Criterion baselines tracked for
  trends.
- Richer Tracy export (zones/frames/plots) if the importer extension is worth it.

## Open questions / risks

- How much determinism to keep with a real pool. A hybrid (real background pool +
  deterministic foreground driver, optional seed) may be the sweet spot.
- Instrumentation must be zero-cost when the bench/instrumentation feature is off,
  since the hooks sit on gpui hot paths.
- CI noise budget for any time-based signal; lean on counts for gating.
- Whether to extend `tracy-import-miniprofiler` for zones/plots or just rely on
  Perfetto for the rich view.
- How to bridge Criterion `Profiler` lifecycle hooks to the active `BenchAppContext`
  recorder without global state that makes parallel or multi-app benchmarks fragile.

## Reference points in the existing codebase

- Per-poll timing hook: `crates/gpui_macos/src/dispatcher.rs` (`trampoline`,
  `TaskTiming`, `add_task_timing`, `GLOBAL_THREAD_TIMINGS`).
- Spawn-location attribution: `crates/scheduler/src/scheduler.rs`
  (`RunnableMeta { location }`).
- Single-threaded test scheduling to replace for benches:
  `crates/scheduler/src/test_scheduler.rs`.
- Effect system to instrument for cascade counts: `gpui` `App::flush_effects` /
  `pending_effects` / the `Effect` enum.
- Existing miniprof + Tracy import path: `crates/miniprofiler_ui/` and
  `docs/src/performance.md`.
- Current PoC bench: `crates/editor/benches/editor_render.rs`.
- Near-term consumers: agent panel render benchmarks, terminal render benchmarks,
  and `crates/agent/benches/edit_file_tool.rs` (already `harness = false`,
  `criterion`, `--profile-time` compatible).
