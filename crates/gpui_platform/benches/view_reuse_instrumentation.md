# View reuse instrumentation: stage 1

The existing randomized and text workloads reconstruct view output rather than
exercise `cached()` reuse. A separate `CachedTree` family now exposes the reuse
path. Its most important result is the dense-entity leaf-style workload:
**one own-dirty Render causes another 77.93 ancestor-caused Render calls per
frame**, with 76.93 cached misses attributed to synthetic refreshing.

This is instrumentation, not a rendering optimization. Production builds exclude
the window counters, notification-origin tracking, snapshots, and reconstruction
switch. The architect accepted the measured benchmark-only overhead as tooling;
the stage 2 baseline and candidate must both include stage 1.

## Provenance and reproduction

- Worktree: `/home/anth/.local/share/delta-trunk/worktrees/p8dz6y9kakrh/zed`.
- Branch: `gpui-reuse-instrumentation`.
- Base: `f9a1c7126fa86816de1f32c08e83915f42ef6a45`.
- Initial instrumentation: `01c7be7935c8795c16af70ae1f267171ef385b29`.
- Compiling benchmark and primary measured candidate:
  `952d04601a6d0c2478e879bd1867028488f32f47`.
- Attribution correction: `4827f93b17010e24bb0b701ad37c3d058361b7fb`.
  This distinguishes explicit refresh from a cached ancestor's synthetic
  refreshing and recognizes cached first mounts by occurrence-state absence.
- Linux x86_64, Rust 1.98.1 (`48a229cea`, LLVM 22.1.8).
- Profile: `release-fast`; features: `bench-support,x11`, no default features.
  The complete normal/build/dev feature tree contained no `test-support`.
- Noto Sans Mono, Fontconfig face 0,
  `/usr/share/fonts/noto/NotoSansMono-Regular.ttf`,
  SHA-256 `65b5e2b2c4a1fba9ae8be1f026cb35b03dcb8886d9b2a4147054fde12f7e767d`.
- Lockfile SHA-256:
  `a25b0e36e2cafe3d9b38ec79819f7ebc168fbd6f285b6f98dc05bad9e4e3dc41`.
- Both binaries were built in this worktree from their respective sources,
  using the private reflink-seeded `$PWD/target/private`. Tracked sources were
  touched after seeding and on the source switch. No shared-target executable
  was used.
- Logs and private executable copies remain in the ignored local
  `target/reuse-instrumentation/` directory. These artifacts are not replicated
  as source changes.

Build command, wrapped in the experiment lock and a 900-second timeout:

```sh
CARGO_TARGET_DIR="$PWD/target/private" cargo bench --locked -p gpui_platform \
  --bench randomized_element_tree --no-default-features \
  --features bench-support,x11 --profile release-fast --no-run
```

Counter quick runs use `GPUI_LAYOUT_CACHE=1`, `SEED=3`, `ITERATIONS=1`,
`BENCH_MEASUREMENT=foreground-instructions`, `--quick`, and one invocation per
`^CachedTree/<group>/` filter. Both retention variables and both verification
variables are unset. These runs used the updated shared-lock protocol and CPU 18,
with a 300-second timeout per invocation. The added group name deliberately
does not contain `RandomizedTree`, so existing substring filters do not silently
select additional workloads.

## Counter interpretation

`Window::view_reuse_stats()` reads accumulated work;
`take_view_reuse_stats()` returns and resets it without changing invalidation
state. The harness resets after setup and prints totals after measurement.
Divide by the recorded frame count, not by attempted mutations. Quick counter
tables below include all observed quick/calibration frames, not exclusively
Criterion's measurement phase.

Render causes are mutually exclusive: first mount, own notification, synthetic
ancestor refresh, explicit refresh/inspector, propagated or ancestor dirtiness,
and finally cache-context reconstruction. Ancestor-caused includes both an
ancestor dirtied by a descendant and a clean descendant rebuilt because its
ancestor was dirtied. Cached miss reasons overlap; their sum can exceed misses.
The normal cached predicate and normal rendering operations are unchanged.

