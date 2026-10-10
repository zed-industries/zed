# Retained rendering stage 3: replay segments

## Provenance

- Worktree: `/home/anth/.local/share/delta-trunk/worktrees/d3h3dgh7wpdv/zed`.
- Branch: `gpui-retained-segments`.
- Stage 2 base: `008ecc8855031b54c653cac6190ac2820f6f99ec`.
- Implementation and regression tests: `7d58a867dfd416ebd663e3f678c1de1cde4260a9`.
- Untimed counter fixture: `630ecefd36148a11f68e8e4bb16867c644b8226f`.
- Baseline smoke: base plus only the counter fixture, diagnostic detached commit
  `a8505e1c3c`. It is not part of the candidate branch.
- Linux x86_64, Rust 1.98.1. All Cargo commands used the private reflink-seeded
  `$PWD/target/private`; tracked workspace sources were touched after seeding
  and after each source switch. Neither baseline nor candidate used a shared
  target executable.
- Shared experiment lock: `flock -s /tmp/gpui-experiments.lock`.
  Test/smoke CPU: 7; clippy/check CPU: 8; `CARGO_BUILD_JOBS=6`.
  Builds had a 900-second timeout, runs a 300-second timeout.
- Local ignored logs: `target/private/counters-before.log`,
  `target/private/counters-after.log`, and `target/private/tests-{layout}-{splice}.log`.

## Implementation

Dispatch topology is a separate plain array with node-relative parent offsets.
Replaying a range bulk-copies that array, moves the three listener vectors,
and clones key contexts so the old frame can still answer key-context queries.
Sorted focus-node and view-node side lists limit map translation to those nodes.
Internal parent offsets do not change. A topology scan discovers external edges
and remaps only those edges; a fragment can be a forest with several different
external parents, not just one subtree root. Replay does not rebuild dispatch
stacks or clone listeners.

Element-state access keys, hitboxes, and cursor requests use `extend_from_slice`.
Hitbox retains its public Clone-only API as requested by the architect; its
derived clone has only plain fields. Tooltip, input-handler, and mouse-listener
slots move with `Option::take`. Deferred draws move captured ID/style stacks,
the resolved style `Rc`, and entity-access sets. Their old parent IDs and ranges
remain readable.

Frame finish builds a set of accessed keys, then uses one `HashMap::extract_if`
pass to transfer the old states that remain live. Duplicate access records
do not cause repeated hash-map removals. Unaccessed states remain in the old
frame until clear after swap, preserving their drop timing.

Scene operations are Copy records. Plain primitives remain inline; paths and
surfaces have index/ID, bounds, and mask records referring to the typed arrays.
Replay bulk-copies the operation range, then recomputes painter order with the
existing BoundsTree insertion. Path vertices move from the old typed array;
neither insertion nor replay deep-clones paths. The old path's ID, bounds, mask,
and other scalar fields remain. Post-sort side indexes resolve path IDs and
surface identities, including repeated finish calls.

Debug builds reject overlapping consumption in dispatch, scene, and frame
payload ranges. These bookkeeping records are cleared with the frame.
Splicing-disabled cached reuse uses the same internals.

Remaining clones:

- KeyContext values remain independently readable in both frames. Their entries
  can carry SharedString references. These are separately instrumented now.
- PaintSurface is cloned once on replay. Its public native buffer is non-null,
  so moving it would require changing the public type or inventing a replacement
  buffer. There is no longer a second insertion clone.
- Test-only debug-bound selector Strings are still cloned.
- Tab-stop and text-layout replay internals are unchanged.

No `view.rs`, splice entry points, BoundsTree search, README, or public Copy
derives were changed. The test fixture assertion was updated for the new copied
counter, and its differential sweep now explicitly tests both splicing modes.

## Matched untimed counters

`retained_segment_leaf_style_counter_smoke` samples the same five CachedTree
families with the same FNV name mixing, seed 3, and 16 leaf-color mutation frames
per family per mode. Setup frames are excluded. `GPUI_LAYOUT_CACHE=1`.
It runs through TestAppContext, not Criterion: there is no timed benchmark or
calibration. Thus these numbers are not expected to equal the historical stage 1
quick/calibration averages.

Each cell is **elements / inline bytes per frame**, averaged equally across the
five families. These counters describe logical replay records, not heap payload
bytes or physical memory traffic. Dispatch has one copied topology record and
one moved listener-vector tuple per logical node; do not add those element counts
as if they were distinct nodes. Context counters were not separate in the
baseline, so their before cells are `—`, not a claim of zero cloning.

Before/after logical replay counts agree for every family and both modes.
Total index fixups per frame fall from 245.91 to 121.22 with splicing disabled,
and from 310.35 to 159.51 with splicing enabled. These totals include scene
painter-order fixups, which intentionally remain per primitive.

