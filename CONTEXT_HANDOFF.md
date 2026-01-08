# Context Handoff: Git Diff View Performance Investigation

## Problem

Stuttering when opening large git repos (chromium) with thousands of changed files in the git diff view.

## Key Discovery: The Real Culprit

**The stuttering is NOT caused by excessive git blame operations.**

Initial investigation was misled by async tracing artifacts - `#[ztracing::instrument]` on async functions records a span entry on every poll, not just once per call. A single `for_path` call taking 50+ seconds gets polled hundreds of times, appearing as thousands of "calls" in traces.

**Actual findings with corrected tracing:**
- Only 7-8 git blame spawns per session (not thousands)
- The real bottleneck: `BlockMap::sync` in `crates/editor/src/display_map/block_map.rs`
- Each sync operation takes **25-100 seconds** on large MultiBuffers

## The Causal Chain

1. Git blame completes (after ~26 seconds)
2. Blame results trigger `cx.notify()` on GitBlame entity
3. Editor observes GitBlame and calls `cx.notify()` on itself
4. Editor re-renders, triggering `DisplayMap::update()`
5. This cascades through: `inlay_map.sync → fold_map → tab_map.sync → wrap_map.sync → block_map.sync`
6. `BlockMap::sync` processes the entire MultiBuffer (100,000+ rows from thousands of files)
7. **Sync takes 25-100 seconds → stutter**

**Evidence:** Disabling all `generate()` calls in `GitBlame` eliminates the stutter entirely.

## Fix Attempts

### 1. Debouncing generate() calls
Added 100ms debounce to event-triggered `generate()` calls.
**Result:** Minimal effect - actual spawn count was already low.

### 2. kill_on_drop on git subprocess
Added `.kill_on_drop(true)` to the smol command spawning git blame.
**Result:** Minimal effect - same reason.

### 3. Incremental per-chunk updates (latest)
Instead of batching all blame results and updating once at the end, update `GitBlame.buffers` and call `cx.notify()` after each chunk of 4 buffers completes.

**Result:** Made things WORSE.
- run_9 (batch): 7 spawns, max sync ~51s
- run_10 (incremental): 45 spawns, max sync ~100s

The incremental `cx.notify()` calls trigger more frequent re-renders, each causing expensive `BlockMap::sync` operations on the massive buffer.

## Current Code State

Changes in `crates/editor/src/git/blame.rs`:
- `generate()` now updates `this.buffers` and calls `cx.notify()` per chunk (4 buffers) instead of at end
- Removed `this.buffers.clear()` - updates in-place
- Debouncing code exists but is NOT currently active (reverted to direct `generate()` calls)
- `BLAME_GENERATION_ID` atomic counter for tracing

Changes in `crates/git/src/blame.rs`:
- Tracing moved from async `for_path()` to sync `spawn_git_blame()` to get accurate counts
- `generation_id` parameter threaded through for tracing
- `kill_on_drop` is currently OFF (was tested but reverted)

## Key Files

| File | Purpose |
|------|---------|
| `crates/editor/src/display_map/block_map.rs` | **THE BOTTLENECK** - `sync` function, `while edits` loop (~line 572) |
| `crates/editor/src/git/blame.rs` | GitBlame implementation, `generate()` function |
| `crates/git/src/blame.rs` | `spawn_git_blame()`, `for_path()` - git blame execution |
| `crates/editor/src/display_map.rs` | DisplayMap orchestrating sync chain |
| `PERF_NOTES.md` | Detailed investigation notes |
| `PERF_SUMMARY.md` | Clean summary of findings |

## Useful Commands

```bash
# Build with tracy
RUSTFLAGS="-C force-frame-pointers=yes" ZTRACING=1 cargo build --features tracy --release

# Find long-running spans (>10ms)
tracy-csvexport -u trace.tracy | awk -F, '$5 > 10000000 {print $1","$5/1000000"ms"}' | sort -t, -k2 -rn | head -30

# Count actual git blame spawns
tracy-csvexport -u -f "spawn_git_blame" trace.tracy | grep "spawn_git_blame{" | wc -l

# Count BlockMap syncs taking >1ms
tracy-csvexport -u trace.tracy | grep "block_map" | awk -F, '$5 > 1000000 {count++} END {print count}'
```

## Test Repo

Chromium repo at `/Users/kubkon/dev/chromium`

## Next Steps

Two potential approaches:

### Approach 1: Optimize BlockMap::sync
1. Profile the `while edits` loop in `block_map.rs:572`
2. Understand why operations on large row counts (100k+) are slow
3. Consider algorithmic optimizations for large MultiBuffers

### Approach 2: Avoid triggering full sync
1. Investigate if blame updates can avoid triggering full DisplayMap sync
2. Consider a different notification mechanism that doesn't cause re-render
3. Batch/coalesce notifications to reduce sync frequency

### Approach 3: Architectural change
1. Consider lazy blame - only blame visible buffers, not all 3000+ in MultiBuffer
2. Load blame data on-demand as user scrolls
3. Avoid putting all changed files into one massive MultiBuffer

## Key Insight

The incremental update approach backfired because each `cx.notify()` triggers a full `BlockMap::sync` on the entire 100k+ row MultiBuffer. The problem isn't how often we update GitBlame's internal state - it's that ANY notification causes an expensive full re-render/sync.

The fix likely needs to happen at the DisplayMap/BlockMap level, not the GitBlame level.