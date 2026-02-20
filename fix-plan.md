# Fix Plan: Prevent Nested Git Worktree Creation

## Problem

When creating a new git worktree from a project that is **itself** a git worktree,
the new worktree gets created in a nested `worktrees/` directory rather than as a
sibling of the current worktree.

### Example

1. Original repo lives at `~/code/zed5`.
2. The `git.worktree_directory` setting is `"../worktrees"` (the default).
3. From `~/code/zed5`, "New Worktree" correctly creates `~/code/worktrees/zed5/agent-aaa`.
4. From that worktree, "New Worktree" **should** create `~/code/worktrees/zed5/agent-bbb`
   (a sibling). Instead it creates `~/code/worktrees/zed5/worktrees/agent-aaa/agent-bbb`.
5. Each successive "New Worktree" from within a worktree nests one level deeper.

### Root Cause

The function `resolve_worktree_directory` (in `crates/git/src/repository.rs`) computes
the worktree output directory by applying the relative `worktree_directory` setting to
the provided `working_directory`. Both call sites pass `repo.work_directory_abs_path` as
that working directory, which is the **current checkout's** path — not the original
repository's path.

Trace through the math for the nested case:

```
working_directory = ~/code/worktrees/zed5/agent-aaa   (current worktree checkout)
setting           = "../worktrees"

joined   = ~/code/worktrees/zed5/agent-aaa/../worktrees
resolved = ~/code/worktrees/zed5/worktrees              (normalized)

Does resolved start with working_directory? No.
So: repo_dir_name = "agent-aaa"
Result: ~/code/worktrees/zed5/worktrees/agent-aaa        ← NESTED!
```

What we **want** is to always resolve relative to the **original repository** that
this worktree was created from:

```
working_directory = ~/code/zed5                          (original repo)
setting           = "../worktrees"

joined   = ~/code/zed5/../worktrees
resolved = ~/code/worktrees

Does resolved start with working_directory? No.
So: repo_dir_name = "zed5"
Result: ~/code/worktrees/zed5                            ← FLAT, correct!
```

## Affected Call Sites

There are two places that call `validate_worktree_directory` with
`repo.work_directory_abs_path`:

1. **`crates/agent_ui/src/agent_panel.rs`** ~line 2219, in
   `handle_worktree_creation_requested`:

   ```rust
   let work_dir = repo.work_directory_abs_path.clone();
   let directory = validate_worktree_directory(&work_dir, &worktree_directory_setting)?;
   ```

2. **`crates/git_ui/src/worktree_picker.rs`** ~line 278, in
   `WorktreeListDelegate::create_worktree`:

   ```rust
   let work_dir = repo.work_directory_abs_path.clone();
   let directory = validate_worktree_directory(&work_dir, &worktree_directory_setting)?;
   ```

Both need to use the **original repo's working directory** instead of the current
worktree's checkout path.

## Fix Design

### Key Insight

Git already tracks the relationship between a worktree and its parent repository via
the "common directory" (`.git` for normal repos, or the main repo's `.git` for
worktrees). The `GitRepository` trait already exposes this:

```rust
// crates/git/src/repository.rs
fn main_repository_path(&self) -> PathBuf {
    let repo = self.repository.lock();
    repo.commondir().into()   // e.g. ~/code/zed5/.git
}
```

For a normal (non-worktree) checkout, `commondir()` returns the repo's own `.git` dir.
For a git worktree, it returns the **main repo's** `.git` dir. So
`commondir().parent()` gives us the main repo's working directory.

### Why It Must Live in the Snapshot (Proto)

The `RepositorySnapshot` is serialized over the wire via the `UpdateRepository` proto
message for SSH remoting. When a user is SSH-remoted into a machine and hits "New
Worktree", the **client** computes the worktree output directory and sends it to the
server in `proto::GitCreateWorktree.directory`. If the client doesn't know the main
repo's working directory, it will compute the wrong nested path — same bug, just
over SSH.

So the information must be included in `RepositorySnapshot` and the proto, not just
stored locally on the `Repository` struct.

### Step-by-Step Changes

#### 1. Add proto field: `crates/proto/proto/git.proto`

Add a new optional field to `UpdateRepository`:

```protobuf
message UpdateRepository {
    // ... existing fields 1-15 ...
    optional string main_worktree_abs_path = 16;
}
```

This carries the original repo's working directory path. It's optional so that older
hosts that don't send it degrade gracefully (the client falls back to
`work_directory_abs_path`, preserving current behavior).

After editing the proto, regenerate Rust bindings by running `script/generate-protos`.

#### 2. Add field to `RepositorySnapshot`: `crates/project/src/git_store.rs`

Add to the `RepositorySnapshot` struct:

```rust
pub struct RepositorySnapshot {
    // ... existing fields ...
    /// The working directory of the original (main) repository. For a normal
    /// checkout this equals `work_directory_abs_path`. For a git worktree
    /// checkout, this is the main repo's working directory — used to anchor
    /// new worktree creation so they don't nest.
    pub main_work_directory_abs_path: Arc<Path>,
}
```

#### 3. Update `RepositorySnapshot::empty`

Accept and store the new field. For call sites that don't have the information
(e.g. `Repository::remote` before the first update arrives), pass
`work_directory_abs_path.clone()` as the default — this preserves current behavior.

```rust
fn empty(
    id: RepositoryId,
    work_directory_abs_path: Arc<Path>,
    main_work_directory_abs_path: Option<Arc<Path>>,
    path_style: PathStyle,
) -> Self {
    Self {
        main_work_directory_abs_path: main_work_directory_abs_path
            .unwrap_or_else(|| work_directory_abs_path.clone()),
        work_directory_abs_path,
        // ... rest unchanged ...
    }
}
```

#### 4. Compute `main_work_directory_abs_path` from `common_dir_abs_path`

In `GitStore::update_repositories_from_worktree` (~line 1491), the
`UpdatedGitRepository` already carries `common_dir_abs_path` but it's currently
discarded with `_common_dir_abs_path`. Stop discarding it and compute the main
working directory:

```rust
} else if let UpdatedGitRepository {
    new_work_directory_abs_path: Some(work_directory_abs_path),
    dot_git_abs_path: Some(dot_git_abs_path),
    repository_dir_abs_path: Some(_repository_dir_abs_path),
    common_dir_abs_path: Some(common_dir_abs_path),  // <-- stop ignoring
    ..
} = update
{
    let main_work_directory_abs_path = derive_main_work_directory(&common_dir_abs_path);
    // ... pass to Repository::local and RepositorySnapshot::empty ...
}
```

Add a helper function (in `git_store.rs` or in `crates/git/src/repository.rs`):

```rust
/// Given the git common directory (from `commondir()`), derive the main
/// repository's working directory.
///
/// For a standard checkout, `common_dir` is `<work_dir>/.git`, so the parent
/// is the working directory. For a git worktree, `common_dir` is the **main**
/// repo's `.git` directory, so the parent is the main repo's working directory.
///
/// Falls back to returning `common_dir` itself if it doesn't end with `.git`
/// (e.g. bare repos or unusual layouts).
pub fn main_work_directory_from_common_dir(common_dir: &Path) -> PathBuf {
    if common_dir.file_name() == Some(std::ffi::OsStr::new(".git")) {
        common_dir
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| common_dir.to_path_buf())
    } else {
        // Bare repo or unusual layout — no meaningful "main work dir" to derive.
        common_dir.to_path_buf()
    }
}
```

#### 5. Thread through `Repository::local`

Add a `main_work_directory_abs_path: Arc<Path>` parameter to `Repository::local()`
(~line 3715) and pass it into `RepositorySnapshot::empty`.

Update the call site in `update_repositories_from_worktree` to pass it.

#### 6. Serialize in `initial_update` and `build_update`

In `RepositorySnapshot::initial_update` (~line 3449):

```rust
fn initial_update(&self, project_id: u64) -> proto::UpdateRepository {
    proto::UpdateRepository {
        // ... existing fields ...
        main_worktree_abs_path: Some(
            self.main_work_directory_abs_path.to_string_lossy().into_owned()
        ),
    }
}
```

Similarly in `build_update` (~line 3527).

#### 7. Deserialize in `apply_remote_update`

In `Repository::apply_remote_update` (~line 5867):