### Splicing disabled

| Array | Before cloned | Before moved | Before rebuilt | After copied | After moved | After cloned | After rebuilt |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Element-state keys | 389.81 / 12474.00 | 0 | 0 | 389.81 / 12474.00 | 0 | 0 | 0 |
| Dispatch nodes | 0 | 0 | 151.14 / 19345.60 | 151.14 / 4836.40 | 151.14 / 10881.90 | 0 | 0 |
| Dispatch contexts | — | — | — | 0 | 0 | 18.85 / 452.40 | 0 |
| Quads | 94.78 / 15164.00 | 0 | 0 | 94.78 / 15164.00 | 0 | 0 | 0 |
| Hitboxes | 27.88 / 1338.00 | 0 | 0 | 27.88 / 1338.00 | 0 | 0 | 0 |
| Mouse-listener slots | 0 | 110.44 / 1767.00 | 0 | 0 | 110.44 / 1767.00 | 0 | 0 |
| Tooltips | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Cursor styles | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Input handlers | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Deferred draws | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Tab stops | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Text-layout usage | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Shadows | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Paths | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Underlines | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Monochrome sprites | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Subpixel sprites | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Polychrome sprites | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Surfaces | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### Splicing enabled

| Array | Before cloned | Before moved | Before rebuilt | After copied | After moved | After cloned | After rebuilt |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Element-state keys | 452.54 / 14481.20 | 0 | 0 | 452.54 / 14481.20 | 0 | 0 | 0 |
| Dispatch nodes | 0 | 0 | 178.82 / 22889.60 | 178.82 / 5722.40 | 178.82 / 12875.40 | 0 | 0 |
| Dispatch contexts | — | — | — | 0 | 0 | 22.43 / 538.20 | 0 |
| Quads | 131.53 / 21044.00 | 0 | 0 | 131.53 / 21044.00 | 0 | 0 | 0 |
| Hitboxes | 33.21 / 1594.20 | 0 | 0 | 33.21 / 1594.20 | 0 | 0 | 0 |
| Mouse-listener slots | 0 | 130.25 / 2084.00 | 0 | 0 | 130.25 / 2084.00 | 0 | 0 |
| Tooltips | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Cursor styles | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Input handlers | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Deferred draws | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Tab stops | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Text-layout usage | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Shadows | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Paths | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Underlines | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Monochrome sprites | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Subpixel sprites | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Polychrome sprites | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Surfaces | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Zero rows reflect this tree fixture, not absence of implementation or coverage.
Dedicated regressions exercise moved path vertices, cursor queries, and nested
deferred payloads; existing interaction/fallback tests exercise tooltips and input.

Reproduce smoke after building the library tests:

```sh
flock -s /tmp/gpui-experiments.lock taskset -c 7 \
  env CARGO_TARGET_DIR="$PWD/target/private" CARGO_BUILD_JOBS=6 GPUI_LAYOUT_CACHE=1 \
  timeout 300s cargo test --locked -p gpui --lib \
  retained_segment_leaf_style_counter_smoke -- --nocapture
```

## Correctness gates

- `cargo test --locked -p gpui --lib`: all four combinations of
  `GPUI_LAYOUT_CACHE=0/1` and `GPUI_VIEW_SPLICE=0/1` passed: **442 passed,
  2 existing timing-only tests ignored** per combination.
- Differential sweep: 4 topologies × 12 scheduler seeds × 16 frames,
  explicitly in **both** splicing modes; interaction and fallback tests pass.
- New regressions: focus/view maps and old dispatch paths after reuse;
  reuse of already-reused nodes; overlapping dispatch/frame/scene rejection;
  old-frame hit/focus/cursor queries during actual painting after payload moves;
  nested deferred payload movement and old parent paths; path allocation
  preservation, nested layers, painter-order sorting and repeated finish;
  state transfer without early destruction of unused old states.
- `./script/clippy --locked -p gpui`: passed (release, all targets/features,
  warnings denied). The script skips optional cargo-shear/typos/buf checks
  because cargo-shear is unavailable.
- `cargo fmt --all -- --check`: passed.
- `cargo check --locked -p gpui -p ui -p workspace -p editor`: passed.

One clippy attempt caught clone-on-Copy callers when Hitbox temporarily had a
Copy derive. The architect requested retaining its public API; the derive and
all mechanical caller changes were reverted before the passing checks.
A final compound validation call was interrupted after clippy completed; the
test matrix and workspace check were inspected and rerun to completion, followed
by another successful clippy and format check.
Initial smoke attempts exposed a test harness entity re-entry and redundant
manual draws; the fixture now lets the normal update/dispatcher draw each frame,
and verifies its 16-frame count.

Not run: timed benchmarks, native macOS/iOS surface execution, GPU pixel
comparison, or a complete Zed application build. No performance claim is made
until the architect's combined benchmark round.