Replay counts describe **inline records processed**, not deep owned heap bytes,
physical memory traffic, or memcpy calls. Moves include processed `Option`
slots. Dispatch and deferred-draw records are rebuilt even though some payloads
inside those records are moved. Scene primitive counts count logical replayed
primitives once; they do not count every subsequent internal primitive copy.
Element-state keys can appear in both phase replay ranges. These are operation
counts, not unique retained objects.

### Render calls and cache outcomes per frame

Four mutation-group tables were captured on `952d04601a`; explicit full refresh
was recaptured on `4827f93b17` after correcting its attribution. Numbers are
frame-weighted within each family. Setup first mounts are excluded.

| Group | Family | Own Render | Ancestor Render | First mount Render | Refresh Render | Cache hits | Cache misses |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| full refresh | any | 0 | 0 | 0 | 46 | 0 | 45 |
| full refresh | wide | 0 | 0 | 0 | 23 | 0 | 22 |
| full refresh | tall | 0 | 0 | 0 | 15 | 0 | 14 |
| full refresh | mixed | 0 | 0 | 0 | 94 | 0 | 93 |
| full refresh | dense-entities | 0 | 0 | 0 | 205 | 0 | 204 |
| leaf style | any | 1 | 0.00 | 0 | 0 | 45.00 | 0.00 |
| leaf style | wide | 1 | 0.61 | 0 | 0 | 21.39 | 0.61 |
| leaf style | tall | 1 | 0.87 | 0 | 0 | 7.55 | 0.87 |
| leaf style | mixed | 1 | 0.57 | 0 | 0 | 50.71 | 0.57 |
| leaf style | dense-entities | 1 | 77.93 | 0 | 0 | 8.00 | 77.93 |
| leaf bounds | any | 1 | 0.33 | 0 | 0 | 44.67 | 0.33 |
| leaf bounds | wide | 1 | 0.13 | 0 | 0 | 21.87 | 0.13 |
| leaf bounds | tall | 1 | 1.03 | 0 | 0 | 7.52 | 1.03 |
| leaf bounds | mixed | 1 | 2.29 | 0 | 0 | 50.43 | 2.29 |
| leaf bounds | dense-entities | 1 | 82.63 | 0 | 0 | 7.95 | 82.63 |
| insert-remove | any | 1 | 2.00 | 0.67 | 0 | 43.33 | 2.67 |
| insert-remove | wide | 1 | 6.16 | 0.16 | 0 | 16.68 | 6.32 |
| insert-remove | tall | 1 | 1.19 | 0.00 | 0 | 7.13 | 1.19 |
| insert-remove | mixed | 1 | 0.57 | 0.14 | 0 | 50.86 | 0.71 |
| insert-remove | dense-entities | 1 | 84.45 | 0.35 | 0 | 8.00 | 84.81 |
| reorder | any | 1 | 19.67 | 0 | 0 | 25.33 | 19.67 |
| reorder | wide | 1 | 4.16 | 0 | 0 | 17.84 | 4.16 |
| reorder | tall | 1 | 4.29 | 0 | 0 | 5.52 | 4.29 |
| reorder | mixed | 1 | 2.57 | 0 | 0 | 49.71 | 2.57 |
| reorder | dense-entities | 1 | 97.47 | 0 | 0 | 7.00 | 97.47 |
| installed Noto text warm, 96 panels | original uncached | 2 | 95 | 0 | 0 | 0 | 0 |

Cache-context Render was zero in every row. The original text workload performs
97 Render calls despite updating only two panels, and has no cache replay.
Its existing Render bodies, labels, mutations, and workload sizes are unchanged.

### Cached misses by reason per frame

Text-style and inspector misses are zero throughout these workloads. Forced
reconstruction is disabled for these counter runs.

