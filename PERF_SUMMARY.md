# Git Diff View Performance Investigation - Summary

## Problem

Stuttering when opening large git repos (chromium) with thousands of changed files in the git diff view.

## Initial Hypothesis (INCORRECT)

We initially believed the stuttering was caused by excessive git blame operations. Early Tracy traces showed thousands of `for_path` calls, suggesting git blame was being called repeatedly for the same files.

## Critical Discovery: Async Tracing Artifacts

The `#[ztracing::instrument]` macro on an `async fn` records a span entry **every time the future is polled**, not just once when called.

A single `for_path` call taking 50+ seconds gets polled hundreds of times by the async executor. Each poll records a new span entry, making it appear as thousands of separate calls.

### Evidence

- Trace showed 2959 "calls" for one file with `generation_id=1`
- But `generation_id=1` only had `buffer_count=6` buffers
- Analysis: 2956 spans completed in microseconds (poll overhead), only 3 took >1ms (actual work)
- Conclusion: ~3 real calls, polled ~1000 times each

### The Fix

Moved tracing from async function to sync spawn function:

```rust
// Traced sync fn - records once per actual spawn
#[ztracing::instrument(skip_all, fields(path = %path.as_unix_str(), generation_id = ?generation_id))]
fn spawn_git_blame(...) -> Result<smol::process::Child> {
    util::command::new_smol_command(git_binary)
        .current_dir(working_directory)
        .spawn()
}
```

## Correct Results (run_8 & run_9)

With corrected tracing:

| Metric | run_8 (with fixes) | run_9 (no fixes) |
|--------|-------------------|------------------|
| Actual `spawn_git_blame` calls | 8 | 7 |
| `generate` calls | 5 | 6 |

**Only 7-8 actual git blame spawns** across the entire session - not thousands.

The debouncing and kill_on_drop fixes we implemented had minimal effect because the actual spawn count was always low.

## Debouncing Experiments (run_14, run_15, run_16)

We tested various debouncing approaches to reduce `generate()` call frequency:

### run_14 & run_15: Debounce Inside Task (FAILED)

Placed debounce timer inside the spawned task in `generate()`:

```rust
self.task = cx.spawn(async move |this, cx| {
    cx.background_executor().timer(DEBOUNCE_INTERVAL).await;  // Debounce here
    // ... actual blame work
});
```

**Problem:** Each `generate()` call still increments generation_id, collects buffers, and spawns a new task before the debounce. Result: 3000+ `generate()` calls instead of ~6.

### run_16: Separate Debounce Task (SUCCESS)

Debounce happens **before** entering `generate()`:

```rust
fn debounced_generate(&mut self, cx: &mut Context<Self>) {
    self.debounced_generate_task = cx.spawn(async move |this, cx| {
        cx.background_executor().timer(GENERATE_DEBOUNCE_INTERVAL).await;
        this.update(cx, |this, cx| this.generate(cx))
    });
}
```

| Metric | run_9 (baseline) | run_14/15 (broken) | run_16 (fixed) |
|--------|------------------|-------------------|----------------|
| `generate` calls | 6 | 3102-3103 | **4** ✅ |
| `spawn_git_blame` calls | 7 | 36-144 | 42 |
| `blame_task` completions | 1 | 1 | **2** ✅ |
| Max `sync` duration | 51.6s | 47-49s | 55.5s |

The debouncing now works correctly. Sync times remain ~50s, confirming the problem is in `BlockMap::sync`.

## Definitive Proof: run_13 (Git Blame Disabled)

To confirm that `BlockMap::sync` slowness is independent of git blame, we ran a test with
all `generate()` calls commented out in `crates/editor/src/git/blame.rs`.

| Metric | run_9 (with blame) | run_13 (no blame) |
|--------|-------------------|-------------------|
| `spawn_git_blame` calls | 7 | **0** |
| `generate` calls | 6 | **0** |
| Max `sync` duration | 51.6s | **51.7s** |
| Typical `sync` durations | 25-51s | **21-51s** |

**The sync times are essentially identical!** This definitively proves:

1. **Git blame is NOT the cause** of the slow `BlockMap::sync` operations
2. The slowness is inherent to processing a MultiBuffer with 100,000+ rows
3. The stuttering occurs during initial loading/rendering and scrolling of the git diff view

## The REAL Culprit: block_map.rs

After fixing tracing, analysis of long-running spans revealed the true cause:

```
sync{edits=Patch([Edit { old: WrapRow(161428)..WrapRow(161429),...}  51652.5ms
while edits{edit=Edit { old: WrapRow(133179)..WrapRow(133180),...}  51639.2ms
sync{edits=Patch([Edit { old: WrapRow(133134)..WrapRow(133135),...}  51600.2ms
```

Operations in `crates/editor/src/display_map/block_map.rs` are taking **25-51 SECONDS** each.

### Why

- `BlockMap::sync` processes edits for the MultiBuffer
- Git diff view creates a MultiBuffer with **thousands of files**
- This translates to **100,000+ rows**
- Each sync operation on this massive buffer takes tens of seconds

### Location

`crates/editor/src/display_map/block_map.rs` - `sync` function, `while edits` loop (around line 572)

## Chunk Size Experiments (run_11 & run_12)

Tested whether larger chunk sizes (buffers processed per iteration) would reduce sync triggers.

### Results

| Metric | run_9 (chunk=4) | run_11 (chunk=64) | run_12 (chunk=16) |
|--------|-----------------|-------------------|-------------------|
| `spawn_git_blame` calls | 7 | 195 | 68 |
| Max `sync` duration | 51.6s | 144.9s | 124.8s |

### Conclusion

**Increasing chunk size BACKFIRED:**
- 28x more git spawns with chunk=64 vs chunk=4
- Worse sync times (145s max vs 51s baseline)
- UI freezing during batch spawns

Reverted to chunk=4. Tuning chunk size is NOT the solution.

## Root Cause Summary

**The stuttering is caused by git blame completion triggering expensive `BlockMap::sync` operations.**

Key evidence:
- Disabling all `generate()` calls eliminates the stutter (documented at start of investigation)
- But git blame only spawns 7-8 times total
- `BlockMap::sync` operations take 25-51 seconds each

The causal chain:
1. Git blame completes (after 26+ seconds)
2. Blame results trigger updates/notifications to the editor
3. These updates cause `BlockMap::sync` to run on the massive MultiBuffer (100,000+ rows)
4. The sync takes 25-51 seconds → **stutter**

So git blame spawns are not excessive, but their **completion triggers** the expensive sync operations on the large buffer.

## Key Files

- `crates/editor/src/display_map/block_map.rs` - sync operations taking 25-51s (what's slow)
- `crates/editor/src/git/blame.rs` - GitBlame implementation (triggers the slow operations)
- `crates/git/src/blame.rs` - `Blame::for_path()` git blame execution

## Useful Commands

```bash
# Build with tracy
RUSTFLAGS="-C force-frame-pointers=yes" ZTRACING=1 cargo build --features tracy --release

# Find long-running spans (>10ms)
tracy-csvexport -u trace.tracy | awk -F, '$5 > 10000000 {print $1","$5/1000000"ms"}' | sort -t, -k2 -rn | head -30

# Count actual git blame spawns
tracy-csvexport -u -f "spawn_git_blame" trace.tracy | grep "spawn_git_blame{" | wc -l

# Get spawn_git_blame details
tracy-csvexport -u -f "spawn_git_blame" trace.tracy | grep "spawn_git_blame{"

# Count spawns by generation
tracy-csvexport -u -f "spawn_git_blame" trace.tracy | grep "spawn_git_blame{" | awk -F'generation_id=' '{print $2}' | cut -d'}' -f1 | sort | uniq -c
```

## Current Code State

`crates/editor/src/git/blame.rs`:
- `GENERATE_DEBOUNCE_INTERVAL` = 200ms for event-triggered regeneration
- `REGENERATE_ON_EDIT_DEBOUNCE_INTERVAL` = 2 seconds for edit-triggered
- Separate `debounced_generate_task` and `regenerate_on_edit_task` fields
- Event handlers call `debounced_generate(cx)`, not `generate(cx)` directly
- Bulk update pattern: collect all results, single `cx.notify()` at end

## Next Steps

Since run_13 confirmed the problem is entirely in `BlockMap::sync` (not git blame), focus on:

### Approach 1: Optimize BlockMap::sync
1. Profile the `while edits` loop in `block_map.rs` (around line 572)
2. Understand why operations on large row counts are slow
3. Consider algorithmic optimizations (chunked/lazy updates, better data structures)

### Approach 2: Architectural changes to display map
1. Investigate if large MultiBuffers can use a different sync strategy
2. Consider lazy/incremental sync that only processes visible regions
3. Add size thresholds to skip or defer expensive operations