## Full-retention follow-up

### Outcome and provenance

Warm Noto/96 draw p50 fell from the supplied full-retention baseline's
3.49–3.50 ms to 1.262–1.284 ms. The matched final candidate comparison is
7.549–7.619 ms with retention off versus 1.262–1.284 ms on.
The no-reuse overhead target was **not met**: first populated frames remain
4.6% slower, mixed full refresh 6.4% slower, and dense full refresh 23.0% slower.
CachedTree/mixed leaf instructions also regress; dense leaf p95 regresses
despite its large median improvement.

- Worktree: `/home/anth/.local/share/delta-trunk/worktrees/ev1qp56we1dh/zed`.
- Branch: `gpui-retained-next`; supplied base `50c6b95acb`, saved WIP `47fa268cae`.
- Measurement revision: `7da4f7036b`. Subsequent correctness-only revision
  `ce9c0a8840` rebases inherited layer depth in scene checkpoints and adds a
  regression test. The measured fixtures preserve enclosing layer depths.
  Measurements were not repeated after this safety fix.
- Implementation commits, in order:
  - `633f2d14d3`: preserve reachable custom callbacks and validate owner reads.
  - `7476fe1d83`: dense scope/ordinal ownership, repeated-mark stamps, phase timers.
  - `81a54a4c97`, `272c4cda62`: guarded root-layout skip and unrelated cleanup fix.
  - `075c6c4bc0`: boundary-only input/output snapshots, dense layouts, shared
    frame dependency union, and reused layout boxes.
  - `e1bd3a4239`: merge grid/replay commits `2a05aa1f28` and `cba575e1cf`.
  - `d895b3c7f1`: exact bulk scene replay from lazy typed snapshots.
  - `d16422cbdc`: stable bounded counting sort of `(draw order, tile_id)`.
  - `7da4f7036b`: sparse dispatch payload replay and text phase attribution.
  - `ce9c0a8840`: inherited-layer checkpoint safety fix.
- Linux x86_64, Rust 1.98.1, LLVM 22.1.8; production Linux text shaping,
  headless CPU scenes, no GPU presentation/pixel correctness claim.
- Private reflink-seeded `target/private`; workspace sources touched after seeding.
  No shared target binaries were trusted and the supplied base was not rebuilt
  or remeasured. Builds/tests used CPU8, shared lock, six build jobs, 900-second
  build and 300-second run limits.
- Benchmark: `gpui_platform/randomized_element_tree`, `release-fast`,
  `--no-default-features --features bench-support,x11`.
  Complete feature graph contains no `test-support`.
- Noto: `/usr/share/fonts/noto/NotoSansMono-Regular.ttf`, Regular, face 0,
  SHA-256 `65b5e2b2c4a1fba9ae8be1f026cb35b03dcb8886d9b2a4147054fde12f7e767d`.
- Benchmark source SHA-256
  `c94e2bd944847c141ff30a3fb0e6c0e668b286fb94f4ce05afe470ee3a2daec0`;
  lockfile SHA-256
  `a25b0e36e2cafe3d9b38ec79819f7ebc168fbd6f285b6f98dc05bad9e4e3dc41`.

### Correctness and retained-state rules

Custom callbacks retain their owning occurrence and separately register entity
and global reads during opaque measurement. `clear` keeps reachable callbacks;
replacement or removal releases their captures. Every speculative boundary
update must preserve its node, parent-facing style, exact layout input/output,
and allocated size before its write span can advance the root's accepted write
frontier. Unvalidated writes, font changes, refresh, root/viewport/scale changes,
and changed boundary outputs retain the full-root compute fallback.

Retained absolute bounds survive frame teardown. A root compute advances the
global bounds epoch; an in-place boundary compute invalidates only that subtree.
Ownership marking and reset use dense node records rather than scanning all
scope ordinals. Exact inputs/outputs are stored only for declared boundaries.

Ordinary boundary reads retain exact own sets, with one capacity-reused union
for window invalidation instead of transitive copies at every boundary.
Deferred draws still collect transitive sets for later replay. Required
next-frame boundary scopes and state are still recorded on refresh; blanket
omission would lose next-frame splicing.

Bulk scene reuse requires a layer-balanced segment without inherited layers
or path/surface payloads, and exact old orders validated against both sides of
every changed prefix bounds. Six Copy primitive arrays, operation records, and
ordering indices then append in bulk without clipping or Primitive conversion.
Typed snapshots are created lazily from immutable paint-order records, not on
first/full refresh. Unsupported or overlapping cases use the original replay.
The grid still registers bounds eagerly so later fresh inserts remain exact.

Final gates, rerun after `ce9c0a8840`:

- `cargo test --locked -p gpui --lib`, all six `GPUI_LAYOUT_CACHE=0/1` ×
  `GPUI_VIEW_SPLICE` unset/`1`/`all`: **467 passed, 2 existing timing tests ignored**
  in each mode. Both 12-seed differential sweeps are included.
- Targeted coverage: measure-owner entity/global reads and capture retirement;
  ownership marking/reset; root skip, nested resize and moved-sibling bounds;
  unchanged text panels; boundary-only input capture; seeded bulk/fallback scene
  replay across sorting and changed overlaps; stable high/sparse sprite keys;
  sparse dispatch payloads, old-frame queries, truncation and inherited layers.
- Both production-font 16-frame text differential smokes passed.
- `./script/clippy --locked -p gpui`, `cargo fmt --all -- --check`, and
  `cargo check --locked -p gpui -p ui -p workspace -p editor`: passed.
  Optional cargo-shear/typos/buf checks were not reached because cargo-shear
  is unavailable.
- Not run: full Zed build, native Metal rendering, GPU pixel oracle or samply.
  Glyph painting was not changed.

### Untimed phase attribution

`GPUI_RENDER_PHASE_TIMERS=1` enables bench-support-only `Instant` accumulators.
Below are one-frame Noto/96 CPU8 smoke totals in milliseconds, not measured
benchmark percentiles. They are inclusive and must not be added together:
dispatch is inside prepaint replay, and layout/replay/checks also appear in the
root phases. Shared CPU runs and the final differential warmup differ in cache
state, so the wall benchmark below is the latency evidence.

| Phase | Before ownership | After ownership | After root skip | Final |
| --- | ---: | ---: | ---: | ---: |
| Layout compute | 0.637 (3 calls) | 0.592 (3) | 0.573 (2) | 0.562 (2) |
| Retain marking | 1.862 | 0.077 | 0.079 | 0.098 |
| Boundary checks/scopes | 0.035 | 0.024 | 0.026 | 0.029 |
| Prepaint replay | 0.264 | 0.254 | 0.257 | 0.083 |
| Dispatch replay | 0.257 | 0.249 | 0.251 | 0.077 |
| Paint replay | 0.735 | 0.710 | 0.718 | 0.299 |
| Scene replay insertion, includes BoundsTree | 0.728 | 0.705 | 0.712 | 0.292 |
| Scene finish | 0.158 | 0.159 | 0.161 | 0.170 |
| Root request | 0.931 | 0.763 | 0.790 | 0.800 |
| Root prepaint | 2.244 | 0.512 | 0.496 | 0.170 |
| Root paint | 0.817 | 0.772 | 0.786 | 0.364 |

Final nested layout: TextMeasure 0.428 ms / 384 calls, actual platform shaping
0.356 ms / 64 misses, wrapping 0.007 ms / 64 calls. The layout-minus-measure
residual is approximately 0.134 ms, not a separately measured pure-Taffy timer.
The two changed panels share label strings, so their second copies hit the
line cache in the same frame. Numeric glyph synthesis from gpui-fast excludes
the fixture's `Metric NN: NNNNNN` strings; no fixture-specific synthesis was added.
BoundsTree-exclusive timing is not claimed: the caller timer includes the
whole scene replay insertion path.

### Final measurement protocol

Both reported modes use `GPUI_LAYOUT_CACHE=1`. OFF unsets `GPUI_VIEW_SPLICE`;
ON sets it to `all`. Phase timers and differential verification are disabled
for measured runs. Seeded groups use `SEED=3 ITERATIONS=1`.

Instructions: one process per mode, CPU8, `flock -s`, `BENCH_MEASUREMENT=instructions`,
0.2-second warmup, 1-second measurement, 10 samples, no plots.
Wall: one corrected exclusive CPU0 ABAB pass, `flock -x -w 600`,
`BENCH_MEASUREMENT=wall-time`, 0.5-second warmup, 1-second measurement, 20 samples.
Every benchmark invocation has a 300-second timeout.

An initial comparison mistakenly disabled the layout cache as well as retention.
The supplied reference logs confirmed that OFF keeps the cache on. Those
cache0 supplemental logs are excluded below; OFF instruction measurements and
the ABAB pass were corrected. This deviation adds an extra diagnostic pass.
All measurement/smoke/quick work, including the correction, took 9m14s, from
04:59:36 to 05:08:50 local time on 2026-10-10.

#### Process-wide instruction point estimates, millions