| Group | Family | First mount | Bounds | Content mask | Dirty | Refreshing |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| full refresh | any | 0 | 0 | 0 | 0 | 45 |
| full refresh | wide | 0 | 0 | 0 | 0 | 22 |
| full refresh | tall | 0 | 0 | 0 | 0 | 14 |
| full refresh | mixed | 0 | 0 | 0 | 0 | 93 |
| full refresh | dense-entities | 0 | 0 | 0 | 0 | 204 |
| leaf style | any | 0 | 0 | 0 | 0.00 | 0.00 |
| leaf style | wide | 0 | 0 | 0 | 0.61 | 0.00 |
| leaf style | tall | 0 | 0 | 0 | 0.52 | 0.42 |
| leaf style | mixed | 0 | 0 | 0 | 0.29 | 0.29 |
| leaf style | dense-entities | 0 | 0 | 0 | 3.67 | 76.93 |
| leaf bounds | any | 0 | 0.33 | 0.00 | 0.33 | 0.00 |
| leaf bounds | wide | 0 | 0.13 | 0.00 | 0.13 | 0.00 |
| leaf bounds | tall | 0 | 0.23 | 0.48 | 0.29 | 0.55 |
| leaf bounds | mixed | 0 | 0.29 | 0.00 | 1.00 | 1.71 |
| leaf bounds | dense-entities | 0 | 5.35 | 1.16 | 4.16 | 81.59 |
| insert-remove | any | 0.67 | 1.33 | 0.00 | 0.67 | 0.33 |
| insert-remove | wide | 0.16 | 6.10 | 0.19 | 0.06 | 0.06 |
| insert-remove | tall | 0.00 | 0.81 | 0.58 | 0.39 | 0.55 |
| insert-remove | mixed | 0.14 | 0.00 | 0.00 | 0.57 | 0.29 |
| insert-remove | dense-entities | 0.35 | 1.42 | 0.48 | 3.23 | 83.45 |
| reorder | any | 0 | 19.67 | 0.00 | 0.00 | 0.00 |
| reorder | wide | 0 | 4.16 | 0.00 | 0.00 | 0.00 |
| reorder | tall | 0 | 4.29 | 2.94 | 0.00 | 1.81 |
| reorder | mixed | 0 | 2.00 | 0.29 | 0.29 | 1.29 |
| reorder | dense-entities | 0 | 39.73 | 21.27 | 2.87 | 95.47 |

### Replay per array per frame

These are arithmetic means of each family's frame-normalized counters, giving
the five families equal weight. Each cell is **elements / inline bytes**.

| Group | Array | Cloned | Moved | Rebuilt |
| --- | --- | ---: | ---: | ---: |
| leaf style | hitboxes | 29.49 / 1415.66 | 0 | 0 |
| leaf style | element-state keys | 409.54 / 16381.57 | 0 | 0 |
| leaf style | scene quads | 107.44 / 17190.33 | 0 | 0 |
| leaf style | dispatch nodes | 0 | 0 | 159.36 / 20397.45 |
| leaf style | mouse-listener slots | 0 | 116.00 / 1855.97 | 0 |
| leaf bounds | hitboxes | 28.40 / 1363.14 | 0 | 0 |
| leaf bounds | element-state keys | 397.24 / 15889.54 | 0 | 0 |
| leaf bounds | scene quads | 101.63 / 16261.29 | 0 | 0 |
| leaf bounds | dispatch nodes | 0 | 0 | 154.17 / 19734.23 |
| leaf bounds | mouse-listener slots | 0 | 112.98 / 1807.70 | 0 |
| insert-remove | hitboxes | 28.39 / 1362.58 | 0 | 0 |
| insert-remove | element-state keys | 398.60 / 15943.89 | 0 | 0 |
| insert-remove | scene quads | 97.93 / 15668.65 | 0 | 0 |
| insert-remove | dispatch nodes | 0 | 0 | 154.27 / 19747.08 |
| insert-remove | mouse-listener slots | 0 | 112.97 / 1807.48 | 0 |
| reorder | hitboxes | 25.94 / 1245.03 | 0 | 0 |
| reorder | element-state keys | 361.89 / 14475.46 | 0 | 0 |
| reorder | scene quads | 87.79 / 14045.71 | 0 | 0 |
| reorder | dispatch nodes | 0 | 0 | 141.47 / 18108.70 |
| reorder | mouse-listener slots | 0 | 99.45 / 1591.16 | 0 |

