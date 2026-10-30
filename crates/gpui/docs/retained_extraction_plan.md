# GPUI retained rendering: extraction plan

This plan splits the experimental `gpui-retained-v6` branch (`2c049f0ba3`)
into pull requests against `main` that can be reviewed and landed one at a
time. Each stage is useful on its own and keeps today's behavior, with
anything risky behind a flag until it's proven.

The v6 branch has 73 commits in experiment order, including measurement
documents, reverts and approaches that were later replaced. These PRs are
re-cut from `main`, not cherry-picked. The commits listed for each item are
where the code comes from.

## Where things stand

- **Microbenchmarks.** With retention on, warm text with 96 panels takes about
  1.26 ms per frame, against about 25.8 ms on main and 1.54 ms for gpui-fast
  retained. With retention off, we still run 7–30% more instructions than
  gpui-fast fresh on trees, and about 16% more on text.
- **Real Zed.** These figures are total `Window::draw` CPU, 3 runs per mode.
  - **Retention off** (layout cache on, splicing off): 9–17% less draw CPU
    than main on typing, scrolling, cursor movement, tab switching and panels,
    with output identical to main at all 60 checkpoints. It is 7–12% less than
    gpui-fast on typing, scrolling and cursor movement, and 3% more on tab
    switching.
  - **Retention on:** 1–3% slower than off on ordinary workloads. It also has
    two bugs: a stale terminal region after dock toggles, and about 3.5× as
    many idle redraws. See `crates/gpui_platform/benches/retained_v7_investigation.md`
    on `gpui-retained-v7`.
  - gpui-fast's retained and fresh modes were within ±1.5% of each other in
    Zed.

Most of the real-Zed win therefore comes from the fresh-path and layout-cache
work (stages 0–2), not from retention itself. Retention pays off in Zed only
once the editor's own frames stop being rebuilt, which is a later problem.

## Stage 0: standalone fixes

No flags and no new concepts. Each PR is small.

| PR | Source | Status |
|---|---|---|
| Repaint cached views when inherited opacity changes | `gpui-fix-cached-view-opacity` | draft #65951 |
| Keep subscriptions for reused deferred draws | `gpui-fix-deferred-reuse-subscriptions` | draft #65952 |
| Avoid rebuilding cached font resolution errors | `70b9afd455` / `gpui-font-resolution-fix` | draft #65953 |
| Reduce per-line decoration run allocation | `bf89c19c4a` / `gpui-decoration-run-capacity` | draft #65954 |
| `Div` cold interactivity sidecar, plus the derived default text run | `02114b69e0`, `f9a1c7126f` / `gpui-div-cold-interactivity`, `gpui-derived-default-text-run` | to combine and review |
| Reuse fitting unwrapped text measurements | `f5602320cd` | to cut |
| Skip background painting for undecorated text | `3b3b555957` | to cut |
| Cache the resolved window text style between stack mutations | `e222a9e06f` | to cut |
| Avoid generic style conversion for default text nodes | `2d90309841` | to cut; check it still applies after Taffy 0.15 |
| **New:** replay last frame's painter orders in `BoundsTree` | gpui-fast `fast/bounds_tree.rs` | to implement, see below |
| **New:** per-window glyph raster-bounds and atlas-tile cache | gpui-fast `fast/glyphs.rs` | to implement, see below |
| Exact stable scene sort, with counting sort for sprites and ordinary primitives | `b96a980f18`, `d16422cbdc`, `e448103a11` | to cut |

The Taffy upgrade is covered by #65950 (0.15). The rest of the stack rebases
onto that instead of using our 0.14 commit (`4223238c9e`).

## Stage 1: measurement and verification infrastructure

There's no runtime behavior change in this stage, but every later PR needs it
to show its numbers and prove it's correct.

- **Benchmarks** in `gpui_platform`, buildable without `test-support`:
  - text panels (`2a0f38283c`, `652891a2f5`);
  - randomized trees;
  - scene ordering (`7f4d418b2d`);
  - first-frame heap.

  Linux instruction counting is already on `origin` as
  `gpui-linux-instruction-benchmarks`.
- **Reuse counters and the differential oracle** (`67acc82e43`,
  `d0748bab3a`). The oracle compares a retained window's scenes against a
  window drawn from scratch, frame by frame, over random histories. It should
  also compare hitboxes, as gpui-fast's oracle does.
- **The real-Zed autopilot harness.** This doesn't go upstream; it's a local
  tool. The patches are in `target/zed-real-compare/patches/` of the
  comparison worktree. Each stage should be checked with it, because
  microbenchmark wins didn't predict real-Zed results.
