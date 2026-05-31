# `#[gpui::bench]` + `BenchAppContext` plan

## Motivation

Zed has no first-class way to benchmark and regression-gate the work that actually
drops frames: synchronous main-thread cost and the fan-out a single state change
triggers. We currently reach for `criterion` + `TestAppContext`, then fall back to
`xctrace` + `dsymutil` for any real attribution. That workflow is slow, manual, and
its numbers are misleading.

This came out of the agent `edit_file` performance work. The pathological case was
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

A `#[gpui::bench]` macro that provides a `BenchAppContext`, layered on top of
`criterion`. The macro owns app/window/scheduler setup (excluded from timing),
exposes richer measurement than wall-clock, and can drop a trace next to the
criterion results.

The core design is **one backend-agnostic span + counter recorder**, with the
measurement frontends (criterion) and the trace exporters (Tracy/Perfetto) both
reading from it. Capture once, surface many ways.

## Capabilities

1. **Real thread pool.** Run background work on real threads via a `BenchScheduler`
   (the `Scheduler` trait already abstracts this; `PlatformScheduler` is the
   production impl). This lets us measure only foreground occupancy, fixing the
   single-thread conflation.
2. **Per-update closure timing.** Reuse the miniprof hook
   (`TaskTiming`/`GLOBAL_THREAD_TIMINGS`, recorded in the dispatcher trampoline) and
   extend it to the `cx.update` / `Context::update` / `flush_effects` entry points,
   attributed by `#[track_caller]` call site.
3. **Update/effect cascade counts.** Per `flush_effects` cycle: events emitted,
   observer/subscription callbacks fired, entities notified, windows invalidated,
   re-renders triggered, buffer transactions, tree-sitter reparses. These are
   **deterministic** and make the best regression gates.
4. **Foreground-blocking / inter-frame time / frame drops.** With a real scheduler
   and a modeled frame cadence, bucket foreground occupancy into frames against a
   budget (16.67 ms / 8.33 ms) and report longest contiguous span + frames dropped.
5. **Allocation counting.** Allocations/bytes per update via a counting allocator,
   to catch alloc storms.
6. **Realistic frame loop + window sizing.** Model a normal pane size and coalesce
   draws to a vsync cadence, removing the maximized-window / per-edit-repaint
   artifacts.
7. **Trace export.** Emit Tracy (via the existing importer) and Chrome/Perfetto JSON
   for timeline drill-down.
8. **Regression gates.** Count-based `assert!`s that fail the build on fan-out
   regressions.

## Criterion integration

`criterion` only models **one scalar per iteration**, but it gives us a lot we should
not reimplement: warmup, adaptive sampling, outlier detection, summary statistics,
baseline save/compare with change detection (p-values), and CLI/HTML reporting.

### The seam: `iter_custom`

`iter`/`iter_batched` time wall-clock of the whole closure, which is what conflates
threads. `iter_custom` instead lets the harness return the `Duration`, so the
`BenchAppContext` runs the workload on a real pool and hands criterion only the
**foreground-blocking time**:

```rust
b.iter_custom(|iters| {
    let mut foreground = Duration::ZERO;
    for _ in 0..iters {
        let mut cx = BenchAppContext::new();      // setup excluded
        cx.run_workload(|cx| run_streamed_edit(cx));
        foreground += cx.foreground_busy_time();  // harness-measured, not wall-clock
    }
    foreground
});
```

Criterion then runs its statistics on the right number. This is a small step from the
`iter_batched` we already use.

### The clean version: a custom `Measurement`

`criterion` is generic over a `Measurement` trait (default `WallTime`; third-party
impls exist for CPU cycles / perf counters). Implementing it for "foreground busy
time" makes criterion report and graph **"foreground ms"** natively. One
`Measurement` per group, so it's one metric per run (run separate groups for
wall-time vs foreground-time vs alloc-count).

### What stays out of criterion

- **Counts** (events, transactions, reparses, allocations) → plain `#[test]`
  assertions. Deterministic, and far less flaky as CI gates than wall-clock.