All three categories are zero for full refresh and the original text workload.
All three categories are also zero for tooltips, deferred draws, cursor styles,
input handlers, tab stops, text-layout usage indices, shadows, paths, underlines,
monochrome sprites, subpixel sprites, polychrome sprites, and surfaces in these
cached box-tree runs. The counters for those arrays exist; these fixtures simply
do not produce those replay records.

### Phase visits and geometry

Actual element callbacks are counted at `Drawable`, including cached boundary
callbacks. Cache-hit request-layout and prepaint boundary visits are separated
from cache-hit paint visits: clipping can prevent paint after prepaint. Retained
descendants execute zero callbacks, not a synthetic replay visit per element.
Fresh callback counts are total callbacks minus cache-hit boundary callbacks.

| Full refresh family | request-layout | prepaint | paint |
| --- | ---: | ---: | ---: |
| any | 242 | 242 | 242 |
| wide | 163 | 163 | 163 |
| tall | 92 | 92 | 71 |
| mixed | 741 | 741 | 572 |
| dense-entities | 432 | 432 | 359 |
| original text warm, 96 panels | 12483 | 12483 | 12483 |

The geometry counters currently cover cached occurrences only: a size change
increments `bounds_size_changed`; unchanged size with changed origin increments
`bounds_origin_only_changed`. Noncached occurrence geometry is not instrumented.
The raw quick logs contain the phase, geometry, and fixup counts for each family.
`index_fixups` counts reconstructed dispatch-parent references, deferred-draw
parent relocation, primitive order assignment, and path-ID assignment; it is not
a complete count of all internal indexing in tab-stop reconstruction.

## Differential verification

The architect replaced the originally requested in-frame scratch pass with two
independent deterministic fixtures. Re-rendering in the live window would mutate
entities and consume/move callbacks, so it is not a safe scratch oracle.

- `Window::set_force_view_reconstruction(true)` forces cached views to miss.
- `GPUI_FORCE_VIEW_RECONSTRUCTION=1` sets this mode at window creation in
  test/bench-support builds.
- `GPUI_VERIFY_REUSE_DIFFERENTIAL=1` enables untimed independent-window scripted
  comparison in the benchmark harness. It is disabled during performance runs.
- `GPUI_VERIFY_VIEW_REUSE` is not implemented as an in-frame mode.
- Canonical snapshots compare ordered primitive/layer operations, hitbox
  geometry/masks/behavior, dispatch parent shape/logical view identity/key
  contexts/listener summaries, cursor styles with canonical hitbox indices, and
  tab stops with dispatch-node focus identity.
- A mismatch reports the first component/index, frame number, compared values,
  and logical dispatch identities. Primitive ownership is not independently
  mapped to one exact owning view.
- Native surface identity and nondeterministic Render implementations are outside
  the supported oracle contract. This is CPU-output verification, not pixels,
  callable-listener equivalence, or end-to-end presentation verification.

No mismatch was found:

- The GPUI test compares cached wide and mixed trees for 16 frames each, including
  style, bounds, insert, remove, reorder, and unchanged updates.
- Benchmark differential verification passed all five seed-3 tree families,
  including tall and dense-entity trees, with a 16-frame script.
- Installed-Noto text verification passed 96 cached panels with four frames and
  the same two-panel update script in both independent windows. The separate
  cached verification root leaves the existing timed text Render body unchanged.
- The entire GPUI test suite also passed with forced reconstruction enabled.

## Overhead against the base

