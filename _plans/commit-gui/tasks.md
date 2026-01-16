# Git Commit History Panel - Tasks

## CURRENT ISSUE
The "Recent Commits" panel is STILL showing at the TOP even though it should be removed.
Need to find where `render_commit_history_panel` is being called and remove it completely.

## Completed Tasks
- [x] Backend API - `branch_history_paginated` added to GitRepository trait
- [x] Backend API - Implemented for RealGitRepository
- [x] Backend API - Implemented for FakeGitRepository (test stub)
- [x] Backend API - Added to GitStore wrapper
- [x] Settings - Added `show_commit_history`, `commit_history_height`, `commit_history_page_size`
- [x] Settings - Added defaults to default.json
- [x] Core UI - Added CommitHistoryState struct
- [x] Core UI - Added fields to GitPanel
- [x] Core UI - Initialize commit history on repo change
- [x] Modified render_previous_commit to show multiple commits

## Remaining Tasks
- [ ] **CRITICAL: Remove render_commit_history_panel from render chain**
  - Find where it's still being called
  - Remove the call completely
  - The "Recent Commits" section at top should NOT exist

- [ ] Verify render_previous_commit is the ONLY place showing commits
  - Should show at BOTTOM of git panel
  - HEAD commit first with undo button
  - Additional commits below with same tooltip styling

- [ ] Test hover tooltips work correctly
  - Should show full commit message like existing single commit does

- [ ] Test scrolling when many commits
  - Container should be scrollable with max height

- [ ] Clean up unused code
  - render_commit_history_panel function (can be deleted if not used)
  - render_commit_history_entry function
  - DraggedCommitHistoryResize struct
  - Related unused fields

## Files Changed
1. `crates/git/src/repository.rs` - BranchHistory struct, branch_history_paginated
2. `crates/project/src/git_store.rs` - Wrapper methods
3. `crates/git_ui/src/git_panel.rs` - Main UI changes
4. `crates/git_ui/src/git_panel_settings.rs` - Settings
5. `crates/settings/src/settings_content.rs` - Settings schema
6. `assets/settings/default.json` - Default values
7. `crates/fs/src/fake_git_repo.rs` - Test stub
