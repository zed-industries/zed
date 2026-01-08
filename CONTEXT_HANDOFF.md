# Context Handoff: Git Diff View Performance Investigation

## Problem

Stuttering when opening large git repos (Chromium) with thousands of changed files in the git diff view.

## Key Discovery: The Real Culprit

**The stuttering is NOT caused by excessive git blame operations.**

The real bottleneck is `BlockMap::sync` in `crates/editor/src/display_map/block_map.rs`, which takes **25-145 seconds** on large MultiBuffers (100,000+ rows from thousands of files).

## Evidence

1. Disabling all `generate()` calls in GitBlame eliminates stutter completely
2. Only 7-8 actual git blame spawns per session (not thousands - earlier traces were misleading due to async polling artifacts)
3. Tracy traces show `BlockMap::sync` operations taking 25-51 seconds baseline, up to 145 seconds under certain conditions

## Failed Optimization Attempts

### 1. Debouncing generate() calls
- Added 100ms debounce to event-triggered generate() calls
- **Result:** Minimal effect - actual spawn count was already low

### 2. Incremental per-chunk updates
- Update GitBlame.buffers and call cx.notify() after each chunk completes
- **Result:** Made things WORSE - more frequent syncs increased total time

### 3. Chunk size tuning (run_11 & run_12)
Tested larger chunk sizes (buffers per iteration):

| Metric | chunk=4 (baseline) | chunk=16 | chunk=64 |
|--------|-------------------|----------|----------|
| spawn_git_blame calls | 7 | 68 | 195 |
| Max sync duration | 51.6s | 124.8s | 144.9s |

**Result:** BACKFIRED - larger chunks caused more total spawns and worse sync times.
Relationship is non-linear; larger chunks affect how many buffers queue for subsequent generate() calls.

## The Causal Chain

1. Git blame completes (~26 seconds)
2. Blame results trigger `cx.notify()` on GitBlame entity
3. Editor observes GitBlame → calls `cx.notify()` on itself
4. Editor re-renders → `DisplayMap::update()`
5. Cascade: `inlay_map.sync → fold_map → tab_map.sync → wrap_map.sync → block_map.sync`
6. `BlockMap::sync` processes entire MultiBuffer (100,000+ rows)
7. **Sync takes 25-145 seconds → stutter**

## Current Code State

`crates/editor/src/git/blame.rs`:
- Chunk size: 4 (reverted to baseline)
- `BLAME_GENERATION_ID` atomic counter for tracing
- Debouncing code exists but NOT active

`crates/git/src/blame.rs`:
- Tracing on sync `spawn_git_blame()` function (not async `for_path()`)
- `generation_id` parameter threaded through for tracing

## Key Files

| File | Purpose |
|------|---------|
| `crates/editor/src/display_map/block_map.rs` | **THE BOTTLENECK** - `sync` function, line ~572 |
| `crates/editor/src/git/blame.rs` | GitBlame implementation, `generate()` function |
| `crates/git/src/blame.rs` | `spawn_git_blame()`, `for_path()` |
| `crates/editor/src/display_map.rs` | DisplayMap orchestrating sync chain |
| `PERF_NOTES.md` | Detailed investigation notes with all runs |
| `PERF_SUMMARY.md` | Clean summary of findings |

## Tracy Traces

- `run_9.tracy` - Baseline (chunk=4), 7 spawns, max sync 51.6s
- `run_11.tracy` - chunk=64, 195 spawns, max sync 144.9s
- `run_12.tracy` - chunk=16, 68 spawns, max sync 124.8s

## Useful Commands

```bash
# Build with Tracy
RUSTFLAGS="-C force-frame-pointers=yes" ZTRACING=1 cargo build --features tracy --release

# Long-running spans (>10ms)
tracy-csvexport -u trace.tracy | awk -F, '$5 > 10000000 {print $1","$5/1000000"ms"}' | sort -t, -k2 -rn | head -30

# Count git blame spawns
tracy-csvexport -u -f "spawn_git_blame" trace.tracy | grep "spawn_git_blame{" | wc -l

# Spawns by generation
tracy-csvexport -u -f "spawn_git_blame" trace.tracy | grep "spawn_git_blame{" | awk -F'generation_id=' '{print $2}' | cut -d'}' -f1 | sort | uniq -c

# Sync operations >10ms
tracy-csvexport -u -f "sync" trace.tracy | awk -F, '$5 > 10000000 {print $1","$5/1000000"ms"}' | sort -t, -k2 -rn | head -20
```

## Test Repo

Chromium repo at `/Users/kubkon/dev/chromium`

## Next Steps (Recommended)

### Approach 1: Optimize BlockMap::sync
1. Profile the `while edits` loop in `block_map.rs:572`
2. Understand why operations on 100k+ rows are slow
3. Consider algorithmic optimizations (chunked/lazy updates)

### Approach 2: Avoid triggering full sync
1. Investigate if blame updates can avoid full DisplayMap sync
2. Different notification mechanism that doesn't cause re-render
3. Partial invalidation - only sync affected buffers

### Approach 3: Architectural change
1. Lazy blame - only blame visible buffers
2. Skip blame entirely for MultiBuffers exceeding size threshold
3. On-demand blame as user scrolls

## Key Insight

The problem is NOT the frequency of blame operations. It's that ANY notification causes an expensive full `BlockMap::sync` on the entire 100k+ row MultiBuffer. The fix must happen at the DisplayMap/BlockMap level, not the GitBlame level.