The original exclusive-lock ABAB matrix was already running when the revised
tiered protocol arrived. It completed; it was not repeated. These captures are
**unpinned**, so their wall medians are not the later CPU-0 protocol's adoption
gate. Each benchmark invocation had a 300-second timeout; smoke was run for both
binaries before measurement. Text cases also ran quick mode. Criterion used the
existing 500 ms warmup / 1 s measurement / 20 samples configuration.

The matrix used the original uncached tree inputs, unchanged text workloads,
both requested seeds, and layout-cache modes from `ACCEPTANCE-MATRIX.md`.
There is no randomized first-populated-frame group in this base.

Below, tree instruction changes are geometric means across the five families.
Wall columns are geometric means of reported secondary per-iteration medians,
which include warmup/calibration; they are not measurement-phase frame p95.

| Case | Instruction change A1 | Instruction change A2 | Wall change A1 | Wall change A2 |
| --- | ---: | ---: | ---: | ---: |
| text warm, cache 1 | +0.181% | +0.294% | -1.187% | -1.712% |
| text warm, cache 0 | +0.068% | -0.018% | -2.740% | -2.146% |
| text first populated, cache 1 | +0.124% | +0.000% | -0.468% | -0.979% |
| seed 3 full refresh | +0.135% | +0.140% | +0.377% | +0.309% |
| seed 3 unchanged | +0.447% | +0.472% | +0.169% | +4.869% |
| seed 3 leaf style | +0.426% | +0.422% | +0.956% | +1.007% |
| seed 3 leaf bounds | +0.413% | +0.385% | +0.957% | +1.888% |
| seed 3 root layout | +0.427% | +0.448% | -0.914% | +2.796% |
| seed 3 insert-remove | +0.471% | +0.460% | -0.587% | -0.537% |
| seed 3 reorder | +0.428% | +0.442% | +2.172% | -0.413% |
| seed 11 full refresh | +0.161% | +0.144% | -1.357% | +1.937% |
| seed 11 unchanged | +0.681% | +0.651% | +8.490% | +0.124% |
| seed 11 leaf style | +0.531% | +0.484% | +3.062% | +0.941% |
| seed 11 leaf bounds | +0.463% | +0.462% | +3.665% | +3.392% |
| seed 11 root layout | +0.719% | +0.659% | +2.486% | +3.016% |
| seed 11 insert-remove | +0.566% | +0.526% | +0.345% | +0.647% |
| seed 11 reorder | +0.382% | +0.559% | +2.854% | +1.174% |
| seed 3 full refresh, cache 0 | +0.088% | +0.118% | +2.049% | -1.494% |

No individual family exceeded 2% instruction overhead. The maximum was
**+1.817%**, seed 11 root-layout/dense-entities, first pair. Thus group instruction
means satisfy approximately 1%, but the strict per-family 1% requirement is not
met. The architect explicitly accepted that benchmark-only tooling cost.

Repeated individual-family wall increases above 2% were observed in seed-11
full-refresh/any, leaf-bounds/wide/mixed/dense, reorder/any,
unchanged/mixed/dense, and seed-3 root-layout/mixed and unchanged/mixed.
These measurements do **not** establish below-1% wall overhead or a
rendering-performance improvement.

Representative raw text results from the first pair:

| Installed Noto, 96 panels | Base | Instrumented |
| --- | ---: | ---: |
| warm/cache1 foreground instructions | 132.58 M | 132.82 M |
| warm/cache1 secondary wall median | 9.264 ms | 9.154 ms |
| warm/cache1 foreground allocations/frame | 22,108 | 22,108 |
| first/cache1 foreground instructions | 242.65 M | 242.95 M |
| first/cache1 secondary wall median | 17.727 ms | 17.644 ms |
| first/cache1 foreground allocations/iteration | 61,170 | 61,170 |
| first population setup live-heap delta | 59,198,108 B | 59,189,916 B |

Heap deltas are one process-wide requested-allocation probe, not exclusive
ownership, peak heap, RSS, or GPU memory. Linux headless presentation does not
establish GPU execution or display latency.

## Validation and remaining scope

