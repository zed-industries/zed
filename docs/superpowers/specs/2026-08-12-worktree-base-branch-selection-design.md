# Worktree Base Branch Selection Design

## Goal

Let users choose a local or remote base branch when creating a detached Git worktree, without changing the existing fast paths for the default or current branch.

## Current Behavior

The worktree picker supports immediate creation from the repository default branch or current branch. When the user types a worktree name, creation uses the remote default branch when available in a single-repository project and otherwise uses the current HEAD. The creation service and Git backend already accept explicit local and remote base refs.

## Scope

- Add custom base-branch selection for single-repository projects.
- Preserve immediate default-branch and current-branch creation.
- Support both generated and user-entered worktree names.
- Reuse the existing branch selection UI and existing worktree creation service.
- Keep multi-repository projects on their existing current-HEAD behavior.

Different base branches per repository, branch creation, and backend or protocol changes are out of scope.

## User Experience

With an empty worktree-picker query, keep the existing default-branch and current-branch creation rows and add a `Choose Base Branch…` row for single-repository projects. Confirming that row opens the existing branch selection modal. Confirming a branch creates a detached worktree with a generated name.

With a non-empty query, confirming `Create <normalized-name>` opens the branch selection modal instead of creating immediately. Confirming a branch creates the detached worktree with the normalized name.

The branch selector lists and searches existing local and remote branches using the established `BranchList` select mode. Escape cancels selection and creates nothing. Multi-repository projects do not show custom base selection and retain the current creation flow.

## Architecture

Extend the type-erased branch-picker builder seam in `git_ui_core` so core code can request select mode without depending on the higher-level `git_ui` crate. The concrete builder installed by `git_ui::init` continues to construct the existing `branch_picker::select_modal` and `BranchList::new_select` UI.

The worktree picker captures the optional normalized worktree name, dismisses itself, and opens the branch selector. The branch callback converts the selected `Branch` as follows:

- Local branch: `NewWorktreeBranchTarget::ExistingBranch { name }`
- Remote branch: `NewWorktreeBranchTarget::RemoteBranch { remote_name, branch_name }`

The callback then invokes the existing `worktree_service::handle_create_worktree`. The service continues to resolve the ref, fetch selected remote branches, create detached worktrees, roll back partial multi-path creation, and open the resulting workspace. No action, project, Git, or protobuf contract changes are required.

The sidebar worktree creation menu adds equivalent custom-base selection for single-repository projects so all manual creation surfaces remain consistent.

## State and Error Handling

- Reuse `creation_blocked_reason` and the active-creation guard to prevent concurrent worktree creation.
- Do not dispatch creation when branch selection is cancelled, the workspace is dropped, or no repository is available.
- Preserve existing remote-fetch error toasts and retry behavior.
- Keep local branch selection fetch-free.
- Continue creating detached worktrees even when the selected local branch is checked out elsewhere.
- Do not expose custom base selection for multi-repository projects because one shared ref may not exist in every repository.

## Verification

Add focused tests covering:

- `Choose Base Branch…` appears only for a single-repository project.
- A named worktree is not created before branch confirmation.
- Confirming a local branch forwards `ExistingBranch` and the normalized name.
- Confirming a remote branch forwards `RemoteBranch` and the normalized name.
- Cancelling selection creates nothing.
- Existing immediate default-branch and current-branch creation behavior remains unchanged.

Run the focused `git_ui_core` and `git_ui` tests, Rust formatting, and an interactive smoke test of name → branch → creation.