| Workload, seed 3 | OFF | ON | Change |
| --- | ---: | ---: | ---: |
| Full refresh / any | 4.1902 | 4.4210 | +5.5% |
| Full refresh / wide | 3.0871 | 3.2123 | +4.1% |
| Full refresh / tall | 1.7557 | 1.8275 | +4.1% |
| Full refresh / mixed | 10.8540 | 11.3220 | +4.3% |
| Full refresh / dense | 6.1442 | 7.1896 | +17.0% |
| Leaf style / any | 4.3322 | 3.8113 | -12.0% |
| Leaf style / wide | 3.0281 | 2.7908 | -7.8% |
| Leaf style / tall | 1.7714 | 1.4388 | -18.8% |
| Leaf style / mixed | 11.4390 | 7.7674 | -32.1% |
| Leaf style / dense | 6.3811 | 3.8756 | -39.3% |
| Cached leaf style / any | 4.1653 | 4.1206 | -1.1% |
| Cached leaf style / wide | 2.9220 | 2.9275 | +0.2% |
| Cached leaf style / tall | 1.4575 | 1.5181 | +4.2% |
| Cached leaf style / mixed | 6.6704 | 8.0753 | +21.1% |
| Cached leaf style / dense | 5.2643 | 3.4377 | -34.7% |
| Warm monospace / 96 | 120.550 | 19.937 | -83.5% |
| Warm Noto / 96 | 116.610 | 15.755 | -86.5% |
| First populated Noto / 96 | 224.060 | 236.290 | +5.5% |

#### Exclusive CPU0 draw, milliseconds

Each cell shows the two ABAB values. These draw reports include Criterion
warmup/calibration, not measurement-phase-only draws. Percentage change uses the
midpoint of the two p50s. Dense leaf updates span multiple paths: their low
median does not remove the slower tail.

| Workload | OFF p50 | ON p50 | Change | OFF p95 | ON p95 |
| --- | ---: | ---: | ---: | ---: | ---: |
| Warm Noto / 96 | 7.549 / 7.619 | 1.284 / 1.262 | -83.2% | 8.163 / 8.233 | 1.545 / 1.523 |
| Warm monospace / 96 | 7.696 / 7.692 | 1.523 / 1.502 | -80.3% | 8.282 / 8.569 | 1.850 / 1.840 |
| First populated Noto / 96 | 15.344 / 15.442 | 16.196 / 16.007 | +4.6% | 16.540 / 16.458 | 17.154 / 17.351 |
| Full refresh / mixed | 0.811 / 0.811 | 0.863 / 0.863 | +6.4% | 0.862 / 0.878 | 0.921 / 0.921 |
| Full refresh / dense | 0.484 / 0.485 | 0.597 / 0.595 | +23.0% | 0.526 / 0.531 | 0.645 / 0.649 |
| Leaf style / mixed | 0.800 / 0.797 | 0.848 / 0.847 | +6.1% | 0.864 / 0.862 | 0.968 / 0.972 |
| Leaf style / dense | 0.488 / 0.489 | 0.155 / 0.156 | -68.2% | 0.537 / 0.536 | 0.696 / 0.700 |

First-populated process-wide requested live heap, before validation logging:

| Metric | OFF | ON |
| --- | ---: | ---: |
| Before window | 7,942,412 B | 8,043,283 B |
| Empty window | 8,727,905 B | 8,834,776 B |
| Populated | 64,468,145 B | 66,343,972 B |
| Window + setup delta | 56,525,733 B | 58,300,689 B |
| Population-only delta | 55,740,240 B | 57,509,196 B |

Population-only ON overhead is 1,768,956 B / 3.2%. These estimates are not
exclusive ownership, RSS, peak heap, or GPU memory. Lazy replay snapshots have
not yet been created at first population.

Raw logs, feature graph, exact final test matrices, phase totals, and parsed
`measurements/summary.json` are under ignored `target/retained-next/`.
Remaining work is the refresh-boundary bookkeeping and CachedTree/mixed fallback
path, not a claim that the approximately 3% no-reuse target was achieved.

## Retained-overhead follow-up

### Outcome

The near-zero no-reuse target is **not achieved**. Dense refresh remains about
16% above the supplied OFF wall baseline, mixed refresh about 5%, and first
population is not improved. The adopted changes fix the mixed cached-tree
regression and the dense leaf tail without losing warm text reuse.

The adaptive recording and dense layout-storage experiments were rejected.
Retaining a hot child's placement is necessary to let its clean ancestors
splice around a fresh child; almost all eager boundary metadata in this base
is placement, not optional own-content reuse data.

### Provenance and commits

- Worktree: `/home/anth/.local/share/delta-trunk/worktrees/kyyxn50kg5vn/zed`.
- Branch: `gpui-retained-overhead`; supplied base `986a6c6f03`.
- Final measured code: `fdcee7eab8`.
- Commits:
  - `92f6e7840f`: fixed-frame phase traces and fine boundary attribution.
  - `91a0a77360`: preserve clean children across implicit reconstruction.
  - `d9c63d132a`: carry implicit occurrences without persistent element-state
    handles; explicit cached handles and their range conventions are unchanged.
  - `5b0dba3ca6`: recycle empty entity-read sets and directly index transitive
    dependency collectors.
  - `1d8bb6071d`: hoist disabled per-node timer checks.
  - `77b7c6cc66`: adaptive K=2 content eligibility with placement preserved.
  - `5980d82caa`: reject/revert the adaptive experiment after its screen.
  - `fdcee7eab8`: isolate fine scope-field timers in an opt-in profiled helper.
