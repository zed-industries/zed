# Context Handoff: Git Blame Performance Investigation

## Problem
Stuttering when opening large git repos (chromium) with thousands of changed files in git diff view. Hypothesis: originates from `GitBlame::generate` in `crates/editor/src/git/blame.rs`.

## Key Files
- `crates/editor/src/git/blame.rs` - GitBlame implementation, `generate()` function
- `crates/project/src/git_store.rs` - Git store events, `blame_buffer()`
- `crates/git/src/blame.rs` - `Blame::for_path()` - actual git blame execution
- `crates/git/src/repository.rs:1528` - `blame()` function calling `for_path`
- `crates/git_ui/src/project_diff.rs` - ProjectDiff view with MultiBuffer
- `PERF_NOTES.md` - Full analysis documentation

## Code Flow
1. `GitBlame::new()` subscribes to events and calls `generate()`
2. Events that trigger `generate()`:
   - `multi_buffer::Event::DirtyChanged`
   - `project::Event::WorktreeUpdatedEntries`
   - `GitStoreEvent::RepositoryUpdated` (includes StatusesChanged!)
   - `GitStoreEvent::RepositoryAdded/Removed`
   - Focus events
3. `generate()` collects ALL buffer IDs from multi_buffer, processes in chunks of 4
4. Each buffer triggers `project.blame_buffer()` → `git_store.blame_buffer()` → `repository.blame()` → `Blame::for_path()`

## Tracing Spans Added (blame.rs)
- `blame_trigger_init` - constructor
- `blame_trigger_dirty_changed` - DirtyChanged event
- `blame_trigger_worktree_updated` - WorktreeUpdatedEntries event
- `blame_trigger_repo_updated` - RepositoryUpdated event
- `blame_trigger_repo_added/removed` - Add/remove events
- `blame_trigger_focus` - focus event
- `blame_task{buffer_count=N}` - main task with buffer count
- `blame_task_for_buffers` - each chunk of 4 buffers

## Tracy Analysis (run_1.tracy)

### Blame Tasks Timeline
| Task | Start | Duration | Notes |
|------|-------|----------|-------|
| `blame_task{buffer_count=2}` | 16.45s | **128.85s** | Only task that completed |
| `blame_task{buffer_count=249}` | 20.58s | 24µs | Returned immediately |
| `blame_task{buffer_count=1620}` | 33.45s | 41µs | Returned immediately |
| `blame_task{buffer_count=3098}` | 53.94s | 66µs | Returned immediately |

### Critical Finding
**All 5127 `for_path` git blame calls occurred within `blame_task{buffer_count=2}` window (16.45s to 145.3s)**

This is ~2500x more operations than expected for 2 buffers.

### Chunks in buffer_count=2 task
| Chunk | Start | Duration |
|-------|-------|----------|
| 1 | 16.45s | 70.03s |
| 2 | 86.49s | 18.97s |
| 3 | 105.46s | 31.06s |
| 4 | 136.52s | 8.79s |

Expected: 1 chunk (since 2 < 4). Actual: 4 chunks.

## Open Questions
1. Why does `buffer_count=2` produce 4 chunks and 5127 blame operations?
2. Are the same 2 files blamed ~2500 times each, or are there actually more files?
3. Is `buffer_count` captured correctly, or is there a race/bug?

## Useful Commands
```bash
# Build with tracy
RUSTFLAGS="-C force-frame-pointers=yes" ZTRACING=1 cargo build --features tracy --release

# Export tracy data
tracy-csvexport /Users/kubkon/dev/zed/run_1.tracy

# Get individual zone events with timestamps
tracy-csvexport -u /Users/kubkon/dev/zed/run_1.tracy

# Filter for blame spans
tracy-csvexport -u -f "blame" /Users/kubkon/dev/zed/run_1.tracy

# Count for_path in blame_task window
tracy-csvexport -u /Users/kubkon/dev/zed/run_1.tracy | grep "for_path,crates/git/src/blame.rs" | \
  awk -F, '{ if ($4 >= 16452908042 && $4 <= 145304815375) count++ } END { print count }'
```

## Suggested Next Step
Add file path tracing to `for_path` in `crates/git/src/blame.rs:24`:
```rust
#[ztracing::instrument(skip_all, fields(path = %path.as_ref().display()))]
pub async fn for_path(...) {
```

This will reveal if 2 files are blamed repeatedly or if there are actually 5127 different files.

## Test Repo
Chromium repo at `/Users/kubkon/dev/chromium`

---

# Investigation Update (Continued Session)

## ROOT CAUSE CONFIRMED

The stuttering is caused by **orphaned background tasks**. When `generate()` is called:

1. `generate()` calls `cx.spawn(blame_task)` - this CAN be cancelled
2. `blame_task` calls `project.blame_buffer()` for each buffer
3. `blame_buffer()` eventually calls `executor.spawn(for_path)` - this CANNOT be cancelled

When a new `generate()` replaces `self.task`, the parent `blame_task` is cancelled, but all `executor.spawn()` calls already dispatched continue running.

## Tracing Enhancement Added

Added `generation_id` (auto-incremented atomic) to trace orphaned tasks:

**Files modified:**
- `crates/editor/src/git/blame.rs` - Added `BLAME_GENERATION_ID: AtomicU64`, included in `blame_task` span
- `crates/project/src/project.rs` - Added `generation_id: Option<u64>` param to `blame_buffer()`
- `crates/project/src/git_store.rs` - Threading `generation_id` through
- `crates/git/src/repository.rs` - Added to `GitRepository::blame` trait and impl
- `crates/git/src/blame.rs` - Added to `for_path()` span
- `crates/fs/src/fake_git_repo.rs` - Added to `FakeGitRepository::blame()`

