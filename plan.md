# Thread Target Selector — Plan

## What's Been Done

### PR #49141 (`AI-34/thread-target-selector-ui`)

This PR adds three new UI elements to the agent panel, all gated behind `AgentV2FeatureFlag`:

1. **Thread Target Selector** — A "Start Thread In…" dropdown in the agent panel toolbar. Options are "Local Project" (default) and "New Worktree". Selecting "New Worktree" sets `self.thread_target` but doesn't create a worktree yet. The dropdown is hidden once a thread has had its first prompt submitted (the target cannot be changed after the agent starts running).

2. **Worktree Creation Status Banner** — A status bar below the toolbar. Shows a spinner with "Creating worktree…" or an error message with a warning icon. Currently scaffolding only (`WorktreeCreationStatus` has `#[allow(dead_code)]`).

3. **Worktree Branch Labels on History Rows** — When a thread was created in a worktree, its history row shows a git branch badge. This is now dead code (see "Dead code to clean up" below) since branch/worktree info for active workspaces will be derived from live git state in the sidebar instead.

### Commits on the branch

**Original PR work:**
- `3c33311c2f` — Add thread target selector UI + worktree labels in history
- `c16eddc51a` — Review fixes: persist thread target, restore-thread validation, model defaults, target validation, timestamps
- `32c0b19280` — Fix CI: remove unused import and non-existent agents_panel_dock field
- `4c39cc395d` — Fix test: only validate thread existence in DB for NativeAgent threads

**Code review fixes:**
- `c770822a6d` — Fix `is_via_collab` inconsistency in `render_thread_target_selector`
- `263110ad83` — Extract shared `format_relative_time` to eliminate duplication
- `430fbd00ea` — Add iteration guard to `set_selected_index`
- `02d5fde7f4` — Add missing `self.serialize(cx)` call in `set_thread_target`
- `f4044ee138` — Derive `Serialize`/`Deserialize` on `ThreadTarget` directly, eliminating `SerializedThreadTarget`
- `ae4867e459` — Validate thread target on deserialization (fall back to `LocalProject` if conditions no longer met)
- `3776b288c7` — Add integration test + TODO for `set_active` worktree creation guard

**Test fixes for CI:**
- `1b5961e0eb` — Initialize `GlobalFs` in the `set_active` test
- `d02f8a7e70` — Use `StubAgentServer` instead of triggering full agent infra in test

**Visual tests (added in this session):**
- `e2657d50ad` — Add visual tests for thread target selector, worktree status banner, and history branch labels
- `2670c153ac` — Add visual test for thread target selector dropdown open state

### Visual tests

Six screenshots are now captured in `target/visual_tests/`:

| Screenshot | What It Shows |
|---|---|
| `thread_target_selector_default.png` | Agent panel toolbar with "Local Project ▾" button |
| `thread_target_selector_open.png` | The dropdown open, showing "Start Thread In…" header, "Local Project" (selected), "New Worktree" |
| `thread_target_selector_new_worktree.png` | Toolbar with "New Worktree ▾" shown as the selected target |
| `worktree_creation_status_creating.png` | Banner below toolbar with spinner + "Creating worktree…" |
| `worktree_creation_status_error.png` | Banner with ⚠ warning icon + error message in yellow |
| `history_with_worktree_branch.png` | History view with entries showing git branch badges |

### Key bug fix: re-entrant window update

The visual tests were originally broken because `workspace_window.update(cx, ...)` was called **inside** `cx.update_window(workspace_window.into(), ...)`. In GPUI, `update_window` `.take()`s the window out of the window map while the closure runs, so a nested `update_window` on the same window finds `None` and silently fails (swallowed by `.log_err()`). The fix was to call `workspace_window.update(cx, ...)` directly instead of nesting it.

Note: the existing `run_agent_thread_view_test` (Test 5) has the same re-entrant bug — its `add_panel`/`open_panel` calls silently fail too, but the test "passes" because there are no baseline images to compare against. That's a pre-existing issue, not something we introduced.