```rust
if let Some(main_path) = update.main_worktree_abs_path {
    self.snapshot.main_work_directory_abs_path = Path::new(&main_path).into();
}
// If the field is absent (old host), leave it as-is (defaults to work_directory_abs_path).
```

Also handle it in `Repository::remote()` / `handle_update_repository` — on the
initial creation from proto, if the field is present, pass it through; otherwise
default to `work_directory_abs_path`.

#### 8. Update the two call sites

**`crates/agent_ui/src/agent_panel.rs`** ~line 2219:

```rust
// BEFORE:
let work_dir = repo.work_directory_abs_path.clone();
let directory = validate_worktree_directory(&work_dir, &worktree_directory_setting)?;

// AFTER:
let main_work_dir = repo.main_work_directory_abs_path.clone();
let directory = validate_worktree_directory(&main_work_dir, &worktree_directory_setting)?;
```

Keep using `repo.work_directory_abs_path` for `path_remapping` — that maps open file
paths from the current worktree to the new one, so it must stay relative to the current
checkout.

**`crates/git_ui/src/worktree_picker.rs`** ~line 278:

```rust
// BEFORE:
let work_dir = repo.work_directory_abs_path.clone();
let directory = validate_worktree_directory(&work_dir, &worktree_directory_setting)?;

// AFTER:
let main_work_dir = repo.main_work_directory_abs_path.clone();
let directory = validate_worktree_directory(&main_work_dir, &worktree_directory_setting)?;
```

#### 9. Update existing tests

The `resolve_worktree_directory` and `validate_worktree_directory` tests in
`crates/git/src/repository.rs` don't need to change — they're pure functions that test
path math. The change is in **what path gets passed to them**.

Update any tests in `agent_panel.rs` or `worktree_picker.rs` that mock repository
creation to set `main_work_directory_abs_path` appropriately (usually equal to
`work_directory_abs_path` for a non-worktree repo).

#### 10. Add new tests

Add a test (ideally in `crates/git/src/repository.rs` near the existing
`test_resolve_worktree_directory`) that verifies the helper function:

```rust
#[test]
fn test_main_work_directory_from_common_dir() {
    // Normal repo: common_dir is <work_dir>/.git
    assert_eq!(
        main_work_directory_from_common_dir(Path::new("/code/zed5/.git")),
        PathBuf::from("/code/zed5")
    );

    // Worktree: common_dir is the main repo's .git
    // (same result — that's the point, it always traces back to the original)
    assert_eq!(
        main_work_directory_from_common_dir(Path::new("/code/zed5/.git")),
        PathBuf::from("/code/zed5")
    );

    // Bare repo: no .git suffix, returns as-is
    assert_eq!(
        main_work_directory_from_common_dir(Path::new("/code/zed5.git")),
        PathBuf::from("/code/zed5.git")
    );
}
```

Add an integration-style test that creates a worktree from a worktree and verifies
the output path is flat (not nested). This could live near the existing
`test_create_and_list_worktrees` test in `crates/git/src/repository.rs`.

## Files Changed (Summary)

| File | Change |
|------|--------|
| `crates/proto/proto/git.proto` | Add `optional string main_worktree_abs_path = 16` to `UpdateRepository` |
| `crates/project/src/git_store.rs` | Add `main_work_directory_abs_path` field to `RepositorySnapshot`; thread `common_dir_abs_path` through `Repository::local`; serialize/deserialize in `initial_update`, `build_update`, `apply_remote_update`; update `RepositorySnapshot::empty` |
| `crates/git/src/repository.rs` | Add `main_work_directory_from_common_dir` helper function and tests |
| `crates/agent_ui/src/agent_panel.rs` | Use `repo.main_work_directory_abs_path` instead of `repo.work_directory_abs_path` for `validate_worktree_directory` |
| `crates/git_ui/src/worktree_picker.rs` | Same change as agent_panel |

## Backward Compatibility

- The new proto field is `optional`. If an older host doesn't send it, the client
  falls back to `work_directory_abs_path` (current behavior). No breakage.
- If a newer host sends it to an older client, protobuf silently ignores unknown
  fields. No breakage.
- `resolve_worktree_directory` and `validate_worktree_directory` are unchanged —
  only their inputs change. All existing tests for those functions continue to pass.