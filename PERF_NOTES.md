# Problem description

This file describes some manual analysis of stuttering encountered when opening large
git repository in Zed with thousands of changed/untracked files opened in the git diff view.

Our current idea is that this originates from git-blame calculations and starts in `GitBlame::generate`
defined in `crates/editor/src/git/blame.rs`.

1. With this code snippet, the stutter is gone:

```diff
diff --git a/crates/editor/src/git/blame.rs b/crates/editor/src/git/blame.rs
index 15d429ee19..b470ca5c1e 100644
--- a/crates/editor/src/git/blame.rs
+++ b/crates/editor/src/git/blame.rs
@@ -200,9 +200,9 @@ impl GitBlame {
             &multi_buffer,
             |git_blame, multi_buffer, event, cx| match event {
                 multi_buffer::Event::DirtyChanged => {
-                    if !multi_buffer.read(cx).is_dirty(cx) {
-                        git_blame.generate(cx);
-                    }
+                    // if !multi_buffer.read(cx).is_dirty(cx) {
+                    //     git_blame.generate(cx);
+                    // }
                 }
                 multi_buffer::Event::ExcerptsAdded { .. }
                 | multi_buffer::Event::ExcerptsEdited { .. } => git_blame.regenerate_on_edit(cx),
@@ -227,7 +227,7 @@ impl GitBlame {
                         .any(|(_, entry_id, _)| project_entry_id == Some(*entry_id))
                     {
                         log::debug!("Updated buffers. Regenerating blame data...",);
-                        git_blame.generate(cx);
+                        // git_blame.generate(cx);
                     }
                 }
             }
@@ -240,7 +240,7 @@ impl GitBlame {
                 | GitStoreEvent::RepositoryAdded
                 | GitStoreEvent::RepositoryRemoved(_) => {
                     log::debug!("Status of git repositories updated. Regenerating blame data...",);
-                    this.generate(cx);
+                    // this.generate(cx);
                 }
                 _ => {}
             });
@@ -260,7 +260,7 @@ impl GitBlame {
                 git_store_subscription,
             ],
         };
-        this.generate(cx);
+        // this.generate(cx);
         this
     }

@@ -338,7 +338,7 @@ impl GitBlame {
         self.focused = true;
         if self.changed_while_blurred {
             self.changed_while_blurred = false;
-            self.generate(cx);
+            // self.generate(cx);
         }
     }

@@ -644,7 +644,7 @@ impl GitBlame {
                 .await;

             this.update(cx, |this, cx| {
-                this.generate(cx);
+                //     this.generate(cx);
             })
         });
     }
```

2. With this code snippet on the other hand the stutter is visibly worse:

```diff
diff --git a/crates/editor/src/git/blame.rs b/crates/editor/src/git/blame.rs
index 15d429ee19..ae6752e714 100644
--- a/crates/editor/src/git/blame.rs
+++ b/crates/editor/src/git/blame.rs
@@ -514,7 +514,7 @@ impl GitBlame {
             let mut all_results = Vec::new();
             let mut all_errors = Vec::new();

-            for buffers in buffers_to_blame.chunks(4) {
+            for buffers in buffers_to_blame.chunks(16) {
                 let span = ztracing::info_span!("blame_task_for_buffers");
                 let _enter = span.enter();
                 let blame = cx.update(|cx| {
```

3. If we revert PR https://github.com/zed-industries/zed/pull/44843/ we get spinning wheel and
   explode in RAM.

# How we profile the app

We use `tracy` for profiling. In order to enable `tracy` spans, we compile with

```
$ RUSTFLAGS="-C force-frame-pointers=yes" ZTRACING=1 cargo build --features tracy --release
```

We can then launch `tracy` profiler as

```
$ tracy
```

And wait for the connection. Ask user to perform this action if required.