### Key files

| File | What's in it |
|---|---|
| `crates/agent_ui/src/agent_panel.rs` | `AgentPanel` struct. `ThreadTarget` enum (~line 413), `WorktreeCreationStatus` enum (~line 443), `set_thread_target` (~line 1875), `render_thread_target_selector` (~line 2545), `render_worktree_creation_status` (~line 3044), test helpers (~line 3799). |
| `crates/agent_ui/src/agent_ui.rs` | Re-exports `ThreadTarget` and `WorktreeCreationStatus` (line 52). |
| `crates/agent_ui/src/acp/thread_history.rs` | History rendering. |
| `crates/zed/src/visual_test_runner.rs` | Visual test: `run_thread_target_selector_visual_tests` (~line 3124). Registration as Test 11 (~line 553). |

---

## PR 1: Create Worktree on First Prompt

When "New Worktree" is selected as the thread target and the user submits their first prompt, the system should create git worktrees for all git-enabled folders in the project, open the result as a new workspace, switch to it, and then submit the prompt there.

### 1. Route the first send through `AgentPanel` when target is `NewWorktree`

The send flow today is entirely inside `AcpThreadView`. The chain: user hits Enter → `MessageEditor` emits `MessageEditorEvent::Send` → `AcpThreadView` subscribes to it (line 375 of `active_thread.rs`) and calls `self.send()` (line 627) → `self.send_impl()` (line 692) → message goes to the agent. `AgentPanel` has zero visibility into any of this — it only has a `cx.observe()` on `AcpServerView` for re-rendering.

The send must NOT reach the agent when the target is `NewWorktree`, because the agent may run tool calls that depend on the working directory being the new worktree. Instead, `AcpThreadView` will branch in its `send()` method and emit an event upward so `AgentPanel` can run the worktree creation flow.

#### Plumbing `thread_target` down to `AcpThreadView`

