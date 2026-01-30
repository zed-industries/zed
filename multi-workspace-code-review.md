# Multi-Workspace Code Review Progress

## Review Criteria

### Core Problem Pattern
Code that stores `WindowHandle<MultiWorkspace>` and later calls `multi_workspace.workspace()` across async boundaries or after user interaction. This dynamically resolves to the *currently active* workspace, which may have changed since the user's original intent was captured.

### Fix Pattern
**Capture workspace at intent time**: Store `WeakEntity<Workspace>` instead of `WindowHandle<MultiWorkspace>` when the user initiates an action. Use `workspace.downgrade()` to capture it early, before any async boundaries.

### Questions to Ask
- Is this capturing user intent that should be stable? → Capture `WeakEntity<Workspace>` early
- Is this replacing window contents? → Add to Phase 2 QA list
- Is this opening a new window? → Probably fine, no existing workspaces to preserve
- Is this a window-level feature that aggregates or iterates? → Think through multi-workspace semantics

---

## Files Reviewed

### Core Changes
- [x] `crates/workspace/src/workspace.rs` - MultiWorkspace struct, delegation methods
- [x] `crates/zed/src/zed.rs` - workspace creation, quit handling
- [x] `crates/zed/src/zed/open_listener.rs` - URL handling
- [x] `crates/zed/src/main.rs` - entry point changes

### Recent Projects / Remote
- [x] `crates/recent_projects/src/recent_projects.rs`
- [x] `crates/recent_projects/src/disconnected_overlay.rs`
- [x] `crates/recent_projects/src/remote_connections.rs`
- [x] `crates/recent_projects/src/remote_servers.rs`
- [x] `crates/recent_projects/src/wsl_picker.rs`

### UI Components
- [x] `crates/settings_ui/src/settings_ui.rs`
- [x] `crates/settings_ui/src/pages/edit_prediction_provider_setup.rs`
- [x] `crates/collab_ui/src/collab_panel.rs`
- [x] `crates/git_ui/src/file_diff_view.rs`
- [x] `crates/git_ui/src/worktree_picker.rs`
- [x] `crates/miniprofiler_ui/src/miniprofiler_ui.rs`
- [x] `crates/settings_profile_selector/src/settings_profile_selector.rs`

### Other
- [x] `crates/rules_library/src/rules_library.rs`
- [x] `crates/journal/src/journal.rs`
- [x] `crates/workspace/src/notifications.rs`
- [x] `crates/dev_container/src/devcontainer_api.rs`
- [x] `crates/vim/src/state.rs`

### Tests (mechanical updates - spot checked)
- [x] `crates/collab/src/tests/test_server.rs`
- [x] `crates/debugger_ui/src/tests.rs`
- [x] `crates/file_finder/src/file_finder_tests.rs`

---

## Findings

### 🔴 Issues Found (Need Fix)

None remaining - see "Fixed in This Session" below.

---

### 🟡 Phase 2 QA List (replace_root / replace_window patterns)

These use `window.replace_root()` which destroys the MultiWorkspace. After Phase 2 adds `replace_active_workspace`, these need migration:

From plan document:
- [ ] `crates/workspace/src/workspace.rs` - `open_remote_project_inner` (uses replace_root)
- [ ] Reconnect to disconnected SSH project (`disconnected_overlay.rs`)
- [ ] Open WSL project (`recent_projects.rs` line ~187)
- [ ] Open SSH project from recent projects picker (`recent_projects.rs` line ~258)
- [ ] Open remote workspace from recent projects picker (`recent_projects.rs` line ~637)
- [ ] Open project from remote servers modal (`remote_servers.rs`)
- [ ] Open WSL distro from WSL picker (`wsl_picker.rs`)

New findings:
- [ ] `crates/workspace/src/workspace.rs` - `new_local` around line ~1754 - uses `replace_root` when reusing existing window
- [ ] `crates/git_ui/src/worktree_picker.rs` - `open_remote_worktree` creates new MultiWorkspace in new window (OK for new window, but pattern should be reviewed)
- [ ] `crates/recent_projects/src/remote_connections.rs` - `open_remote_project` creates new MultiWorkspace (needs review)

---

### 🟢 Reviewed & OK

#### `crates/git_ui/src/file_diff_view.rs`
**Good fix!** Changed `FileDiffView::open` to take `WeakEntity<Workspace>` instead of `&Workspace`. The caller in `open_listener.rs` now captures the workspace early:
```rust
let workspace_weak = workspace.read_with(cx, |multi_workspace, _cx| {
    multi_workspace.workspace().downgrade()
})?;
```

#### `crates/git_ui/src/worktree_picker.rs`
**Good fix!** Captures workspace at intent time:
```rust
let workspace: WeakEntity<Workspace> =
    workspace_window.update(cx, |mw, _, _| mw.workspace().downgrade())?;
```
Then uses this captured workspace throughout the async flow for showing/dismissing modals.

#### `crates/miniprofiler_ui/src/miniprofiler_ui.rs`
**Good fix!** Changed from storing `WindowHandle<Workspace>` to `WeakEntity<Workspace>`:
```rust
workspace: Option<WeakEntity<Workspace>>,
```
Captures workspace handle at registration time via `cx.entity().downgrade()`.