It is possible to save the trace file for further machine analysis. You can ask the user to do that too.
The description of `tracy` and the file format can be found at
[deepwiki.com/wolfpld/tracy](https://deepwiki.com/wolfpld/tracy/4.1-tracy-file-format).

# Our test repo

We use the chromium repo that is available at `/Users/kubkon/dev/chromium`.

# Plan of action

Before we even remotely attempt at fixing this, we should have a much better understanding of where
in the code we might have potential slowness/hot loops.

1. Generate preliminary analysis of where the potential cause might be, and append it into this file.

# Preliminary Analysis

## Key Findings

### 1. Event Storm from Git Store Updates

When a large git repository is opened, the git status scanning emits many `RepositoryEvent::StatusesChanged`
events (see `crates/project/src/git_store.rs:5505`). Each status change triggers:

1. `RepositoryEvent::StatusesChanged` is emitted per batch of status updates
2. `GitStore::on_repository_event()` forwards this as `GitStoreEvent::RepositoryUpdated` (line 1337-1341)
3. `GitBlame`'s git_store subscription catches this and calls `generate()` (blame.rs:238-246)

**Problem:** The subscription doesn't filter by event type - ALL `RepositoryUpdated` events trigger
a full blame regeneration, even though `StatusesChanged` events don't actually affect blame data.

```rust
// blame.rs:238-246
GitStoreEvent::RepositoryUpdated(_, _, _)
| GitStoreEvent::RepositoryAdded
| GitStoreEvent::RepositoryRemoved(_) => {
    this.generate(cx);  // Called for EVERY repository update, including status changes
}
```

### 2. No Debouncing for Git Store Events

Unlike `regenerate_on_edit()` which has a 2-second debounce (blame.rs:653), the git store
event handler calls `generate()` immediately without any coalescing or debouncing.

In a large repo with thousands of changed files, many `StatusesChanged` events may fire
in rapid succession, each triggering a full blame regeneration.

### 3. Full Multi-Buffer Blame on Each Generate

Each `generate()` call:
1. Collects ALL buffer IDs from the multi_buffer (blame.rs:499-507)
2. Processes them in chunks of 4 (blame.rs:517)
3. Runs `project.blame_buffer()` for EACH buffer

When the diff view contains thousands of changed files, each `generate()` call initiates
thousands of git blame operations.

### 4. Interaction with ProjectDiff View

The `ProjectDiff` view (git_ui/src/project_diff.rs) creates a `MultiBuffer` containing
excerpts from all changed files (potentially thousands). When blame is enabled on its
editor:

1. The `GitBlame` entity is created with this multi-buffer
2. Every git store status update triggers `generate()`
3. `generate()` attempts to blame ALL files in the diff view

### 5. No Filtering of Relevant Buffers

The blame code doesn't filter to only blame buffers that might have actually changed.
It always processes the entire multi-buffer, regardless of what triggered the regeneration.

## Potential Hotspots

| Location | Issue |
|----------|-------|
| `blame.rs:238-246` | No filtering of event types - status changes trigger blame |
| `blame.rs:493-636` | No debouncing for git store events |
| `blame.rs:499-507` | Collects ALL buffers without filtering |
| `blame.rs:517` | Chunk size affects concurrent load (4 vs 16 experiment) |
| `git_store.rs:1337` | Every repository event forwarded without aggregation |

## Hypotheses to Test

1. **Primary:** The stuttering is caused by repeated `generate()` calls triggered by
   `StatusesChanged` events during git status scans of large repos.

2. **Secondary:** Even a single `generate()` call is expensive when the multi-buffer
   contains thousands of files.

3. **Tertiary:** The chunking at 4 buffers is a bandaid - the real issue is that we
   shouldn't be blaming all these files in the first place.

## Suggested Next Steps

1. **Add tracing spans** to measure:
   - How often `generate()` is called during the stutter period
   - Which events trigger these calls
   - How many buffers are being blamed per call

2. **Filter event types:** Only regenerate blame on events that could actually change
   blame data (not `StatusesChanged`).

3. **Add debouncing:** Coalesce rapid `generate()` calls similar to `regenerate_on_edit()`.

4. **Consider lazy blame:** Only blame buffers that are visible/scrolled to, not all
   buffers in a multi-buffer.

# Tracy Analysis (run_1.tracy)

## Timeline of Blame Tasks

| Task | Start | Duration | Notes |
|------|-------|----------|-------|
| `blame_task{buffer_count=2}` | 16.45s | **128.85s** | Only task that ran to completion |
| `blame_task{buffer_count=249}` | 20.58s | 24µs | Returned immediately (unfocused?) |
| `blame_task{buffer_count=1620}` | 33.45s | 41µs | Returned immediately |
| `blame_task{buffer_count=3098}` | 53.94s | 66µs | Returned immediately |

## Key Finding: Massive Over-Blaming

**All 5127 `for_path` (git blame) calls occurred within the `blame_task{buffer_count=2}` window.**

This is ~2500x more blame operations than expected for 2 buffers.

### Chunks within `blame_task{buffer_count=2}`

| Chunk | Start | Duration |
|-------|-------|----------|
| 1 | 16.45s | 70.03s |
| 2 | 86.49s | 18.97s |
| 3 | 105.46s | 31.06s |
| 4 | 136.52s | 8.79s |

With 2 buffers and `chunks(4)`, there should be **1 chunk**, not 4.

## Open Questions

1. Why does a task with `buffer_count=2` have 4 sequential chunks?
2. Why are there 5127 git blame operations for 2 buffers?
3. Are the same files being blamed repeatedly, or different files?

## Next Steps

Need to add more tracing to capture:
- The actual buffer IDs being passed to `blame_buffer`
- The file paths being blamed in `for_path`
- Whether buffers are being blamed multiple times

# Tracy Analysis (run_2.tracy)

## Test Scenario

Opened chromium repo with uncommitted changes (git diff view) as active tab. After loading,
scrolled up/down and occasionally positioned cursor in buffers within the multi-buffer.
No edits were made. Stutter was observed.

## Tracing Enhancement

Added file path tracing to `Blame::for_path` in `crates/git/src/blame.rs:24`:

```rust
#[ztracing::instrument(skip_all, fields(path = %path.as_unix_str()))]
pub async fn for_path(...) {
```

Source: `CONTEXT_HANDOFF.md` suggested next step, adapted for `RepoPath` type.

## Key Metrics

| Metric | Value |
|--------|-------|
| Total `for_path` calls | **1357** |
| Unique files blamed | **5** |
| `generate` calls | 6 |
| `blame_task` completions | 1 |
| `blame_buffer` calls | 12 |

## Blame Counts Per File

| File | Times Blamed |
|------|-------------|
| `DEPS` | **844** |
| `android_webview/.../WebViewChromiumFactoryProvider.java` | **234** |
| `.gitmodules` | **145** |
| `android_webview/.../AwWebContentsObserver.java` | **87** |
| `AUTHORS` | **47** |

**The same 5 files were blamed 1357 times total!**

## Timeline Analysis

### Generate Calls
| Time (s) | Trigger |
|----------|---------|
| 0.37 | init |
| 0.38 | repo_added |
| 10.59 | repo_updated |
| 10.59 | repo_updated |
| 17.30 | focus |
| 19.77 | (final, completed) |

### for_path Activity
- First call: 17.3s
- Last call: 53.8s
- Duration: ~36 seconds of blame activity

### Critical Observation

The `for_path` calls continued for **34 seconds after** the last `generate()` call (19.77s → 53.8s).

## Thread Analysis

`for_path` calls were distributed across **34 different threads**:

```
Thread 123: 74 calls
Thread 116: 68 calls
Thread 87:  62 calls
Thread 126: 51 calls
... (30 more threads)
```

Meanwhile, all `blame` span entries (in repository.rs) occurred on thread 4 only,
with just 6 calls recorded.

## Root Cause Identified

### The Task Cancellation Problem

When `generate()` is called, it does:

```rust
self.task = cx.spawn(async move |this, cx| {
    // ... calls project.blame_buffer() for each buffer
});
```

Each subsequent `generate()` call **replaces** `self.task`, which should cancel the previous
async work. However, the actual git blame operations are spawned deeper in the call stack:

```rust
// In repository.rs:1540
executor.spawn(async move {
    crate::blame::Blame::for_path(...)  // This spawn is NOT cancelled!
})
```

### The Cascade

1. `generate()` is called (e.g., from repo_updated event)
2. It collects ALL buffers (potentially thousands) and starts blaming them
3. Each `blame_buffer()` call spawns a background task via `executor.spawn()`
4. Before completion, another event triggers `generate()` again
5. `self.task` is replaced, but the **already-spawned executor tasks continue running**
6. The new `generate()` spawns MORE executor tasks for the same buffers
7. Result: Same files get blamed many times from overlapping `generate()` calls

### Evidence

- 6 `generate` calls occurred
- Only 1 `blame_task` completed (the last one, with 35 buffers)
- But 1357 `for_path` calls executed (from ALL 6 generate calls combined)
- The 5 files blamed match the visible/loaded buffers at different points in time

## Why 5 Files?

The 5 unique files likely represent what was visible in the editor at different scroll
positions during the test:
- Initial view showed some files
- Scrolling brought different files into view
- Each scroll/focus event potentially triggered new `generate()` calls
- The background blame tasks from earlier calls continued running

## Potential Fixes

### 1. Cancel Background Spawns (Recommended)

Use an `AbortHandle` or similar mechanism to cancel in-flight blame operations when
a new `generate()` is called:

```rust
// Store abort handles for spawned tasks
struct GitBlame {
    task: Task<()>,
    abort_handles: Vec<AbortHandle>,  // New field
    // ...
}

fn generate(&mut self, cx: &mut Context<Self>) {
    // Cancel all previous spawns
    for handle in self.abort_handles.drain(..) {
        handle.abort();
    }
    // ... rest of generate
}
```

### 2. Debounce generate() Calls

Add debouncing similar to `regenerate_on_edit()`:

```rust
fn generate_debounced(&mut self, cx: &mut Context<Self>) {
    self.pending_generate = cx.spawn(async move |this, cx| {
        cx.background_executor().timer(Duration::from_millis(100)).await;
        this.update(cx, |this, cx| this.generate(cx)).ok();
    });
}
```

### 3. Filter Event Types

Don't regenerate blame for `StatusesChanged` events:

```rust
GitStoreEvent::RepositoryUpdated(_, _, event) => {
    if !matches!(event, RepositoryEvent::StatusesChanged { .. }) {
        this.generate(cx);
    }
}
```

### 4. Lazy/Visible-Only Blame

Only blame buffers that are currently visible in the viewport, not all buffers
in the multi-buffer.

## Commands Used for Analysis

```bash
# Count total for_path calls
tracy-csvexport -u -f "for_path" run_2.tracy | grep "for_path{path=" | wc -l

# Get unique files and counts
tracy-csvexport -u -f "for_path" run_2.tracy | grep "for_path{path=" | \
  sed 's/,.*//' | sort | uniq -c | sort -rn

# Count unique files
tracy-csvexport -u -f "for_path" run_2.tracy | grep "for_path{path=" | \
  sed 's/,.*//' | sort -u | wc -l

# Get generate call times
tracy-csvexport -u run_2.tracy | grep "^generate,"

# Analyze thread distribution
tracy-csvexport -u run_2.tracy | grep "for_path{path=DEPS}" | \
  awk -F, '{print $6}' | sort | uniq -c | sort -rn
```

# Tracing Enhancement: generation_id (for run_3.tracy)

## Purpose

To confirm the hypothesis that `for_path` calls are being orphaned (their parent `generate()` task
was cancelled but they continue running), we added an auto-incremented atomic ID that ties the
main thread `blame_task` span with each `for_path` call in background threads.

## Changes Made

### 1. Static counter in `crates/editor/src/git/blame.rs`

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static BLAME_GENERATION_ID: AtomicU64 = AtomicU64::new(0);
```

### 2. Increment and capture in `generate()`

```rust
let generation_id = BLAME_GENERATION_ID.fetch_add(1, Ordering::SeqCst);

self.task = cx.spawn(async move |this, cx| {
    let span = ztracing::info_span!("blame_task", buffer_count, generation_id);
    // ...
});
```

### 3. Thread ID through call chain

Added `generation_id: Option<u64>` parameter to:
- `Project::blame_buffer()` in `crates/project/src/project.rs`
- `GitStore::blame_buffer()` in `crates/project/src/git_store.rs`
- `GitRepository::blame()` trait method in `crates/git/src/repository.rs`
- `RealGitRepository::blame()` implementation
- `FakeGitRepository::blame()` implementation in `crates/fs/src/fake_git_repo.rs`
- `Blame::for_path()` in `crates/git/src/blame.rs`

### 4. Include in `for_path` span

```rust
#[ztracing::instrument(skip_all, fields(path = %path.as_unix_str(), generation_id = ?generation_id))]
pub async fn for_path(
    // ...
    generation_id: Option<u64>,
) -> Result<Self> {
```

## Expected Results

In the trace, we should see:
- `blame_task{buffer_count=N, generation_id=X}` spans on the main thread
- `for_path{path=..., generation_id=Some(X)}` spans on background threads

If the hypothesis is correct:
- Multiple `blame_task` spans will have different `generation_id` values (e.g., 0, 1, 2, 3, 4, 5)
- Only the last `blame_task` will run to completion
- But `for_path` calls will have `generation_id` values from ALL generations (0, 1, 2, 3, 4, 5)
- The "orphaned" `for_path` calls are those with `generation_id` < max(generation_id)

## Analysis Commands

```bash
# Count for_path calls per generation_id
tracy-csvexport -u run_3.tracy | grep "for_path{" | \
  sed 's/.*generation_id=Some(\([0-9]*\)).*/\1/' | sort | uniq -c

# Count blame_task spans per generation_id  
tracy-csvexport -u run_3.tracy | grep "blame_task{" | \
  sed 's/.*generation_id=\([0-9]*\).*/\1/' | sort | uniq -c

# Find orphaned for_path calls (generation_id != latest)
# First find the max generation_id, then filter
```

# Tracy Analysis (run_3.tracy) - HYPOTHESIS CONFIRMED

## Test Scenario

Same as run_2: opened chromium repo with uncommitted changes in git diff view, scrolled around.

## Key Metrics

| Metric | Value |
|--------|-------|
| Total `for_path` calls | **4173** |
| `for_path` with `generation_id=0` | **3029** |
| `for_path` with `generation_id=1` | **1144** (orphaned!) |
| `blame_task` completions | **1** (generation_id=0 only) |
| `generate` calls | **7** |

## Timeline - Definitive Evidence

| Event | Time |
|-------|------|
| `blame_task{generation_id=0}` started | 21.61s |
| `blame_task{generation_id=0}` ended | 46.55s |
| First `for_path{generation_id=1}` | **51.77s** |
| Last `for_path{generation_id=1}` | **81.99s** |

**Critical Finding:** The 1144 `for_path` calls with `generation_id=1` started **AFTER** the only
`blame_task` (generation_id=0) had already completed!

## What This Proves

1. A `generate()` call with `generation_id=1` was initiated
2. Its `blame_task` was cancelled (replaced by another `generate()` call before it could start)
3. But the background `executor.spawn()` calls it initiated **continued running for 30+ seconds**
4. These are truly "orphaned" tasks - their parent was cancelled but they kept running

## Files Blamed

### By generation_id=0 (completed task):
| File | Count |
|------|-------|
| `AUTHORS` | 2947 |
| `chrome/browser/ash/login/wizard_controller_browsertest.cc` | 579 |
| `third_party/blink/renderer/core/testing/internals.h` | 371 |
| `ash/shelf/shelf_config.cc` | 144 |
| `.gitmodules` | 82 |

### By generation_id=1 (orphaned task):
| File | Count |
|------|-------|
| `cloud_binary_upload_service_unittest.cc` | 21 |
| (and others from scrolled-to view) | ~1123 |

## Root Cause Confirmed

The issue is in the architecture:

```
generate() 
  └─> cx.spawn(blame_task)           // This task CAN be cancelled
        └─> project.blame_buffer()
              └─> git_store.blame_buffer()
                    └─> cx.spawn(...)
                          └─> backend.blame()
                                └─> executor.spawn(for_path)  // This spawn CANNOT be cancelled!
```

When `self.task` is replaced in `generate()`, it cancels the `blame_task` future. But any
`executor.spawn()` calls that were already dispatched to the background executor continue
running independently.

## Conclusion

**The hypothesis is confirmed.** The stuttering is caused by:
1. Multiple `generate()` calls triggered by events (repo updates, focus, scroll)
2. Each `generate()` spawns background blame tasks via `executor.spawn()`
3. Subsequent `generate()` calls cancel the parent task but NOT the background spawns
4. Result: Thousands of redundant git blame operations running in parallel

# Tracy Analysis (run_4.tracy) - Edit-Triggered Regeneration

## Test Scenario

Same as previous runs, but this time **edited a buffer** within the multi-buffer to trigger
the `regenerate_on_edit` code path.

## Key Metrics

| Metric | Value |
|--------|-------|
| Total `for_path` calls | **951** |
| `generate` calls | **7** |
| `blame_task` spans recorded | **3** (gen_id 1, 2, 3) |

## Distribution by generation_id

| generation_id | `for_path` calls | `blame_task` completed? |
|---------------|------------------|------------------------|
| 0 | 9 | **No** (no blame_task span recorded) |
| 1 | 671 | **No** (duration: 19.5µs - cancelled immediately) |
| 2 | 105 | **No** (duration: 25.25µs - cancelled immediately) |
| 3 | 166 | **No** (duration: 126.25µs - cancelled immediately) |

## Timeline

| generation_id | `for_path` First | `for_path` Last | Duration | Notes |
|---------------|------------------|-----------------|----------|-------|
| 0 | 16.74s | 18.79s | 2.05s | No blame_task visible |
| 1 | 18.79s | 30.78s | **11.99s** | Orphaned work |
| 2 | 30.44s | 32.97s | 2.53s | Orphaned work |
| 3 | 32.97s | 44.92s | **11.95s** | Orphaned work |

## Key Finding

All three visible `blame_task` spans completed in **microseconds** (19-126µs), meaning they
were cancelled almost immediately after starting. Yet their spawned `for_path` operations
continued running for **12+ seconds each**.

### `blame_task` Durations (All Cancelled)

| generation_id | buffer_count | Duration |
|---------------|--------------|----------|
| 1 | 2 | 19.5µs |
| 2 | 450 | 25.25µs |
| 3 | 450 | 126.25µs |

## Files Blamed

| File | Count |
|------|-------|
| `AUTHORS` | 577 |
| `.gitmodules` | 103 |
| `cc/layers/tile_display_layer_impl.cc` | 102 |
| `android_webview/common/aw_features.h` | 94 |
| `chrome/browser/actor/actor_features.h` | 54 |
| `chrome/android/.../IncognitoNewTabPageTest.java` | 21 |

## Comparison with run_3

| Metric | run_3 (scroll only) | run_4 (with edits) |
|--------|---------------------|-------------------|
| Total `for_path` calls | 4173 | 951 |
| Unique generation_ids | 2 (0, 1) | 4 (0, 1, 2, 3) |
| Completed blame_tasks | 1 | 0 |
| Max orphaned duration | ~30s | ~12s |

## Conclusion

Editing triggers even more rapid task replacement via `regenerate_on_edit`, leading to:
- More generation_id values (more cancelled tasks)
- **Zero** blame_tasks completing successfully
- All git blame work is effectively wasted

The 2-second debounce in `regenerate_on_edit` doesn't help because:
1. The debounce only delays calling `generate()`
2. Once `generate()` is called, it still spawns background tasks that can't be cancelled
3. If another event triggers `generate()` before the background work completes, all that work is orphaned

# Fix Attempt: Debouncing All generate() Calls

## Rationale

Since cancelling already-spawned background tasks is complex (they run to completion once picked
up by a worker thread), a simpler approach is to reduce the number of `generate()` calls by
debouncing all event-triggered regenerations.

## Implementation

### Changes to `crates/editor/src/git/blame.rs`

1. Added new field to `GitBlame` struct:
```rust
debounced_generate_task: Task<Result<()>>,
```

2. Added debounce interval constant (100ms):
```rust
const DEBOUNCE_GENERATE_INTERVAL: Duration = Duration::from_millis(100);
```

3. Added `debounced_generate` method:
```rust
fn debounced_generate(&mut self, cx: &mut Context<Self>) {
    self.debounced_generate_task = cx.spawn(async move |this, cx| {
        cx.background_executor()
            .timer(DEBOUNCE_GENERATE_INTERVAL)
            .await;

        this.update(cx, |this, cx| {
            this.generate(cx);
        })
    });
}
```

4. Changed all event handlers to use `debounced_generate()` instead of `generate()`:
   - `DirtyChanged` handler (line 214)
   - `WorktreeUpdatedEntries` handler (line 242)
   - `RepositoryUpdated` handler (line 258)
   - `RepositoryAdded` handler (line 264)
   - `RepositoryRemoved` handler (line 270)
   - `focus` handler (line 374)

5. Kept direct `generate()` call only for:
   - Initial `generate()` in constructor (line 293) - needs immediate blame on creation
   - Inside `regenerate_on_edit` (line 684) - already has 2-second debounce

## Expected Behavior

When multiple events fire in rapid succession (< 100ms apart):
1. Each event calls `debounced_generate()`
2. Each call replaces `debounced_generate_task`, cancelling the previous timer
3. Only after 100ms of "quiet time" does `generate()` actually execute
4. Result: Many fewer `generate()` calls, many fewer orphaned background tasks

## Test Command

```bash
RUSTFLAGS="-C force-frame-pointers=yes" ZTRACING=1 cargo build --features tracy --release
```

Then capture a new trace (run_5.tracy) and compare `for_path` counts with previous runs.

# Critical Discovery: Async Tracing Artifacts

## The Problem

Our earlier traces (run_1 through run_7) showed thousands of `for_path` calls, leading us to believe
git blame was being called excessively. For example, run_7 showed:
- `AUTHORS` blamed 2996 times
- `DEPS` blamed 1799 times

But `generation_id=1` only had `buffer_count=6`, so how could 6 buffers produce 5860 `for_path` calls?

## Root Cause: Async Polling

The `#[ztracing::instrument]` macro on an `async fn` records a span entry **every time the future
is polled**, not just once when called.

```rust
#[ztracing::instrument(skip_all, fields(path = ..., generation_id = ...))]
pub async fn for_path(...) -> Result<Self> {
    let output = run_git_blame(...).await?;  // await point 1
    let messages = get_messages(...).await?;  // await point 2
    Ok(Self { entries, messages })
}
```

A single `for_path` call taking 50+ seconds gets polled hundreds/thousands of times by the async
executor. Each poll records a new span entry!

## Evidence

In run_7, for AUTHORS with generation_id=1:
- **2959 span entries** recorded
- **2956** completed in microseconds (just poll overhead)
- **3** took >1ms (actual work completing)

Only ~3 actual `for_path` calls, polled ~1000 times each!

## The Fix

Moved tracing to a sync function that only runs once per spawn:

```rust
// Old: traced async fn (records per poll)
#[ztracing::instrument(...)]
pub async fn for_path(...) { ... }

// New: traced sync fn (records once per spawn)
#[ztracing::instrument(skip_all, fields(path = %path.as_unix_str(), generation_id = ?generation_id))]
fn spawn_git_blame(...) -> Result<smol::process::Child> {
    util::command::new_smol_command(git_binary)
        .current_dir(working_directory)
        // ...
        .spawn()
}
```

# Tracy Analysis (run_8 & run_9) - With Correct Tracing

## run_8.tracy (with debouncing + kill_on_drop, correct tracing)

| Metric | Value |
|--------|-------|
| `spawn_git_blame` calls | **8** (real count!) |
| `generate` calls | 5 |
| generation_ids | 3 |

## run_9.tracy (NO fixes, correct tracing - baseline)

| Metric | Value |
|--------|-------|
| `spawn_git_blame` calls | **7** |
| `generate` calls | 6 |
| generation_ids | 3 |

### Actual spawns per generation:

| generation_id | spawns | blame_task duration |
|---------------|--------|---------------------|
| 0 | 1 | 26.3 seconds (completed) |
| 1 | 2 | 16µs (cancelled) |
| 2 | 4 | No task recorded |

## Key Insight

**Only 7-8 actual git blame spawns** - not thousands!

The debouncing and kill_on_drop fixes had minimal effect because the actual spawn count was
always low. We were misled by async polling artifacts in our earlier tracing.

# The REAL Culprit: block_map.rs

## Discovery

After fixing the tracing, run_9 (without any fixes) still felt stuttery. Analyzing spans by
duration revealed the true cause:

```bash
tracy-csvexport -u run_9.tracy | awk -F, '$5 > 10000000 {print $1","$5/1000000"ms"}' | sort -t, -k2 -rn | head -20
```

Results:
```
sync{edits=Patch([Edit { old: WrapRow(161428)..WrapRow(161429),...}  51652.5ms
while edits{edit=Edit { old: WrapRow(133179)..WrapRow(133180),...}  51639.2ms
sync{edits=Patch([Edit { old: WrapRow(133134)..WrapRow(133135),...}  51600.2ms
...
```

## The Problem

Operations in `crates/editor/src/display_map/block_map.rs` are taking **25-51 SECONDS** each!

- `BlockMap::sync` processes edits for the MultiBuffer
- The git diff view creates a MultiBuffer with **thousands of files**
- This translates to **100,000+ rows**
- Each sync operation on this massive buffer takes tens of seconds

## Location

`crates/editor/src/display_map/block_map.rs` - `sync` function and `while edits` loop (around line 572)

## Conclusion

**The stuttering is NOT caused by git blame operations.** It's caused by `BlockMap::sync` struggling
to process edits on a MultiBuffer with hundreds of thousands of rows from the git diff view.

This is a completely different performance problem requiring investigation of the block_map
and display_map architecture.

# Chunk Size Experiments (run_11 & run_12)

## Hypothesis

Increasing chunk size (number of buffers processed per iteration in `generate()`) might reduce
the number of `cx.notify()` calls and thus reduce BlockMap::sync triggers.

## Changes Made

Modified `crates/editor/src/git/blame.rs` line 556:
- run_11: `buffers_to_blame.chunks(64)` (up from 4)
- run_12: `buffers_to_blame.chunks(16)` (middle ground)

## Results

### run_11.tracy (chunk=64)

```bash
tracy-csvexport -u -f "spawn_git_blame" run_11.tracy | grep "spawn_git_blame{" | wc -l
# Result: 195

tracy-csvexport -u -f "blame_task{" run_11.tracy | grep "blame_task{"
# blame_task{buffer_count=1 generation_id=0} - 26.1s
# blame_task{buffer_count=2 generation_id=1} - 17μs (cancelled)

tracy-csvexport -u -f "sync" run_11.tracy | awk -F, '$5 > 10000000 {print $1","$5/1000000"ms"}' | sort -t, -k2 -rn | head -5
# sync{edits=Patch([Edit { old: WrapRow(552)..  144971ms  <-- NEW worst case!
# sync{edits=Patch([Edit { old: WrapRow(1227).. 80574.8ms
# sync{edits=Patch([Edit { old: WrapRow(1227).. 80065ms
# sync{edits=Patch([Edit { old: WrapRow(161428).. 50005.1ms
# sync{edits=Patch([Edit { old: WrapRow(133179).. 49992ms
```

**Observation:** UI would freeze briefly when spawning batches - not enough for beachball but noticeable.

### run_12.tracy (chunk=16)

```bash
tracy-csvexport -u -f "spawn_git_blame" run_12.tracy | grep "spawn_git_blame{" | wc -l
# Result: 68

tracy-csvexport -u -f "blame_task_for_buffers" run_12.tracy | grep "blame_task_for_buffers" | wc -l
# Result: 5

tracy-csvexport -u -f "sync" run_12.tracy | awk -F, '$5 > 10000000 {print $1","$5/1000000"ms"}' | sort -t, -k2 -rn | head -5
# sync{edits=Patch([Edit { old: WrapRow(1363).. 124823ms
# sync{edits=Patch([Edit { old: WrapRow(1363).. 124131ms
# sync{edits=Patch([Edit { old: WrapRow(1363).. 124006ms
# sync{edits=Patch([Edit { old: WrapRow(1363).. 123254ms
# sync{edits=Patch([Edit { old: WrapRow(1187).. 121813ms
```

### Comparison Table

| Metric | run_9 (chunk=4) | run_11 (chunk=64) | run_12 (chunk=16) |
|--------|-----------------|-------------------|-------------------|
| `spawn_git_blame` calls | 7 | 195 | 68 |
| `blame_task_for_buffers` chunks | 2 | 4 | 5 |
| Max `sync` duration | 51.6s | 144.9s | 124.8s |
| Typical `sync` durations | ~51s | ~50s + 80s spikes | ~52s + 110-124s spikes |

### Spawns by Generation ID

```bash
# run_9 (chunk=4):
#   1 Some(0)
#   2 Some(1)
#   4 Some(2)
# Total: 7

# run_12 (chunk=16):
#   2 Some(0)
#   2 Some(1)
#  64 Some(2)
# Total: 68
```

## Analysis

**Increasing chunk size BACKFIRED:**

1. **More git spawns, not fewer**: 195 spawns (chunk=64) vs 7 spawns (chunk=4) - 28x increase!

2. **Worse sync times**: Max sync time increased from 51s to 145s with larger chunks.

3. **The cause**: Larger chunks affect how many buffers are ready for blame in subsequent
   `generate()` calls. Generation_id=2 processed 64 buffers with chunk=16 vs only 4 with chunk=4.

4. **UI freezing**: With chunk=64, spawning many git processes simultaneously caused brief UI hangs
   during the synchronous setup phase in `cx.update()`.

## Conclusion

Tuning chunk size is NOT the solution. The relationship between chunk size and total spawns is
non-linear and counterproductive. Larger chunks lead to more total work, not less.

**Reverted to chunk=4 (baseline).**

The real problem remains `BlockMap::sync` performance on large MultiBuffers, which needs to be
addressed at the display_map level, not the blame level.
