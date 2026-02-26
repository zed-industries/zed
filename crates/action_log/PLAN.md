# Checkpoint-Based Action Log

## Problem

The action log currently relies on tools explicitly self-reporting their edits via
`buffer_read()` → edit → `buffer_edited()` calls. This means:

- Terminal-based edits are invisible to the action log
- ACP integrations like Claude Code that don't participate in the action log show no diffs
- Any file modification that doesn't go through the edit tool (subprocesses, scripts, etc.) is missed

## Idea

Use git checkpoints to detect **all** file changes, regardless of how they were made, and
populate the action log from the resulting diff. Git checkpoints already exist in the codebase
for the restore/revert feature — this plan extends them to also drive the action log.

## How Checkpoints Work Today

A checkpoint (`GitRepositoryCheckpoint`) is a lightweight git commit created using a temporary
index (never touches the real `.git/index`). The flow is:

1. Copy `.git/index` to a temp file (UUID-named, so concurrent checkpoints are safe)
2. Set `GIT_INDEX_FILE` to the temp copy
3. Apply exclude rules (`checkpoint.gitignore` + files ≥ 2MB)
4. `git add --all` → `git write-tree` → `git commit-tree`
5. Store the resulting commit SHA

**Storage cost**: Minimal. Tree objects are deduplicated by git. Unchanged checkpoints reuse
the same SHA. Only genuinely new file content creates new blobs (compressed).

**Compute cost**: ~50-200ms per checkpoint on a typical project. The `git add --all` (which
hashes files to detect changes) is the expensive part.

**Concurrency safety**: Each `Repository` entity has a sequential job queue
(`spawn_local_git_worker`), so concurrent checkpoint requests on the same repo are serialized
automatically. The temp index uses a unique UUID per call. The `.git/info/exclude` manipulation
is also safe because the job queue prevents concurrent access within a single repo.

### Existing Infrastructure

| What | Where | Does |
|------|-------|------|
| `GitRepository::checkpoint()` | `crates/git/src/repository.rs` | Creates a checkpoint commit, returns `GitRepositoryCheckpoint { commit_sha }` |
| `GitStore::checkpoint()` | `crates/project/src/git_store.rs` | Aggregates across all repos, returns `GitStoreCheckpoint` (HashMap of path → checkpoint) |
| `GitRepository::diff_checkpoints()` | `crates/git/src/repository.rs` | `git diff --find-renames --patch base target`, returns unified diff string |
| `GitRepository::compare_checkpoints()` | `crates/git/src/repository.rs` | `git diff-tree --quiet`, returns bool (equal or not) |
| `GitStore::compare_checkpoints()` | `crates/project/src/git_store.rs` | Aggregates comparison across repos |
| `Repository::diff_checkpoints()` | `crates/project/src/git_store.rs` | Per-repo wrapper via job queue |
| `AcpThread::update_last_checkpoint()` | `crates/acp_thread/src/acp_thread.rs` | End-of-turn: takes new checkpoint, compares to old, sets `checkpoint.show` flag |
| `checkpoint.gitignore` | `crates/git/src/checkpoint.gitignore` | Excludes binaries, media, archives, IDE files, language artifacts |

### What's Missing at the Git Level

- **`GitStore::diff_checkpoints()`** — aggregated across repos (only exists on individual `Repository` today)
- **`GitRepository::changed_files_between_checkpoints()`** — a lightweight `git diff-tree -r --name-status` to get just the file list without full patch content
- **`GitRepository::show_file_at_checkpoint()`** — `git show <sha>:<path>` to get file content at a specific checkpoint

## Design

### Core Concept

After each batch of parallel tool calls completes, take a git checkpoint. Diff it against the
turn's baseline checkpoint. For any changed files not already tracked by the existing action log,
open the buffer and populate the action log with the checkpoint-derived diff base. This plugs
directly into the existing `BufferDiff` → multibuffer → editor UI pipeline with zero changes to
rendering code.