#### `crates/settings_ui/src/settings_ui.rs`
**OK** - The settings window stores `WindowHandle<MultiWorkspace>` which is appropriate since it needs to interact with a specific window (not workspace). The iteration over workspaces to collect projects is correct - it properly iterates over all workspaces in all windows.

#### `crates/workspace/src/notifications.rs`
**OK** - The app notification code iterates over all windows and shows notifications in all workspaces. This is the correct behavior for app-wide notifications.

#### `crates/vim/src/state.rs`
**OK** - The `register_workspace` call iterates over all windows to register with all workspaces. This is initialization code that runs when vim mode changes, not user-intent-driven.

#### `crates/zed/src/zed.rs` - `quit` function
**OK** - The quit handler iterates all workspaces to call `prepare_to_close`. It correctly checks each workspace's active state at quit time.

#### `crates/rules_library/src/rules_library.rs`
**Tech Debt (documented in plan)** - Iterates windows looking for agent panel. Fundamentally broken pattern but not made worse by this change.

---

### ✅ Fixed in This Session

#### 1. `crates/zed/src/main.rs` - LSP store iteration (line ~509)
**Was**: Only collected LSP stores from the *active* workspace in each window.
**Fixed**: Now iterates over ALL workspaces in each window using `flat_map` over `multi_workspace.workspaces()`.

#### 2. `crates/dev_container/` - Major refactor
**Was**: Functions dynamically resolved workspace via `project_directory(cx)` helper, which could return wrong project if user switched workspaces during operation. Additionally, each internal function (`devcontainer_up`, `read_devcontainer_configuration`, `apply_dev_container_template`) independently called `ensure_devcontainer_cli()`.

**Fixed**: 
1. **Created `DevContainerContext` struct** that captures `project_directory`, `use_podman`, and `node_runtime` at intent time
2. **Added `DevContainerContext::from_workspace()` constructor** for easy creation from workspace
3. **Changed public API** (`start_dev_container`, `read_devcontainer_configuration`, `apply_dev_container_template`) to take `&DevContainerContext` instead of `&mut AsyncWindowContext`
4. **Created `DevContainerCli` struct** to hold CLI path info (`path: PathBuf`, `found_in_path: bool`)
5. **Optimized CLI resolution**: `ensure_devcontainer_cli()` is now called once at the public API level and the result is passed to internal functions, rather than each function resolving it independently
6. **Added `dev_container::use_podman(cx)` helper function**

**Files updated:**
- `crates/dev_container/src/lib.rs` - Added `DevContainerContext` struct, `from_workspace()`, `use_podman()` helper, updated `dispatch_apply_templates`
- `crates/dev_container/src/devcontainer_api.rs` - Added `DevContainerCli` struct, made `ensure_devcontainer_cli` pub(crate), updated all functions to take CLI as parameter
- `crates/recent_projects/src/recent_projects.rs` - Updated `OpenDevContainer` handler to use `DevContainerContext`
- `crates/recent_projects/src/remote_servers.rs` - Updated `open_dev_container` method to use `DevContainerContext`

---

### 📝 Notes / Tech Debt

From plan document:
- `crates/rules_library/src/rules_library.rs` (line ~967) - Iterates through all windows looking for an agent panel to show auth UI. Fundamentally broken pattern.
- Copilot status should be moved to a global rather than being workspace-specific.

New notes:
- **WorkspaceStore.workspaces now stores `AnyWindowHandle`** instead of `WindowHandle<Workspace>`. This is correct - it allows multiple workspaces per window while still tracking windows.
- **`remote_connections.rs`** - Deferred to Phase 2 due to `replace_root` interaction. The workspace capture issue exists but needs to be addressed alongside the `replace_active_workspace` work.
- **`journal.rs`** - No fix needed. New windows have a single workspace, so dynamic resolution is safe.
- **URL handlers in main.rs** - Most don't need fixing because they resolve workspace immediately after `get_any_active_workspace()`, before any user-interruptible async work.

---

## Summary

**Status**: Review Complete - Fixes Applied ✅

**Critical Issues**: 0

**Medium Issues**: 0 (2 fixed, others deferred or deemed safe)

**Phase 2 Blockers**: Several `replace_root` patterns identified, already documented in plan.

**Good Patterns Found**: 
- `file_diff_view.rs` - correctly changed to `WeakEntity<Workspace>` 
- `worktree_picker.rs` - correctly captures workspace early
- `miniprofiler_ui.rs` - correctly stores `WeakEntity<Workspace>`

**Files Modified in This Session**:
1. `crates/zed/src/main.rs` - Fix LSP store iteration to include all workspaces
2. `crates/dev_container/src/devcontainer_api.rs` - Added `DevContainerCli` struct, made `ensure_devcontainer_cli` pub(crate), updated functions to take CLI parameter, optimized to call CLI resolution once
3. `crates/dev_container/src/lib.rs` - Added `DevContainerContext` struct with `from_workspace()`, added `use_podman()` helper, updated `dispatch_apply_templates` to call `ensure_devcontainer_cli` once
4. `crates/recent_projects/src/recent_projects.rs` - Updated `OpenDevContainer` handler to use `DevContainerContext`
5. `crates/recent_projects/src/remote_servers.rs` - Updated `open_dev_container` method to use `DevContainerContext`

**Next Steps**:
1. Phase 2 work for `replace_root` patterns (see plan document)
2. Address `remote_connections.rs` workspace capture as part of Phase 2