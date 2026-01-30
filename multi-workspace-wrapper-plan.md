# MultiWorkspace Implementation Plan

## Current State

### Committed
1. **MultiWorkspace wrapper** (`4139046bd8`)
   - `MultiWorkspace` struct wraps `Entity<Workspace>` as window root
   - Changed entry points to return `WindowHandle<MultiWorkspace>`

2. **Test/crate updates** (`109eeaaa6e`)
   - Updated all `downcast::<Workspace>()` calls to use `MultiWorkspace`
   - Fixed tests in collab, debugger_ui, file_finder, vim, etc.

### Uncommitted (Working)
- **Multiple workspaces**: `Vec<Entity<Workspace>>` with `active_workspace_index`
- **Workspace switching**: `NextWorkspaceInWindow`, `PreviousWorkspaceInWindow` actions
- **Sidebar UI**: Full-height workspace list with "+" button, `ToggleWorkspaceSwitcher` action
- **Delegation methods**: `panel()`, `toggle_modal()`, `start_debug_session()`, etc.
- **Window chrome**: Moved `client_side_decorations` from Workspace to MultiWorkspace

### Architecture Decision
We decided to keep most UI concerns in Workspace (titlebar, status bar, modal layer, toast layer). Only `client_side_decorations` moved to MultiWorkspace. Each workspace is self-contained with its own UI chrome.

---

## Path to Shipping

### Phase 1: Code Review
**Owner: User**
- Review uncommitted changes
- Identify any architectural issues

### Phase 2: Replace `replace_root` with `replace_active_workspace`
**Problem:** Several places use `window.replace_root()` to swap out the entire window contents when opening a new project. With multi-workspace, this is too aggressive - it destroys the `MultiWorkspace` and any other workspaces that might have agents running or other important state.

**Solution:**
1. Add `MultiWorkspace::replace_active_workspace(workspace: Entity<Workspace>, ...)` method
   - Keeps the `MultiWorkspace` intact
   - Replaces only the active workspace slot (preserves ordering)
   - Other workspaces (with their agents, state, etc.) remain alive

2. Migrate call sites from `replace_root` pattern to `replace_active_workspace`

**Known locations (code changes):**
- [ ] `crates/workspace/src/workspace.rs` - `open_remote_project_inner` (line ~8865)
  - Currently does `window.replace_root()` to create new MultiWorkspace+Workspace for remote project
  - Should instead call `multi_workspace.replace_active_workspace()`
  - This is the central fix - other call sites go through here via `open_remote_project`

**Manual QA list (features using `open_remote_project` with `replace_window`):**
After fixing `open_remote_project_inner`, manually test these features with multiple workspaces open:
- [ ] Reconnect to disconnected SSH project (`disconnected_overlay.rs` line ~102)
- [ ] Open WSL project (`recent_projects.rs` line ~187)
- [ ] Open SSH project from recent projects picker (`recent_projects.rs` line ~258)
- [ ] Open remote workspace from recent projects picker (`recent_projects.rs` line ~637)
- [ ] Open project from remote servers modal (`remote_servers.rs` line ~1359)
- [ ] Open WSL distro from WSL picker (`wsl_picker.rs` line ~251)
- [ ] Verify other workspaces (especially those with running agents) are preserved after each operation

### Tech Debt (not blocking, clean up later)
- `crates/rules_library/src/rules_library.rs` (line ~967) - Iterates through all windows looking for an agent panel to show auth UI. This pattern is fundamentally broken - authentication should be handled globally, not by hunting for a specific panel. Multi-workspace makes this worse (only checks active workspace per window) but the real fix is architectural.
- Copilot status should be moved to a global rather than being workspace-specific.

### Phase 3: UI Polish
**Owner: User (clickthrough) + Claude (implementation)**

**Known issues:**
- [ ] Add button in workspace titlebar to open the sidebar
- [ ] Sidebar workspace items should show agent running state (if workspace has active agent)
- [ ] Fill sidebar with recent projects (not just currently loaded workspaces)
- [ ] Focus swapping and keyboard navigability
  - Switching workspaces should move focus appropriately
  - Sidebar should be keyboard navigable (up/down/enter)
  - Escape to close sidebar?

**Workspace lifecycle/memory management:**
- Keep workspace in memory if:
  - AI agent is currently running in it, OR
  - It's one of the last 3 workspaces opened
- Otherwise: drop from memory, load on-demand when clicked
- Sidebar shows both loaded workspaces AND recent projects (which load on click)

### Phase 4: Serialization
**Approach:**
- Only serialize the currently active workspace (existing pattern works)
- Other workspaces in the sidebar are just recent projects - lazy load when clicked
- No need for new "Window ID" concept yet - keep it simple



---

## Key Files

**MultiWorkspace core:**
- `crates/workspace/src/workspace.rs` - MultiWorkspace struct and Render impl

**Delegation:**
- `crates/workspace/src/tasks.rs` - start_debug_session delegation

**Entry points:**
- `crates/zed/src/zed.rs` - workspace creation
- `crates/zed/src/zed/open_listener.rs` - URL handling

**Persistence (future):**
- `crates/workspace/src/persistence/` - serialization

---

## Verification

1. `cargo build -p zed` passes
2. Manual testing:
   - Open Zed
   - Toggle workspace switcher sidebar
   - Create new workspace with "+" button
   - Switch between workspaces
   - Verify each workspace has independent state (different files, panels, etc.)
3. `cargo nextest run -p workspace` passes