### When to Checkpoint

Checkpointing is gated on two conditions:

1. **A tool that modifies files has completed.** Tools signal whether they should trigger a
   checkpoint based on their `ToolKind`:
   - `Edit`, `Execute`, `Delete`, `Move` → yes (these can modify the filesystem)
   - `Read`, `Search`, `Fetch`, `Think`, `SwitchMode` → no
   
2. **All parallel tool calls have settled.** When a tool completes, check
   `has_in_progress_tool_calls()`. If it returns `false`, every tool in the current parallel
   batch is done — this is the moment to checkpoint.

The combination of these two conditions means:
- We don't checkpoint after read-only tools
- We don't checkpoint mid-batch when parallel tools are still running (avoiding inconsistent
  filesystem snapshots)
- We do checkpoint once per batch of file-modifying tools, capturing the complete result

This works identically for both the native Zed agent (which sends ACP updates internally) and
external ACP agents like Claude Code, because both go through `AcpThread::upsert_tool_call_inner`
which receives `ToolCallStatus` transitions and has access to `ToolKind`.

### Populating the Action Log

After taking a checkpoint:

1. **Get changed files**: Use `git diff-tree -r --name-status` between the turn's baseline
   checkpoint and the new checkpoint. This returns a list of `(status, path)` tuples — Added,
   Modified, or Deleted.

2. **For each changed file not already in the action log's `tracked_buffers`**:
   - Open the buffer via `project.open_buffer()` (gives current content — the "after")
   - Get the pre-turn content via `git show <baseline_checkpoint_sha>:<path>` (the "before")
   - Create a `TrackedBuffer` in the action log with the "before" content as `diff_base`
   - Mark it as agent-edited so `BufferDiff` computes the diff

3. The existing UI (`AgentDiffPane`, activity bar, reviewing editors) reads `changed_buffers()`
   which returns `BTreeMap<Entity<Buffer>, Entity<BufferDiff>>` — these checkpoint-derived
   entries appear alongside any tool-reported entries with no UI changes needed.

### Data Flow

```text
Tool completes (Edit/Execute/Delete/Move)
  → AcpThread::upsert_tool_call_inner detects Completed status
  → Sets needs_checkpoint flag
  → Checks has_in_progress_tool_calls()
  → If false (all parallel tools done):
      → GitStore::checkpoint()
      → git diff-tree --name-status baseline..new  (changed file list)
      → For each new file:
          → git show baseline:<path>  (old content)
          → project.open_buffer()    (current content)
          → action_log.track_buffer_from_checkpoint(buffer, old_content)
      → Store new checkpoint (replaces previous rolling checkpoint)
      → Continue turn normally
```

### Scope Decisions

- **User edits during a turn are rolled in.** The checkpoint captures total filesystem state,
  not per-author attribution. This is acceptable for agentic workflows where users are mostly
  hands-off during a turn.

- **Accept/reject is cut from initial scope.** The existing hunk-level accept/reject UI works
  with the tool-reported action log. Checkpoint-derived entries initially just show the diff.
  Accept/reject can be added later since we have the `BufferDiff` entities.

- **Non-git projects are not supported.** Zed currently only supports git. If no git repo is
  present, the checkpoint-based action log simply doesn't activate.

