# Worktree Base Branch Selection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let users select an existing local or remote base branch before creating a detached worktree in single-repository projects.

**Architecture:** Extend the existing type-erased branch-picker builder in `git_ui_core` with select-modal parameters and callback data, then wire it to the concrete `git_ui::branch_picker::select_modal`. The worktree picker and sidebar pass the selected branch through the existing `CreateWorktree` action; the backend and protocol remain unchanged.

**Tech Stack:** Rust, GPUI, Zed `Picker`, existing `git_ui_core`/`git_ui`/`sidebar` crates.

## Global Constraints

- Preserve immediate creation from the default branch and current branch.
- Expose custom base selection only for single-repository projects.
- Reuse the existing `BranchList` select mode; do not duplicate branch search or filtering.
- Create detached worktrees through the existing service.
- Do not change Git, project, action, or protobuf contracts.
- Preserve existing remote fetch, retry, rollback, and error reporting.

---

### Task 1: Type-erased branch selection seam

**Files:**
- Modify: `crates/git_ui_core/src/git_ui_core.rs`
- Modify: `crates/git_ui/src/git_ui.rs`
- Modify: `crates/git_ui/src/branch_picker.rs`
- Test: `crates/git_ui/src/branch_picker.rs`

**Interfaces:**
- Consumes: existing `Branch`, `Workspace`, `Repository`, `GitPickerPopover`, and `branch_picker::select_modal`.
- Produces: a `git_ui_core` branch-selection callback type and `build_branch_selector(...) -> Entity<GitPickerPopover>` that accepts `selected_branch: Option<SharedString>` and calls back with the selected `Branch`.

- [ ] **Step 1: Add a failing select-mode callback test**

Use the existing `init_branch_list_test` branch fixtures to construct `BranchList::new_select`, dispatch `menu::Confirm`, and assert that the callback captured the selected local branch without performing checkout. Also assert the select picker emits dismissal after confirmation.

- [ ] **Step 2: Run the focused test and verify failure**

Run: `cargo test -p git_ui test_select_branch_callback`

Expected: FAIL because the callback-focused test or shared selector seam does not exist.

- [ ] **Step 3: Extend the core builder seam minimally**

Add a public callback alias equivalent to:

```rust
pub type SelectBranchCallback = Arc<dyn Fn(Branch, &mut Window, &mut App)>;
```

Add a second type-erased builder and matching setter/builder function for select-modal mode. It must accept the workspace, optional repository, optional selected branch, callback, window, and app, then return `Entity<GitPickerPopover>`. Keep the existing popover builder unchanged for current callers.

- [ ] **Step 4: Install the concrete selector builder**

In `git_ui::init`, install the new builder by constructing `branch_picker::select_modal` and wrapping it with `GitPickerPopover::new`. Reuse the callback alias from `git_ui_core` in `branch_picker.rs` instead of defining a competing public callback type.

- [ ] **Step 5: Run focused tests**

Run: `cargo test -p git_ui test_select_branch_callback`

Expected: PASS.

- [ ] **Step 6: Commit the seam**

```bash
git add crates/git_ui_core/src/git_ui_core.rs crates/git_ui/src/git_ui.rs crates/git_ui/src/branch_picker.rs
git commit -m "feat(git): expose branch selection picker"
```

### Task 2: Worktree picker branch selection flow

**Files:**
- Modify: `crates/git_ui_core/src/worktree_picker.rs`
- Test: `crates/git_ui_core/src/worktree_picker.rs`

**Interfaces:**
- Consumes: `build_branch_selector(...)`, `Branch`, `NewWorktreeBranchTarget`, and `worktree_service::handle_create_worktree`.
- Produces: `WorktreeEntry::ChooseBaseBranch` and a helper that opens branch selection with an optional normalized worktree name.

- [ ] **Step 1: Add failing worktree picker tests**

Extend the existing worktree picker test harness to verify:

```text
single repository + empty query => Choose Base Branch row exists
multiple repositories + empty query => Choose Base Branch row absent
named query + confirm => no creation before branch callback
local callback => ExistingBranch plus normalized worktree name
remote callback => RemoteBranch plus normalized worktree name
cancel => no creation
```

Capture the requested `CreateWorktree` value through a test branch-selector builder or a small pure conversion helper; assert observable action data rather than source text.

- [ ] **Step 2: Run focused tests and verify failure**

Run: `cargo test -p git_ui_core worktree_picker`

Expected: FAIL on missing custom-base entry and selection behavior.

- [ ] **Step 3: Add custom-base entry and branch conversion**

Add `WorktreeEntry::ChooseBaseBranch` for empty-query single-repository pickers. Add a focused conversion helper:

```rust
fn branch_target(branch: &Branch) -> NewWorktreeBranchTarget
```

For a remote branch, return `RemoteBranch` using `Branch::remote_name()` and the branch name without the remote prefix. For a local branch, return `ExistingBranch` using its local name.

- [ ] **Step 4: Open selector before named creation**

For `ChooseBaseBranch`, call the selector with `worktree_name: None`. For enabled `CreateNamed`, call it with `Some(name.clone())`. The selector callback upgrades the workspace and invokes `handle_create_worktree` with the captured name and converted target. Emit the worktree picker dismissal after opening the selector; selector cancellation must not invoke the callback.

Keep `CreateFromCurrentBranch` and `CreateFromDefaultBranch` immediate. For multi-repository projects, keep the existing named-query current-branch creation because custom selection is unavailable.

- [ ] **Step 5: Run focused tests**

Run: `cargo test -p git_ui_core worktree_picker`

Expected: PASS, including existing default/current tests.

- [ ] **Step 6: Commit the picker flow**

```bash
git add crates/git_ui_core/src/worktree_picker.rs
git commit -m "feat(git): select worktree base branch"
```

### Task 3: Sidebar parity and final verification

**Files:**
- Modify: `crates/sidebar/src/sidebar.rs`
- Test: `crates/sidebar/src/sidebar.rs` if an existing focused menu test seam is available; otherwise cover its pure target construction helper in `git_ui_core`.

**Interfaces:**
- Consumes: the new branch selector builder and existing `create_worktree_in_workspace` path.
- Produces: a single-repository `Choose Base Branch…` sidebar menu action; multi-repository behavior remains unchanged.

- [ ] **Step 1: Add a failing sidebar target test**

Assert the single-repository worktree submenu includes the custom-base action while a multi-repository submenu does not. If menu elements are not directly inspectable, extract only the target-list construction into a pure helper and test its returned variants.

- [ ] **Step 2: Run the focused test and verify failure**

Run: `cargo test -p sidebar worktree`

Expected: FAIL because the custom-base menu target is absent.

- [ ] **Step 3: Wire sidebar custom-base selection**

Add `Choose Base Branch…` beside the existing generated-name creation choices for single-repository projects. Open the existing branch selector and pass the chosen local or remote target to `create_worktree_in_workspace`. Do not add the item when multiple repositories are present.

- [ ] **Step 4: Run focused tests**

Run:

```bash
cargo test -p sidebar worktree
cargo test -p git_ui_core worktree_picker
cargo test -p git_ui test_select_branch_callback
cargo fmt --all -- --check
```

Expected: all commands PASS.

- [ ] **Step 5: Smoke test the real UI path**

Launch the application using the repository's normal development command. Open the worktree picker, type a name with spaces, confirm creation, choose a local branch, and verify the new detached worktree uses the normalized name and selected base. Repeat with a remote branch and verify fetch failure, if induced, uses the existing retry toast. Confirm Escape at the branch selector creates nothing.

- [ ] **Step 6: Update repository graph and commit**

Run: `graphify update .`

```bash
git add crates/sidebar/src/sidebar.rs graphify-out
git commit -m "feat(sidebar): choose worktree base branch"
```

Do not stage `graphify-out` if it remains intentionally untracked by repository convention; verify its update locally instead.