- **Frame-drop lists / per-update breakdowns / traces** → diagnostics or a
  side-channel artifact, not criterion measurements.

### Caveats

- A real thread pool widens criterion's CIs and triggers more outlier warnings.
  Manage with more samples; fine for tracked trends.
- **CI gating on criterion wall-clock is flaky** on shared runners. Gate CI on the
  **counts**; use criterion's baseline/change-detection for tracked latency trends,
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
  - nested duration events → the update / `flush_effects` / render hierarchy,
  - **flow events (`ph: "s"/"f"`) → the cascade chain** (edit → arrows to each
    triggered effect), the unique thing criterion can't express,
  - **counter events (`ph: "C"`) → the count metrics** as area-graph tracks.
- **Instruments — qualified.** You can't synthesize a `.trace` bundle from data.
  Realistic options: record the bench binary under `xcrun xctrace record` (works
  today; what we did), or emit `os_signpost` intervals that show in Instruments
  _during a recording_. "Generate a file to open in Instruments" is not practical.

## The determinism principle

Real concurrency trades away deterministic ordering, which is what makes wall-clock
perf gates flaky. Split the two:

- **Times** (foreground-blocking, frame timings) → advisory + tracked trends via
  criterion; do not hard-gate CI on them.
- **Counts** (events, observers, transactions, reparses, allocations) → deterministic
  even with a real pool; these are the actual CI gates.

The headline value is not "faster timing"; it is **making the fan-out observable and
assertable.**

## Phased plan

### Phase 1 — Foundation: criterion + Tracy plumbing (do this first)

Establish the harness scaffolding so every later metric is an incremental add, not a
rewrite. No new scheduler or metrics yet.

- `BenchAppContext` wrapping `AppContext` or `App`, owning setup outside the timed region
  and the leak-detector/window teardown.
- A backend-agnostic span recorder seeded from the existing miniprof timing data.
- `criterion` wired through `iter_custom` (start with wall-clock of the measured
  region; the number gets more accurate in Phase 2).
- A `#[gpui::bench]` macro that expands to a `criterion_group!` + `bench_function`,
  injects the `BenchAppContext`, and on a separate non-timed pass dumps a trace.
- Trace exporters: **miniprof JSON** (reuse `tracy-import-miniprofiler`) and
  **Chrome/Perfetto JSON**.
- First consumer: port the existing `crates/agent/benches/edit_file_tool.rs` to the
  macro and confirm we can open its trace in Tracy/Perfetto.

Outcome: one command runs the bench, gets criterion stats, and drops a trace you can
open in Tracy or Perfetto. Everything below plugs into this.

### Phase 2 — Truthful foreground time

- `BenchScheduler` running a real background thread pool.
- `foreground_busy_time()` measurement; optionally a custom criterion `Measurement`
  so criterion reports "foreground ms" natively.

Outcome: numbers separate main-thread blocking from offloaded work (fixes the
single-thread conflation).

### Phase 3 — Cascade instrumentation + regression gates (highest value)

- Per-`flush_effects` counters: events emitted, observer callbacks, notifies, window
  invalidations, re-renders, buffer transactions, reparses.
- Allocation counting.
- Count-based `assert!` helpers and the first regression tests (e.g. "an N-op edit
  emits one `Edited` per chunk, not per op").
- Surface counters as Perfetto counter tracks / Tracy plots.

Outcome: the footgun class is caught deterministically in CI.

### Phase 4 — Frame realism

- Realistic window sizing (normal pane, not maximized).
- Modeled frame loop with coalesced draws.
- Frame-drop metric (longest foreground span per frame, frames over budget).

Outcome: editor-render cost reflects what users feel, not a maximized-window per-edit
repaint.

### Phase 5 — Polish

- Macro ergonomics, parameters (window size, frame budget, pool size, seed).
- CI integration: count gates as required checks; criterion baselines tracked for
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
- Current bench to port first: `crates/agent/benches/edit_file_tool.rs`
  (already `harness = false`, `criterion`, `--profile-time` compatible).