## Tracy Analysis Summary

### run_2.tracy (scroll only)
- 1357 `for_path` calls, only 5 unique files
- Same files blamed hundreds of times (DEPS: 844x, AUTHORS: 47x, etc.)

### run_3.tracy (scroll only, with generation_id)
| generation_id | for_path calls | Status |
|---------------|----------------|--------|
| 0 | 3029 | Completed |
| 1 | 1144 | **ORPHANED** (ran 51.77s-81.99s, after blame_task ended at 46.55s) |

### run_4.tracy (with edits)
| generation_id | for_path calls | blame_task duration |
|---------------|----------------|---------------------|
| 0 | 9 | No task recorded |
| 1 | 671 | 19.5µs (cancelled) |
| 2 | 105 | 25.25µs (cancelled) |
| 3 | 166 | 126.25µs (cancelled) |

**All blame_tasks cancelled in microseconds, but for_path work continued for 12+ seconds each.**

## Suggested Fixes (Priority Order)

1. **Cancel background spawns** - Use `AbortHandle` or similar to cancel in-flight `executor.spawn()` calls
2. **Debounce `generate()` calls** - Add delay similar to `regenerate_on_edit()` for all triggers
3. **Filter event types** - Don't regenerate for `StatusesChanged` events (they don't affect blame)
4. **Lazy/visible-only blame** - Only blame buffers currently visible in viewport

## Useful Analysis Commands

```bash
# Count for_path calls per generation_id
tracy-csvexport -u run_X.tracy | grep "for_path{" | \
  sed 's/.*generation_id=Some(\([0-9]*\)).*/\1/' | sort | uniq -c

# Get blame_task timings
tracy-csvexport -u run_X.tracy | grep "blame_task{buffer_count"

# Count unique files blamed
tracy-csvexport -u run_X.tracy | grep "for_path{" | \
  sed 's/.*path=\([^ ]*\) .*/\1/' | sort | uniq -c | sort -rn
```

## Full Details
See `PERF_NOTES.md` for complete analysis with all data.

---

# Fix Attempts (Continued Session 2)

## Why Cancellation Tokens Are Complex

Investigated using `CancellationToken` to cancel background tasks. Key finding:

**GPUI's `Task` does cancel on drop** (documented in `crates/gpui/src/executor.rs:102-103`), but:
- `async_task` semantics: if task is **already running** on a worker thread, it runs to completion
- The `executor.spawn()` in `repository.rs` immediately queues work that starts executing
- By the time parent is dropped, `for_path` tasks are already in-flight

## Fix Attempt 1: Debouncing

Added 100ms debounce to all event-triggered `generate()` calls.

**Changes to `crates/editor/src/git/blame.rs`:**
- Added `debounced_generate_task: Task<Result<()>>` field
- Added `DEBOUNCE_GENERATE_INTERVAL: Duration = 100ms` constant
- Added `debounced_generate()` method with timer
- Changed all event handlers to use `debounced_generate()`:
  - `DirtyChanged`, `WorktreeUpdatedEntries`, `RepositoryUpdated`
  - `RepositoryAdded`, `RepositoryRemoved`, `focus`
- Kept direct `generate()` only for init and inside `regenerate_on_edit`

## Fix Attempt 2: kill_on_drop

Added `kill_on_drop` to smol commands executing `git blame` subprocess.

**Purpose:** Kill the actual git subprocess when Task is dropped, rather than letting it run to completion.

## Tracy Results After Fixes

### run_5.tracy (debounce only)
| Metric | Value |
|--------|-------|
| `for_path` calls | 7813 |
| `generate` calls | 5 |
| generation_ids | 2 (0, 1) |

- gen_id=0: 131 calls, completed successfully (~7s)
- gen_id=1: 7682 calls, **orphaned** (51s-186s = 135 seconds!)

### run_6.tracy (debounce + kill_on_drop, missed one spot)
| Metric | Value |
|--------|-------|
| `for_path` calls | 6055 |
| generation_ids | 4 (0, 1, 2, 3) |

All `blame_task` spans were microseconds (cancelled), but orphaned work continued.

### run_7.tracy (debounce + kill_on_drop complete)
| Metric | Value |
|--------|-------|
| `for_path` calls | 9517 |
| generation_ids | 3 (0, 1, 2) |

| gen_id | for_path | blame_task duration | Span |
|--------|----------|---------------------|------|
| 0 | 155 | **121.4s** (completed!) | 17.6s-19.9s |
| 1 | 5860 | 0.027ms (cancelled) | 19.8s-70.8s (orphaned) |
| 2 | 3502 | 1.4ms (cancelled) | 70.8s-139s (orphaned) |

Same files blamed repeatedly:
- `AUTHORS`: 2996 times
- `DEPS`: 1799 times

## Current Status

**Fixes applied but problem persists:**
1. Debouncing reduces `generate()` call frequency
2. `kill_on_drop` should kill git subprocess on Task drop
3. But orphaned tasks still run for 50-130+ seconds
4. Same small set of files blamed thousands of times

**Remaining investigation needed:**
- Verify `kill_on_drop` is correctly propagating
- Check if there are additional spawn layers not being cancelled
- Consider architectural changes (lazy blame, visible-only blame)