- No base rebuild or base remeasurement, push, PR, subagents or README marker.
- Same Linux CPU/text/headless platform and font as the preceding report.
  No native GPU presentation or pixel-correctness claim.
- Reflink-seeded private `target/private`; all workspace Rust/manifests touched
  immediately after seeding. All Cargo commands used that private directory.
- Shared locks, CPU8/9, six build jobs; 900-second build/check limits and
  300-second benchmark limits. Wall runs held the exclusive lock on CPU0.
- The full benchmark feature graph contains no `test-support`.
- Benchmark source SHA-256:
  `33d658ad2d56b4985ca8e774047283d1111dfce1ec0c295de4f8b2c42f4d07ee`.
  Lockfile hash is unchanged from the preceding report.

### Attribution before changing behavior

OFF keeps `GPUI_LAYOUT_CACHE=1` and unsets `GPUI_VIEW_SPLICE`; ON uses `all`.
Tree traces use `SEED=3 ITERATIONS=1 GPUI_RETAINED_PHASE_FRAMES=32` with
Criterion `--test`, not a statistical benchmark. First population is one
explicit setup draw. Values below are **instrumented inclusive microseconds
per frame**, not benchmark latency or independently additive exclusive costs.
Nested timer entry/exit is included in enclosing phases.

| Workload | Phase | OFF | ON |
| --- | --- | ---: | ---: |
| Dense full refresh | Root request | 185.59 | 256.09 |
| | Root prepaint | 62.44 | 145.10 |
| | Root paint | 177.48 | 233.69 |
| | Boundary bookkeeping | 0 | 28.11 |
| | Dependency push + pop | 0 | 25.55 |
| | Scope capture | 0 | 8.90 |
| | Layout input/output capture | 0 | 7.21 |
| | SceneIndex recording | 0 | 3.79 |
| Mixed full refresh | Root request | 406.40 | 440.26 |
| | Root prepaint | 150.42 | 190.45 |
| | Root paint | 191.92 | 217.72 |
| | Boundary bookkeeping | 0 | 13.97 |
| | Dependency push + pop | 0 | 11.36 |
| | Scope capture | 0 | 4.24 |
| | Layout input/output capture | 0 | 4.23 |
| | SceneIndex recording | 0 | 1.75 |
| CachedTree mixed leaf style | Root request | 160.19 | 97.73 |
| | Root prepaint | 120.34 | 378.39 |
| | Root paint | 113.60 | 191.31 |
| | Layout compute, inside prepaint | 13.45 | 73.21 |
| | Boundary bookkeeping | 0 | 8.31 |
| | Dependency push + pop | 0 | 5.48 |
| Text first population | Layout compute | 12576.36 | 15548.42 |
| | Fine layout capture, inside layout | 0 | 1745.05 |
| | Boundary bookkeeping, includes growth | 0 | 58.76 |
| | ViewStore growth | 0 | 27.47 |
| | Dependency push + pop | 0 | 16.38 |
| | Scope capture | 0 | 6.92 |
| | SceneIndex recording | 0 | 2.46 |

Dispatch payload indexing was essentially shared work: dense refresh
2.89/2.96 µs, mixed refresh 4.35/4.34 µs, cached mixed leaf 2.71/2.63 µs,
text first population zero. ViewStore growth is zero on steady refreshes.
The text fine layout timer executes 86,413 times, versus 97 boundaries:
its 1745 µs is strongly affected by timer overhead and is **not** a claim that
snapshot bookkeeping adds 1.745 ms to production. It includes per-node layout
writes and boundary-input/output checks.

The major cached-tree regression was behavioral, not that small bookkeeping
residual: notifying an ordinary implicit root invented a refresh and forced
all explicit cached children to reconstruct. Ordinary ancestors in the OFF
path do not impose that invalidation contract.

Dense leaf slow frames likewise cascaded from a local failure to all 205
boundaries. Fixed 32-frame counts before/after the reconstruction fix:

| Trace | Mean boundary misses/frame before | After |
| --- | ---: | ---: |
| CachedTree mixed leaf | 55.91 | 3.94 |
| Ordinary mixed leaf | 61.13 | 47.97 |
| Dense leaf | 74.53 | 7.56 |

Dense `ExecutionScope` fallback events remain 19, but no longer rebuild the
whole tree. Dense `LayoutConstraints` events fall from 19 to zero. Mixed
`LayoutConstraints` events fall from two to zero; cached mixed
`ExecutionScope` events were three/five. Event counts can include multiple
failed attempts in a frame; they are not a count of distinct affected frames.