- **GPU statistics in the benchmark report** (not started; follow-up to
  #66316). Since #66316 removed `GPUI_BENCH_NO_GPU`, every benchmark frame goes
  through the headless GPU renderer, but only CPU instructions are measured.
  Report these next to instructions, in the bench output and in
  `script/gpui-bench-compare`:
  - **Plumbing:** a bench-support-only method on `PlatformHeadlessRenderer`
    that returns per-frame statistics. GPU results arrive asynchronously, from
    command-buffer completion handlers or query readbacks, so the bench
    context drains them after the measured loop, waits for the GPU outside
    timing, and reports medians.
  - **GPU time:** on Metal, the command buffer's `gpuStartTime` and
    `gpuEndTime`, with per-pass times from `MTLCounterSampleBuffer` at stage
    boundaries on Apple Silicon. On wgpu, `TIMESTAMP_QUERY` (and
    `TIMESTAMP_QUERY_INSIDE_PASSES` per pass) where the adapter supports it.
  - **Memory:** on Metal, `MTLDevice.currentAllocatedSize` plus
    `allocatedSize` per resource (atlas textures, instance buffers, path
    textures). On wgpu, `Device::generate_allocator_report()` where available,
    plus our own accounting of the buffers and textures we create. Track peak
    and growth, so buffer reallocations and atlas growth show up.
  - **Bandwidth:** no portable API reports DRAM bandwidth. Measure the bytes
    uploaded per frame exactly (instance and uniform writes, atlas uploads),
    and estimate render-target traffic from overdraw: fragment invocations
    (Metal's statistic counters, wgpu's `PIPELINE_STATISTICS_QUERY`) divided
    by pixels. Hardware counters stay an opt-in profiling mode: `xctrace` with
    Metal System Trace or GPU Counters on macOS, vendor tools on Linux.
  - **Other cheap stats:** draw calls, pipeline switches and instances per
    primitive kind; atlas uploads and occupancy; instance buffer size and
    reallocations; vertex and fragment invocations.
  - **Coordination:** this touches `metal_renderer.rs` and `wgpu_renderer.rs`,
    which John's renderer stack also changes. Check his rebased stack first,
    and keep the hooks in bench-support-only code.

## Stage 2: identity and layout (fresh path, no retention)

This is where the real-Zed win comes from.

1. **Interned element IDs** (`b3690df975`, `gpui-interned-element-ids`).
   - **What it does:** a per-window trie maps `GlobalElementId` paths to
     generation-checked handles, so element state and layout records are keyed
     by a handle instead of a cloned and hashed path.
   - **Status:** adopted in v6.
   - **Follow-ups:** see "Element identity" below.
2. **The persistent layout cache** (`7325ff4ba6`, `783d5b59fd`, `d1ed32c243`,
   `3d7d60e7bd`, `297391860b`, `0576047003`, `f1bd2f1092`, `3abfd65f88`).
   - **What it does:** keeps Taffy nodes across frames and writes a kept node
     only when its request changed.
   - **How to land it:** ship it behind `GPUI_LAYOUT_CACHE` first, then make it
     the default. Every "ours, off" real-Zed number had it on.
3. **The exact layout-input key** (`dd79424fb0`, `ed2af24e32`).
   - **What it does:** skips `Style::to_taffy` when a node's layout inputs are
     unchanged.
   - **Storage:** the keys live in a separate array that retained walks never
     touch.
4. **Text measurement replay**, new from gpui-fast (`fast/text.rs`).
   - **What it does:** when a label's text changes, it re-measures under the
     constraints Taffy logged last time. If every size matches, the node and
     its ancestors stay clean.
   - **Note:** we have carry but not replay. It helps live-updating labels.

## Stage 3: cheaper, correct `cached()` views

Zed already uses `cached()`, so these change existing behavior only by making
it faster or more correct.

- **Dependency tracking per view:**
  - entity reads, plus tracked global readers (`2b3140be5e`, split out);
  - custom-measure owners (`633f2d14d3`).

  Add from gpui-fast (`fast/dependencies.rs`):
  - version stamps on `ScrollHandle` and `ListState`, so reading a scroll
    position is a dependency;
  - ambient window input (pointer position, modifiers, caps lock) treated as
    dependencies;
  - the narrower change rules:
    - *notified without being updated* (scroll, animation) redraws only that
      view, not the views that read it;
    - *updated without being notified* counts only inside a notified view.

  These close several of the untracked reads the Zed audit found, and may
  explain the idle-redraw bug.
- **A flat per-frame entity-read array with scope ranges** (follow-up to
  #65952; pairs with an entity → reading-views reverse index):
  - **Today:** #65952 keeps a stack of per-scope `FxHashSet` read sets and
    merges each one into its parent on pop, so the cost grows with reads times
    nesting depth.
  - **The idea:** one `accessed: Vec<EntityId>` on the `Frame`, appended in
    tree order. Each cached view or deferred draw keeps a
    `reads_range: Range<usize>`, like its prepaint and paint ranges. Scopes
    nest in tree order, so a scope's reads, children included, are one
    contiguous slice. Popping a scope only records its end, and reuse is an
    `extend_from_slice` from the last frame's array, like `reuse_prepaint` and
    `reuse_paint`.
  - **Duplicates:** a per-frame "last recorded" stamp per entity skips repeat
    reads within one scope. Duplicates across sibling scopes are removed when
    subscriptions are built. An `IndexSet` doesn't work, because deduplicating
    globally breaks the "a subtree is one slice" property.
  - **Size:** roughly 100–150 lines in `EntityMap`, `Frame`, `view.rs` and
    deferred draws. It's also the natural storage for retained mode's per-view
    own-read sets and incremental subscriptions.
- **Segment replay with bulk copies** (`7d58a867df`, `d895b3c7f1`,
  `7da4f7036b`, `ce9c0a8840`, `008ecc8855`): a reused view's output is copied
  by array ranges.
- **Splicing independently dirty cached views** (`c3d92afeee`, `bf96fd64e5`,
  `80b5fd3db0`, behind `GPUI_VIEW_SPLICE`).

## Stage 4: implicit retention (behind a flag)

- **Every entity view becomes a boundary,** with exact layout guards
  (`bee97fae10`, `8a27fc9e56`, `7ef396b896`).
- **Window-root retention and the root layout skip:**
  - `2b3140be5e`, the root part;
  - `7476fe1d83`, `81a54a4c97`, `272c4cda62`, `075c6c4bc0`, `e1bd3a4239`.
  - **TODO: measure what skipping the root layout pass saves.** The skip is
    the only reason for `RetainedLayoutTree` in `taffy.rs`. That wrapper
    records each boundary's exact `LayoutInput`/`LayoutOutput`, because
    `TaffyTree`'s high-level API hides them. gpui-fast always runs Taffy and
    compares outputs instead (change and re-measure counters, plus the final
    layouts of the view's own nodes). Compare v6 against a variant that always
    runs Taffy with the persistent cache and checks outputs, retention on,
    across the tree, text and real-Zed workloads. If the gap is small, drop
    the skip and the wrapper.
- **Overhead reductions:** `91a0a77360`, `d9c63d132a`, `5b0dba3ca6`.
- **Painter-order index for retained frames.** Use stage 0's positional
  replay first. Then decide between the grid (`2a05aa1f28`, `cba575e1cf`) and
  the R-tree by measurement. The grid lost 26% on the 1024-stripe benchmark,
  and its query trims were rejected.
- **The "prebuilt" path,** new from gpui-fast. When a splice fails after a
  nested view was already rebuilt, the parent takes over that rebuilt view
  instead of building it twice. This is a candidate for our mixed-leaf gap,
  where we're +71% instructions against gpui-fast retained and bimodal.
- **Before the flag goes on in Zed:**
  - fix the two v7 bugs;
  - audit the remaining untracked reads in Zed.

Dropped from v6, not to be extracted:
- the balanced `BoundsTree` (`af66c9e73d`, superseded);
- the WIP measure attribution (`47fa268cae`);
- the adaptive recording experiment and its revert (`77b7c6cc66`,
  `5980d82caa`);
- all measurement-only commits.

## Stage 5: composition and partial rendering (last)

#62379 splits one rebuilt frame into GPUI and native surfaces. The goal here
is composition built on retention instead:

- **Each surface is a retained boundary,** or a group of them, with its own
  scene range and dirtiness.
- **Present only what changed.** A frame where only a native view changed
  re-presents that view and keeps the GPUI surfaces. A frame where one GPUI
  boundary changed re-encodes only its surface.
- **Embed other GPUI roots.** A child GPUI app is a root with its own frame,
  presented as a surface or spliced in as a retained segment. Earlier tries are
  on `origin`: `gpui-multi-root-embedded`, `gpui-embedded-in-gpui`.
- **Parallelism after paint.** Entity updates and painting stay on the main
  thread, because `App` is single-threaded. A finished surface scene is plain
  data, so its sort, encoding and submission can run on render threads, one
  per surface. gpui-fast's scroll layers already rasterize retained content
  into tiles and composite them by offset (`fast/layers/`), and are the
  closest precedent.

## New findings in this round

### Painter-order replay works on fresh frames too

gpui-fast's ordering structure is also a 64 px grid. On fresh dense trees its
inserts cost 0.5% of instructions inclusive, against 13.4% for ours. The
difference isn't the grid.

On `clear()`, it keeps the previous frame's `(bounds, order)` sequence. On
each insert:

- **If the bounds match the previous frame's at the same position,** and
  they meet none of the bounds that changed this frame, the old order is
  returned without touching the grid.
- **The changed bounds** sit in a 64×64 bitmask of coarse cells, so ruling
  one out costs a few `u64` operations.
- **Otherwise** it searches what was inserted so far.
- **After 2^15 comparisons,** it builds the grid from the inserted sequence
  and carries on normally.

This is exact, because an order depends only on the bounds inserted before
it. It doesn't depend on retention, so it applies to every Zed frame whose
geometry didn't move, which is most of them. It also fits on top of the
upstream R-tree, so it can land in stage 0.

Our replay works only inside retained segments. That is why our fresh path
pays for a full query on every primitive.

### Element identity

Measured self and inclusive shares on fresh trees (perf, v6):

| Function | Dense | Tall | Mixed |
|---|---:|---:|---:|
| `ElementIdInterner::intern` (inclusive) | 3.9% | 3.2% | 3.9% |
| `SmolStr` hashing (`ElementId::Name`) | 1.7% | 2.2% | 2.5% |
| `ElementId::eq` | 1.9% | — | — |
| `Window::take_element_state` (inclusive) | 3.0% | 4.3% | 4.5% |
| Element-state map `remove_entry` | 1.5% | 2.3% | — |

gpui-fast doesn't solve this. Its `GlobalIdCache::get`, which keeps a
cached path hash and compares full paths, costs about the same: 2.9–4.5%.

Two changes go beyond it:

1. **Sibling-cursor interning.**
   - **The idea:** each interned node remembers its children in the order the
     last frame visited them. A push first compares the id with the next
     expected child, a single `ElementId` comparison with no hashing, and
     falls back to the hash map on a miss.
   - **Why it works:** frames repeat their element order, so most pushes hit.
     It also survives a local insertion better than a positional index.
   - **A cheap addition:** a pointer fast path in `SharedString` equality.
2. **Dense element state.**
   - **The idea:** replace `FxHashMap<(InternedHandle, TypeId), ElementStateBox>`
     with a slot vector indexed by the handle, checked by generation, holding a
     small inline list by `TypeId` (almost always one entry).
   - **What it removes:** the hash remove and insert pair on every
     `with_element_state`.

Together these target roughly 5–9% of fresh tree instructions. That estimate
is based on the profile shares above and hasn't been measured.

### Other gpui-fast techniques not yet taken

| Technique | Module | Relevance |
|---|---|---|
| Glyph raster-bounds and atlas-tile cache (4,096 slots; tiles valid for one frame) | `fast/glyphs.rs` | Text paint is 17% of our instructions against their 13%, and raster bounds 2.3% against 1.1%. High for Zed. Leave out their `quantize_origin` rounding change, because it moved glyphs by about 1 px compared with main. |
| Path tessellation cache (relative to the first point) | `fast/path_cache.rs` | Editor selections and highlights are paths. Worth measuring in Zed. |
| Text measurement replay | `fast/text.rs` | Stage 2 item 4. |
| Dependency rules and version stamps | `fast/dependencies.rs` | Stage 3. |
| Prebuilt splice fallback | `fast/retained.rs`, `fast/splice.rs` | Stage 4. |
| Scroll layers (tile compositing) | `fast/layers/` | Stage 5 precedent. It's large, and lists are off even in gpui-fast. |
| Upstream-hook discipline (`script/check-upstream`) | — | A useful review shape: the logic lives in its own modules, and upstream files get one-line hooks. |

Already matched or rejected:
- **Number shaping:** benchmark-specific.
- **Layout fingerprint:** we use an exact key instead.
- **Grid without replay:** rejected.
- **Dense layout bounds:** we have them.

## Suggested first wave

1. Land stage 0: four drafts are open, and four or five more are small.
2. Implement positional painter-order replay on the upstream `BoundsTree`.
   Measure it on the benchmarks and in real Zed.
3. Implement the glyph bounds cache. Measure it the same way.
4. Stage 1 infrastructure PRs.
5. Prototype sibling-cursor interning and dense element state on the v6
   stack, then land interning (stage 2.1) with them if they hold up.
6. The layout cache behind its flag.