On the compiling candidate `952d04601a`:

- `cargo test --locked -p gpui --lib`, `GPUI_LAYOUT_CACHE=0`: 413 passed,
  2 ignored.
- Same command, `GPUI_LAYOUT_CACHE=1`: 413 passed, 2 ignored.
- Same command, cache 1 plus `GPUI_FORCE_VIEW_RECONSTRUCTION=1`: 413 passed,
  2 ignored.
- `./script/clippy --locked -p gpui`: passed.
- `cargo check --locked -p gpui`, without bench-support or test-support: passed.
- All 25 new cached seed-3 smoke inputs: passed.
- Full existing matrix smoke and measured runs: passed.
- Differential benchmark tree and installed-Noto text checks: passed.

`cargo fmt --all` was run. The exact requested `cargo fmt --all -- --check` was
not run. After the attribution-only `4827f93b17` correction, the optimized
benchmark rebuilt and all five full-refresh quick inputs passed; the entire
test/Clippy/production-check matrix was not rerun on that final source revision.
No additional runs were made after the architect requested the final report
and end of the experiment.

Remaining instrumentation scope is noncached-occurrence geometry, complete
tab-stop index-fixup accounting, exact owning-view attribution of primitive
mismatches, and deep owned-payload byte accounting if that is required rather
than inline-array record sizes. The current counters already provide the
ancestor-refresh and segment-replay evidence needed for stages 2 and 3.

## Stage 2b: implicit boundaries

`GPUI_VIEW_SPLICE=all` enables uncached entity occurrences as well as explicit
cached views. Implicit layout retention requires `GPUI_LAYOUT_CACHE=1`; without
persistent nodes it conservatively follows the cached-only path. `=1` remains
cached-only, and an unset switch leaves the original layout and replay paths.

Uncached Render/request-layout scopes are stored independently of prepaint and
paint scopes. The opt-in Taffy adapter uses the same block, float, flex, grid,
leaf and cache algorithms as TaffyTree, but records the exact `LayoutInput` and
`LayoutOutput` at container nodes. Dispatch identity is absent during layout.
Dirty children request layout in their recorded execution scope before the
containing root is computed. The child is computed under its stored inputs; its
full Taffy style, assigned size and output (including baselines and collapsing
margins) must match. A prepared element stays in the occurrence only for this
frame, and an ancestor fallback consumes it without another Render invocation.

After the containing root is computed, reuse rechecks actual inputs and
geometry. Geometry comparisons share the engine's layout records; only changed
nodes need a transient previous-layout entry, rather than allocating a subtree
snapshot per occurrence. Speculative absolute bounds cannot survive a containing
root computation: flex redistribution can invalidate a child's cached bounds on
the root's *first* computation in the frame.

Conservative implementation choices: the exact style comparison includes
container-facing fields as well as parent-facing fields. A changed non-view
layout on a retained splice path falls back rather than relocating individual
element callbacks. Direct root notifications still reconstruct the root and its
descendants in stage 2b, preserving inherited opacity and visibility changes;
root retention is the subsequent stage.

Correctness evidence in the implementation worktree:

- Six `cargo test --locked -p gpui --lib` combinations (layout cache 0/1 and
  splice off/1/all): 434 passed, 2 ignored in each. The all/cache-1 run exposed
  inherited root opacity/visibility invalidation; it passed after that fix.
- Uncached differential sweep: four topologies, twelve scheduler seeds, sixteen
  frames, including style, bounds, insert/remove and reorder.
- Targeted equal-size margin/style fallback, size fallback, and a mutating
  Render counter asserting exactly one child Render on fallback frames.
- `./script/clippy --locked -p gpui`, `cargo fmt --all -- --check`, and
  `cargo check --locked -p gpui -p ui -p workspace -p editor` passed.

Builds/tests used the private reflink-seeded `target/private`, shared experiment
lock, CPU 16, six Cargo jobs, and 900-second build / 300-second run timeouts.
No timing comparison was performed at this stage.