Pass a `needs_worktree_creation: bool` (not the full `ThreadTarget` enum — `AcpThreadView` doesn't need to know about thread target semantics) through the existing creation chain:

1. **`AgentPanel::_external_thread()`** (line 2032 of `agent_panel.rs`) already receives and passes `initial_content: Option<AgentInitialContent>` to `AcpServerView::new()`. Add `needs_worktree_creation: bool` as a parameter alongside it.
2. **`AcpServerView::new()`** receives it and passes it through to `AcpThreadView::new()`.
3. **`AcpThreadView::new()`** stores it as a field: `needs_worktree_creation: bool`.

`AgentPanel` sets this to `true` when `self.thread_target == ThreadTarget::NewWorktree` and the thread is new (not resumed).

#### Branching in `AcpThreadView::send()`

Add a check at the top of `AcpThreadView::send()` (line 627), after the existing `is_loading_contents` early return but before any other logic:

```rust
if self.needs_worktree_creation {
    self.needs_worktree_creation = false; // one-shot: only the first send
    let contents = message_editor.read(cx).contents_as_blocks(cx);
    cx.emit(AcpThreadViewEvent::WorktreeCreationRequested { contents });
    return;
}
```

This is the same pattern as the existing branches in `send()` for "is editor empty?", "is generating?", and "/login" — a conditional early return. The message stays in the editor (we don't call `editor.clear()`), so it's preserved if worktree creation fails.

#### Making `AcpThreadView` an `EventEmitter`

`AcpThreadView` currently does not implement `EventEmitter`. Add:

```rust
pub enum AcpThreadViewEvent {
    WorktreeCreationRequested {
        contents: Vec<ContentBlock>,
    },
}

impl EventEmitter<AcpThreadViewEvent> for AcpThreadView {}
```

This follows the same pattern as `MessageEditor` → `MessageEditorEvent`. The event carries the message contents as `Vec<ContentBlock>` (the same type used by `AgentInitialContent::ContentBlock`), so the contents can be passed directly to the new workspace's agent panel.

#### `AgentPanel` subscribes to `AcpThreadView` events

In `AgentPanel::set_active_view()` (line 1744), where `_active_view_observation` is already set up for `AcpServerView`, add a subscription to the `AcpThreadView`:

```rust
ActiveView::AgentThread { server_view } => {
    // existing cx.observe on server_view...
    if let Some(thread_view) = server_view.read(cx).active_thread() {
        subscriptions.push(cx.subscribe(thread_view, |this, _view, event, cx| {
            match event {
                AcpThreadViewEvent::WorktreeCreationRequested { contents } => {
                    this.handle_worktree_creation_requested(contents.clone(), cx);
                }
            }
        }));
    }
}
```

Note: `AcpServerView` may create new `AcpThreadView` instances over its lifetime. The subscription needs to be refreshed when that happens. Since `AgentPanel` already observes `AcpServerView` (the observation fires whenever `AcpServerView` calls `cx.notify()`), the subscription can be re-established there if the active thread view has changed.

#### `AgentPanel::handle_worktree_creation_requested()`

This new method on `AgentPanel` receives the message contents and kicks off the worktree creation flow (steps 2–4 below). It stores the contents, sets `self.worktree_creation_status = Some(WorktreeCreationStatus::Creating)`, and spawns the async worktree creation task.

### 2. Create git worktrees for all repos in the project

The project may have multiple root folders, each with its own git repository. We create a worktree in **every** git-enabled repo, all sharing the same branch name.

- **Name:** Generate `agent-` + 8 random lowercase alphanumeric characters (e.g. `agent-k7m2xq9b`). This name is used for **both** the branch name (`-b agent-k7m2xq9b`) and the subdirectory name within the worktree directory. The same name is shared across all repos in the project.
- **Worktree directory:** Use the existing `git.worktree_directory` setting (default: `"../worktrees"`) to determine where worktrees are placed. This is resolved via `validate_worktree_directory()` and `resolve_worktree_directory()` in `crates/git/src/repository.rs`. The resolver already handles multi-repo collision avoidance: when the resolved directory is outside the project root, it appends the repo's directory name. For example, with a project containing `~/code/zed/` and `~/code/ex/` and branch name `agent-olivetti`:
  - `~/code/zed/` → resolved directory `~/code/worktrees/zed/` → worktree at `~/code/worktrees/zed/agent-k7m2xq9b`
  - `~/code/ex/` → resolved directory `~/code/worktrees/ex/` → worktree at `~/code/worktrees/ex/agent-k7m2xq9b`
- **API:** Call `Repository::create_worktree(name, directory, None)` from `crates/project/src/git_store.rs` for each repository. This is the project-level wrapper that dispatches to `GitRepository::create_worktree` for local repos.
- **Non-git folders:** If any root folder in the project is not backed by a git repository, include it in the new workspace as-is (pointing at the same original folder). This is intentional — the agent will edit the same files as the original workspace for those folders. Show a **warning toast** to the user explaining that no worktree was created for those folders because they are not git repositories.
- While creation is in progress, show the `WorktreeCreationStatus::Creating` banner (spinner + "Creating worktree…").
- **Failure and rollback:** If creation fails for any repo, clean up any worktrees that were successfully created (roll back), show `WorktreeCreationStatus::Error(message)`, and do **not** submit the prompt. Leave the user in the original workspace with their prompt text intact in the editor. Rollback uses `GitRepository::remove_worktree(path, force: true)` from the `GitRepository` trait in `crates/git/src/repository.rs` (line 736). This already exists and is implemented on both `RealGitRepository` (calls `git worktree remove`) and `FakeGitRepository`. There is no project-level wrapper on `Repository` in `git_store.rs` — we'll need to add a `Repository::remove_worktree()` method following the same pattern as the existing `Repository::create_worktree()`.
- If the user navigates away from the agent panel (to history, configuration, etc.) while creation is in progress, the creation should continue in the background and complete the workspace switch when done.

### 3. Open the worktrees as a new workspace

- Collect all the new worktree paths (one per git-enabled repo) plus any non-git folders from the original project.
- Use `MultiWorkspace::open_project(paths, window, cx)` to open them as a new workspace. Its signature is `pub fn open_project(&mut self, paths: Vec<PathBuf>, window: &mut Window, cx: &mut Context<Self>) -> Task<Result<()>>` (line 591 of `multi_workspace.rs`).
- Under the hood, when multi-workspace is enabled (gated on `AgentV2FeatureFlag`), it delegates to `Workspace::open_workspace_for_paths(true, paths, window, cx)`, which calls the free function `open_paths`. That function either finds an existing workspace containing those paths (and activates it) or creates a new one via `Workspace::new_local(...)` in the same OS window. Either way, the window is activated and the workspace is made current.
- The return type is `Task<Result<()>>` — it does **not** return a handle to the new workspace. **Do not use `MultiWorkspace::workspace()`** to find the new workspace after awaiting — it returns whatever is at `active_workspace_index`, which is subject to a race condition if the user interacts with the sidebar between the `open_project` completing and our code running. Instead, either modify `open_workspace_for_paths` to propagate the `Entity<Workspace>` (it's available inside `Workspace::new_local`), or find the workspace by matching paths after the fact.
- The new workspace will appear in the multi-workspace sidebar automatically.

### 4. Switch to the new workspace and submit the prompt

- `MultiWorkspace::open_project` already handles making the new workspace active and focused. After awaiting it, the new workspace is the active one.
- Open the agent panel in the new workspace and submit the prompt using `AgentInitialContent::ContentBlock { blocks: contents, auto_submit: true }`. This is the existing mechanism: `AgentInitialContent` is an enum defined in `agent_ui.rs` (line 284) with a `ContentBlock { blocks: Vec<ContentBlock>, auto_submit: bool }` variant. When `auto_submit` is true, `AcpThreadView::new()` calls `this.send(window, cx)` at the end of construction (line 448 of `active_thread.rs`), which sends the message immediately.
- The existing pattern for this is in `agent_panel.rs` (line 310): get the `AgentPanel` via `workspace.panel::<AgentPanel>(cx)`, call `workspace.focus_panel::<AgentPanel>(window, cx)`, then call `panel.update(cx, |panel, cx| panel.external_thread(None, None, Some(initial_content), window, cx))`. This creates a new thread with the content pre-filled and auto-submitted.
- The `contents` passed in the `WorktreeCreationRequested` event are already `Vec<ContentBlock>`, which is the same type `AgentInitialContent::ContentBlock` expects — no conversion needed.
- Clear the creation status banner on the original workspace. The async task is spawned via `cx.spawn_in(window, async move |this, cx| ...)` where `this: WeakEntity<AgentPanel>` refers to the **original** `AgentPanel` entity (not whatever is currently active). Use `this.update_in(cx, ...)` to clear the banner after the workspace switch completes. The original workspace's `WeakEntity<Workspace>` is also available via `self.workspace.clone()` captured before spawning.

### Sequencing

The full flow when the user hits Enter with "New Worktree" selected:

1. `AcpThreadView::send()` hits the `needs_worktree_creation` branch, emits `AcpThreadViewEvent::WorktreeCreationRequested { contents }`, and returns early (message stays in the editor)
2. `AgentPanel` receives the event via its subscription, calls `handle_worktree_creation_requested(contents)`, which shows the "Creating worktree…" spinner banner
3. Enumerate all visible root folders in the project (Zed worktrees via `project.visible_worktrees(cx)`). For each, check whether it has an associated git repository. Classify into two lists: git-enabled folders (with their `Entity<Repository>`) and non-git folders (just paths).
4. For each git-enabled folder: resolve the worktree directory via `validate_worktree_directory`, call `Repository::create_worktree("agent-k7m2xq9b", resolved_directory, None)` (async, all in parallel)
5. On success of all worktree creations: collect all new worktree paths + non-git folder paths (non-git folders are included as-is, pointing at the original location — this is intentional)
6. `MultiWorkspace::open_project(all_paths, window, cx)` — opens the new workspace, switches to it
7. Show warning toast if there were non-git folders
8. Open agent panel in new workspace, submit the prompt via `AgentInitialContent { auto_submit: true }`
9. Clear the creation status banner on the original workspace
10. On failure at step 4: roll back any successfully created worktrees, show error banner, leave user in original workspace with their prompt text still in the editor (it was never cleared because `send()` returned early — see "Editor text preservation" below)

### Editor text preservation on failure

The user's typed message is naturally preserved when `send()` returns early. In the normal send flow, `message_editor.clear()` is called **asynchronously inside `send_impl()`** (around line 733 of `active_thread.rs`), after `contents.await` succeeds. Since our `needs_worktree_creation` branch returns early from `send()` before reaching `send_impl()`, the editor is never cleared. No explicit save/restore of the editor text is needed.

### Panel survival during background creation

`AgentPanel` implements the `Panel` trait (line 2107 of `agent_panel.rs`). Panels are registered on the `Workspace` via `workspace.add_panel()` and stored as `Entity<AgentPanel>` for the lifetime of the workspace. When the user navigates within the agent panel (to history, configuration, etc.), only the `active_view` field changes — the `AgentPanel` entity itself is never dropped. When the user switches to a different panel entirely (e.g. project panel), the `AgentPanel` is deactivated (`set_active(false)`) but the entity remains alive. Any task spawned via `cx.spawn()` on the `AgentPanel`'s context will continue running as long as the entity exists, which is as long as the workspace exists. So the worktree creation task stored on `AgentPanel` will survive navigation and panel switching.

### Thread target lifecycle

- The `thread_target` dropdown is visible only before the first prompt is submitted. Once a thread is active, the dropdown is hidden — the target cannot be changed after the agent starts running.
- The original workspace's `thread_target` stays as `NewWorktree` after the switch. This doesn't matter because the dropdown is hidden once a thread is running.
- The new workspace's agent panel gets a fresh `AgentPanel` with its own default `thread_target = LocalProject`. This is correct: the new workspace *is* local to its worktree.

---

## PR 2: Sidebar Hierarchy

Worktree-based workspaces should appear **nested under** the original project in the multi-workspace sidebar, rather than as flat peers. This makes the worktree creation flow immediately useful — when the agent creates a new worktree workspace, you can see it grouped under the parent project with its worktree name visible.

### Design principles

**Single source of truth:** Filesystem worktree discovery drives the hierarchy exclusively. Whether a worktree is created via Zed's agent UI, the worktree picker, or `git worktree add` in a terminal, the sidebar only ever updates by detecting filesystem changes through the worktree scanner. Zed's "create worktree" code path is just a trigger that writes to the filesystem — it never directly updates the sidebar grouping or inserts anything into the hierarchy. This guarantees no duplicates and no divergence between what's on disk and what's shown in the UI.

**No new persistent storage.** The hierarchy is derived entirely from git metadata that Zed already caches in memory. The worktree scanner already detects whether `.git` is a file (git worktree) vs a directory (main repo/regular clone). Each `LocalRepositoryEntry` tracks a `common_dir_abs_path` that points to the main repo's `.git/` directory. Two workspaces whose repos share the same `common_dir_abs_path` are related (same underlying git repo). The workspace where `.git` is a directory is the "parent" (main checkout). Workspaces where `.git` is a file are "children" (worktrees).

**Performance:** The sidebar maintains a **pre-computed in-memory grouping** rather than comparing paths on every render. The grouping is rebuilt reactively when workspaces are added/removed, git repos are discovered/removed (`GitStoreEvent`), or a workspace's root folders change. The render path just traverses the pre-built grouping — no path comparisons, no filesystem reads.

### Implementation steps

#### 1. Expose `common_dir_abs_path` on `RepositorySnapshot`

Two fields need to be exposed: `common_dir_abs_path` and `dot_git_abs_path`. Both already flow from `LocalRepositoryEntry` → `UpdatedGitRepository`, but `GitStore::update_repositories_from_worktree` currently **discards** both (variables prefixed with `_`). The chain:

| Layer | Has `common_dir_abs_path`? | Has `dot_git_abs_path`? | Status |
|---|---|---|---|
| `LocalRepositoryEntry` (worktree crate, private) | ✅ | ✅ | Source of truth |
| `UpdatedGitRepository` (worktree crate, public) | ✅ | ✅ | Flows via `changed_repos` |
| `GitStore::update_repositories_from_worktree` | ❌ (discarded) | ❌ (discarded) | Needs to store both |
| `Repository` / `RepositorySnapshot` | ❌ | ❌ | Needs new fields |

Both fields are needed because comparing them is how we distinguish parent from child:
- **Main checkout (parent):** `dot_git_abs_path == common_dir_abs_path` (both point to the repo's own `.git/` directory)
- **Git worktree (child):** `dot_git_abs_path != common_dir_abs_path` (`dot_git_abs_path` is the local `.git` file, `common_dir_abs_path` points to the main repo's `.git/`)

These fields are `Option` because they come from local filesystem state. They are always `Some` for local repositories (the data is always available from `LocalRepositoryEntry`) and always `None` for remote/collab repositories (these paths don't exist on the client side). There are no local edge cases where they'd be `None`.

Changes needed:
- Add `pub common_dir_abs_path: Option<Arc<Path>>` and `pub dot_git_abs_path: Option<Arc<Path>>` to `RepositorySnapshot`.
- In `GitStore::update_repositories_from_worktree`, pass both through to `Repository` and store them on the snapshot. Currently both are destructured but discarded (prefixed with `_`) at line ~1494 of `git_store.rs`.
- Update `RepositorySnapshot::empty()` and `compute_snapshot()` to carry both fields forward.

#### 2. Build the workspace grouping in the sidebar

The sidebar delegate (`WorkspacePickerDelegate` in `crates/sidebar/src/sidebar.rs`) currently stores a flat `Vec<SidebarEntry>`. It needs a pre-computed grouping that maps parent workspaces to their worktree children.

**Terminology:** A root folder's **worktree name** is the last component of its filesystem path (e.g. if the worktree path is `~/code/worktrees/zed/foo/`, the worktree name is `foo`). This concept is only meaningful for root folders that are both git-enabled *and* are git worktrees (i.e. `dot_git_abs_path != common_dir_abs_path`). A workspace's **worktree name** is the shared worktree name across all its git-enabled worktree roots — but only if they all have the same worktree name. If they differ, the workspace has no worktree name.

**Parent-child matching rule:** A workspace B is nested under workspace A if and only if:
1. Workspace A has at least one git-enabled root folder.
2. Every git-enabled root in B is a git worktree (i.e. `dot_git_abs_path != common_dir_abs_path`) of a unique git-enabled root in A — each child root's `common_dir_abs_path` matches a unique parent root's `common_dir_abs_path`.
3. B has at least one such matching root.
4. Non-git root folders are ignored for the purpose of this matching.

Note: B may have **fewer** git-enabled roots than A. This is the common case — `create_worktree` typically creates a single worktree and opens it as a single-root workspace, even when the parent has multiple roots. The matching allows subsets: if A has roots for repos X and Y, and B has a worktree only of repo X, B still nests under A.

If these conditions aren't met (e.g. none of B's roots are worktrees of any root in A, or two of B's roots are worktrees of the same repo in A), the workspaces are unrelated and both appear at the top level.

The grouping is built from each workspace's project repositories:
- For each workspace, read its project's `RepositorySnapshot`s and collect their `common_dir_abs_path` and `dot_git_abs_path` values.
- Classify each workspace's git-enabled roots as either main checkouts (`dot_git_abs_path == common_dir_abs_path`) or git worktrees (`dot_git_abs_path != common_dir_abs_path`).
- Apply the parent-child matching rule above to determine nesting.
- Workspaces with no git repos, or that don't satisfy the matching rule against any other workspace, appear ungrouped at the top level.

This grouping is stored as a pre-computed data structure (e.g. `HashMap<Arc<Path>, Vec<usize>>` mapping `common_dir_abs_path` → child workspace indices) and rebuilt when:
- A workspace is added or removed from `MultiWorkspace`
- A git repo is discovered or removed (`GitStoreEvent`)
- A workspace's root folders change

Workspace added/removed and root folder changes already fire notifications the sidebar observes (via `cx.observe_in` on `MultiWorkspace` and `ProjectEvent::WorktreeAdded/Removed`). However, **git repo discovered/removed is not currently observed by the sidebar** — `GitStoreEvent::RepositoryAdded` and `GitStoreEvent::RepositoryRemoved` are emitted by `GitStore` but the sidebar doesn't subscribe to them. We'll need to either add new `Project::Event` variants (e.g. `GitRepositoryAdded`, `GitRepositoryRemoved`) that forward from `GitStoreEvent`, or have the sidebar directly subscribe to each project's `GitStore` entity (accessible via `project.git_store()`).

#### 3. Render the hierarchy

Currently `rebuild_entries` produces a flat list:
```
Separator("Active Workspaces")
WorkspaceThread(workspace_0)
WorkspaceThread(workspace_1)
WorkspaceThread(workspace_2)
```

With grouping, it becomes:
```
Separator("Active Workspaces")
WorkspaceThread(workspace_0)           ← parent (main checkout)
  WorkspaceThread(workspace_1)         ← child worktree, indented, shows worktree name
  WorkspaceThread(workspace_2)         ← child worktree, indented, shows worktree name
WorkspaceThread(workspace_3)           ← unrelated workspace, top-level
```

Changes needed:
- Child `WorkspaceThread` entries are indented under their parent. The exact visual treatment (indentation amount, tree lines, etc.) will be based on a design screenshot provided separately.
- Child entries show the **worktree name** — the last folder component of the worktree path (e.g. if the path is `~/code/worktrees/zed/agent-k7m2xq9b/`, show `agent-k7m2xq9b`). This is derived from the filesystem path, not from git branch state. The worktree name is only shown if all git-enabled worktree roots in the workspace share the same worktree name; if they differ, fall back to the normal workspace display name.
- Parent entries render normally (as they do today).
- Workspaces with no parent/child relationship render as they do today (flat, top-level).

### Testing strategy

Both integration tests and visual tests are needed for the new functionality. Integration tests (like the existing `set_active` test) cover the worktree creation flow, event plumbing, and workspace switching logic. Visual tests cover the sidebar hierarchy rendering and the creation status banner.

### Feature flag

The worktree creation flow and sidebar hierarchy are gated behind the same `AgentV2FeatureFlag` used by the existing thread target selector UI. No new feature flags are needed.

### Emergent behavior

Because the hierarchy is derived from git state rather than stored explicitly:
- Worktrees created by Zed's agent flow automatically appear nested.
- Worktrees created manually (via the worktree picker, `git worktree add` in a terminal, etc.) also auto-discover and nest.
- Closing/removing a worktree workspace removes it from the hierarchy automatically.
- Closing the parent workspace causes its former children to appear as top-level entries (there's nothing to match against anymore). Re-opening the parent workspace re-establishes the nesting.