### Correctness and policy choices

Real refreshes and explicit cached misses still refresh descendants.
Reconstructing an ordinary implicit boundary does not. Own-dirty implicit
ancestors conservatively reconstruct their implicit descendants because
inherited paint opacity and visibility are not represented by the prepaint
reuse key. Explicit cached children retain the existing OFF opt-in contract.
Fallback-only clean ancestors can reuse unaffected implicit siblings.

Hidden ancestors prepaint children without painting them. Retention now
invalidates an implicit child's paint eligibility when its previous paint
range did not survive into the rendered frame. Removing the invented refresh
without this guard exposed stale ranges in the differential property tests.

No size heuristic or small-view eligibility threshold was adopted.

### Rejected experiments

The adaptive experiment counts fresh rendered frames, suspends own-content
eligibility after two consecutive renders, preserves placement and all
invalidation dependencies, and records once when it cools before own reuse
can resume. A hot child remains spliceable as a fresh child. The targeted
cooling test and all six modes passed, including both differential sweeps.

Fixed-sampling instruction screen, adaptive OFF/ON in the same binary:

| Workload | OFF M instructions | ON | Change |
| --- | ---: | ---: | ---: |
| Dense full refresh | 7.0665 | 7.0033 | -0.89% |
| Mixed full refresh | 11.609 | 11.475 | -1.15%, noisy |
| Warm Noto/96 | 15.732 | 15.734 | unchanged |

This fails the architect's 3% dense-refresh adoption gate and was reverted.
It saves little because typed scene snapshots are already lazy, while shared
scene/dispatch records must remain available for an ancestor's reuse.

A capacity-reused dense ViewLayout side array and revision-guarded exact
input/output captures passed the library/oracle tests but screened at
7.0222 M dense-refresh instructions versus 7.0265 M before (-0.06%).
It was not committed and was discarded.

Before adopting scope interning, individual fields were timed on 32 dense
refresh frames, 580 captures/frame:

| Scope component | Raw µs/frame | Minus matched empty-timer floor |
| --- | ---: | ---: |
| Text style / Rc | 8.547 | 2.634 |
| Identity/path/offset | 6.414 | 0.501 |
| Dispatch/parent | 6.121 | 0.208 |
| Content mask | 6.637 | 0.724 |
| Rem/scale/opacity/fallback | 6.439 | 0.526 |
| Image cache stack | 8.670 | 2.757 |
| Empty timer | 5.913 | 0 |

The subtraction is a diagnostic floor adjustment, not an exact exclusive
production measurement. Bookkeeping was 28.447 µs, dependencies 23.005 µs,
layout capture 6.570 µs and range indexing 3.787 µs in this trace. No individual
scope field exceeds 20% of placement cost. Full scopes also contain unique
occurrence, element and dispatch identities; blindly interning those would
not share sibling snapshots. A common-context interner draft was discarded
before building, per the architect's stop rule.

Final inline sizes are hot 32 B, cold 504 B, cache 568 B, layout 248 B, for
97 occurrences in TextPanels/96. The supplied 408/800 cache/layout estimates
do not describe these actual types. The rejected adaptive and dense-array
experiments do not change the final sizes.

### Final performance evidence

All statistical runs use the explicit equal **sampling configuration**
`--sample-size 10 --warm-up-time 1 --measurement-time 1 --noplot`, never
`--quick`. Criterion still chooses different actual iteration counts.
Phase timers, fixed-frame traces and differential verification are disabled.
The supplied base was not remeasured.

The table compares the supplied OFF/ON instruction point estimates with
the final candidate ON run at `fdcee7eab8`, in millions:

| Workload | Supplied OFF | Supplied ON | Final ON | Final vs supplied OFF |
| --- | ---: | ---: | ---: | ---: |
| Mixed full refresh | 10.8540 | 11.3220 | 11.485 | +5.8% |
| Dense full refresh | 6.1442 | 7.1896 | 7.0315 | +14.4% |
| CachedTree mixed leaf | 6.6704 | 8.0753 | 4.4069 | -33.9% |
| Mixed leaf | 11.4390 | 7.7674 | 6.7866 | -40.7% |
| Dense leaf | 6.3811 | 3.8756 | 2.1677 | -66.0% |
| Warm monospace/96 | 120.550 | 19.937 | 20.002 | -83.4% |
| Warm Noto/96 | 116.610 | 15.755 | 15.726 | -86.5% |
| First population Noto/96 | 224.060 | 236.290 | 237.020 | +5.8% |

An initial candidate OFF/ON comparison also measured 6.1502/7.0265 M dense
refresh, 6.7180/4.4132 M cached mixed leaf, and 11.454/6.7841 M mixed leaf.
Churn final ON point estimates, without a baseline claim: replace-8-of-96
125.83 M, remount-48-of-96 118.06 M, ID/anonymous-count 222.03 M, text-96
222.82 M.