- **Rolling checkpoints overwrite.** We don't keep every intermediate checkpoint — each new
  one replaces the previous rolling checkpoint. The turn baseline checkpoint is kept for the
  duration of the turn (it's already stored on the `UserMessage`). The rolling checkpoint
  just represents "latest known state."

### Feature Flag

A setting (e.g. `"agent.checkpoint_action_log": true`) controls whether the checkpoint-based
action log runs. When disabled, only the existing tool-reported action log operates. This
allows running both in parallel during development and A/B testing before committing to one
approach.

## Implementation Plan

### Phase 1: Git Plumbing

Add the missing git-level operations.

**`GitRepository` trait** (`crates/git/src/repository.rs`):
- `changed_files_between_checkpoints(base, target)` → runs
  `git diff-tree -r -z --name-status base target`, returns `Vec<(TreeDiffStatus, RepoPath)>`
- `show_file_at_checkpoint(checkpoint, path)` → runs `git show <sha>:<path>`, returns file
  content as `String`

**`GitStore`** (`crates/project/src/git_store.rs`):
- `diff_checkpoints(base, target)` → aggregates `changed_files_between_checkpoints` across
  all repos, returns combined list of `(status, project_path)` tuples

**`FakeGitRepository`** (`crates/fs/src/fake_git_repo.rs`):
- Implement the new trait methods for test support

### Phase 2: Rolling Checkpoints in AcpThread

Add checkpoint tracking to the turn lifecycle.

**`AcpThread`** (`crates/acp_thread/src/acp_thread.rs`):
- Add fields:
  - `needs_checkpoint: bool` — set when a file-modifying tool completes
  - `rolling_checkpoint: Option<GitStoreCheckpoint>` — latest checkpoint within current turn
  - `turn_baseline_checkpoint: Option<GitStoreCheckpoint>` — checkpoint taken at turn start
    (this is already stored on `UserMessage.checkpoint`, but having a direct reference is
    cleaner)
- In `upsert_tool_call_inner`, when status is `Completed` or `Failed`:
  - Check `ToolKind` — if it's `Edit`/`Execute`/`Delete`/`Move`, set `needs_checkpoint = true`
  - Check `has_in_progress_tool_calls()` — if false and `needs_checkpoint`:
    - Take checkpoint, store as `rolling_checkpoint`
    - Clear `needs_checkpoint`
    - Emit a new `AcpThreadEvent::CheckpointReady` event

### Phase 3: Action Log Population from Checkpoints

Connect checkpoint diffs to the action log.

**`ActionLog`** (`crates/action_log/src/action_log.rs`):
- Add `track_buffer_from_checkpoint(buffer, base_content, cx)`:
  - Like `track_buffer_internal` but sets `diff_base` to the provided `base_content`
    (rather than the buffer's current content)
  - Schedules a diff update as `ChangeAuthor::Agent`

**`AcpThread`** (or a new handler responding to `CheckpointReady`):
- Diff `turn_baseline_checkpoint` against `rolling_checkpoint` via `GitStore::diff_checkpoints`
- For each changed file:
  - Skip if already in `action_log.tracked_buffers`
  - Open the buffer
  - Get old content via `show_file_at_checkpoint`
  - Call `action_log.track_buffer_from_checkpoint(buffer, old_content)`

### Phase 4: Feature Flag and Testing

- Add the `"agent.checkpoint_action_log"` setting to `AgentSettings`
- Gate the checkpoint logic behind the setting
- Add tests:
  - Tool call completes → checkpoint taken → action log populated
  - Parallel tool calls → single checkpoint after all complete
  - Read-only tools → no checkpoint
  - File created/deleted detection
  - Files already tracked by tool-reported action log are not duplicated

## Files to Modify

| File | Change |
|------|--------|
| `crates/git/src/repository.rs` | Add `changed_files_between_checkpoints`, `show_file_at_checkpoint` to trait + `RealGitRepository` impl |
| `crates/fs/src/fake_git_repo.rs` | Implement new trait methods for `FakeGitRepository` |
| `crates/project/src/git_store.rs` | Add `GitStore::diff_checkpoints` (aggregated), `Repository` wrappers |
| `crates/acp_thread/src/acp_thread.rs` | Add checkpoint fields, trigger logic in `upsert_tool_call_inner`, new event |
| `crates/action_log/src/action_log.rs` | Add `track_buffer_from_checkpoint` method |
| `crates/agent_settings/src/*.rs` | Add `checkpoint_action_log` setting |