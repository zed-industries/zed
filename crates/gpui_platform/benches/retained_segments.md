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