CPU0 draw milliseconds, including Criterion warmup/calibration:

| Workload | Supplied OFF p50 | Supplied ON p50 | Candidate ON p50 | Candidate ON p95 |
| --- | --- | --- | --- | --- |
| Mixed refresh | 0.811 | 0.863 | 0.854 / 0.852 | 0.916 / 0.911 |
| Dense refresh | 0.484–0.485 | 0.595–0.597 | 0.564 / 0.565; final 0.562 | 0.611 / 0.610; final 0.608 |
| Cached mixed leaf | not supplied | not supplied | 0.371 / 0.372 | 0.485 / 0.481 |
| Mixed leaf | 0.797–0.800 | 0.847–0.848 | 0.300 / 0.298 | 0.901 / 0.895 |
| Dense leaf | 0.488–0.489 | 0.155–0.156 | 0.136 / 0.136 | 0.200 / 0.200 |
| Warm monospace/96 | 7.692–7.696 | 1.502–1.523 | final 1.501 | final 1.763 |
| Warm Noto/96 | 7.549–7.619 | 1.262–1.284 | final 1.255 | final 1.476 |
| First population Noto/96 | 15.344–15.442 | 16.007–16.196 | final 17.449 | final 18.432 |

Tree pairs are from the original candidate ABAB at `5b0dba3ca6`;
the final focused pass is at `fdcee7eab8`. Differences between these revisions
are bench-only timer guards/helpers, not production behavior. The first-frame
wall result regresses and is not accepted as an improvement: corrected text
passes also gave 16.941/16.417 ms ON and 15.303/16.114 ms OFF. No stable
first-frame reduction is established.

Mixed leaf is bimodal. Equal Criterion sample settings do not mean equal
mutation iteration counts: the initial ABAB observed 3092/2068 OFF draws and
3697/3697 ON. Its median can cross the inexpensive/whole-root boundary;
the p95 and fixed-frame traces must accompany that median. Corresponding
instruction-run secondaries were OFF/ON 4.876/3.033 M cycles,
21.408/15.646 K branch misses, 1.522/1.305 K cache misses, IPC 2.349/2.261.
These support less work overall, not a claim that every mutation is faster
or that the older wall/instruction disagreement was purely hardware.

Final first-population requested live heap is 57,497,172 B versus the
supplied OFF 55,740,240 B (+3.15%) and supplied ON 57,509,196 B.
The 12,024 B reduction is negligible; no substantial heap win is claimed.

### Gates, deviations and remaining work

- Final `cargo test --locked -p gpui --lib`: 469 passed, two existing timing
  tests ignored, in all six cache/splice modes. Both 12-seed differential
  sweeps pass. Adaptive experiment had 470 passed before being reverted.
- Targeted coverage includes ordinary ancestor notification preserving an
  explicit cached child, layout fallback preserving an unaffected implicit
  sibling, and existing opacity/visibility/property and retained-range tests.
- Both production-font 16-frame text differential smokes pass.
- Final `./script/clippy --locked -p gpui`, `cargo fmt --all -- --check`, and
  `cargo check --locked -p gpui -p ui -p workspace -p editor` pass.
- Measurement deviations: initial measurement window 06:07:23–06:17:34
  (10m10s) included a text-only corrected pass after timer hoisting. The
  architect then requested adaptive and placement experiments; these added
  instruction screens and the final 06:53:15–06:57:29 focused measurement
  window (4m14s), rather than pretending there was only one measurement pass.
  Coarse phase smokes run concurrently with a build were not used as
  before/after latency evidence. No intermediate `--quick` screen was used.
- First mount was left recording; no first-mount-policy toggle was measured.
  Rejected experiments were not promoted to another full ABAB wall matrix.
- No full Zed build, GPU pixel oracle, native Metal run or external profiler.
  Optional cargo-shear/typos/buf checks were not run.
- Near-zero refresh/first-frame overhead remains open. The captured placement
  contract is the documented trade-off; scope interning was not justified by
  the field attribution.

Source-only upstream follow-up at `9ab0715969`: explicit cached reuse in
`crates/gpui/src/view.rs:484–490` checks bounds, mask, text style, dirtiness and
refresh, but not inherited opacity. `elements/div.rs:2544` applies parent
opacity during paint, after that prepaint decision. A minimal repro is an
ordinary notifying parent that renders
`div().opacity(self.opacity).child(clean_child.cached(fixed_size))`, then
changes only its opacity: the child can replay primitives carrying the old
opacity. Hiding/unhiding an ancestor also warrants a separate stale-range
repro (`div.rs:2535` skips painting hidden children). This was inspected,
not built or fixed on main; hidden/unhidden behavior is not claimed proved
by the opacity source check.
