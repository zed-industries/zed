# Requirements: Auto Worktree Creation on New Thread

## Problem Statement

As a frustrated user, when agents work simultaneously on my repository, they make a mess -- committing to the same branch, causing conflicts, and overwriting each other's changes. This is especially painful when the AI agent modifies files on the same branch I'm actively working on, or when multiple agent threads operate on the same codebase without isolation.

## Goal

When opening a **new agent thread** in Zed, the editor should **automatically create a git worktree** for that thread, branching from `main`/`master` (NOT the user's current branch). This gives each thread an isolated working tree, preventing conflicts between threads and between the user's own work and the agent's work.

## User Scenarios

### Scenario 1: Multiple Parallel Threads
A user opens two new agent threads simultaneously to work on different features. Without this feature, both threads share the same working directory, so file edits from Thread A can conflict with or overwrite changes from Thread B. With this feature, each thread gets its own git worktree based on `main`, ensuring completely isolated development environments.

### Scenario 2: Agent Does Not Pollute User's Current Work
A user is on a feature branch with uncommitted changes. They open a new agent thread to explore another idea. Without this feature, the agent's work would be in the same working tree, potentially interfering with the user's uncommitted changes. With this feature, the agent gets a clean worktree based on `main`, leaving the user's current work untouched.

### Scenario 3: Thread Persistence
A user closes Zed and later reopens it, selecting a previous thread. The worktree associated with that thread must be preserved so the user can continue where they left off. Threads need robust recovery after a restart.

## Functional Requirements

### REQ-1: Automatic Worktree Creation
When a new native agent thread is created, and the thread references a git repository, Zed **must** automatically create a git worktree for that thread.

### REQ-2: Base from `main`/`master` (NOT Current Branch)
The new worktree **must** be based on the repository's default branch (`main`/`master`), **not** the user's currently checked-out branch. This is the critical behavior -- agents should not start from the user's feature branch; they should start from a clean, known-good state.

### REQ-3: Worktree Naming Convention
The worktree directory **must** follow a predictable naming scheme:
```
{.git parent}/.zed-agent-worktrees/worktree-{thread-id}/
```
This keeps worktrees organized and hidden by default, preventing workspace clutter.

### REQ-4: Thread-Worktree Association
Thread metadata **must** store the associated worktree path, so the thread always knows where its files are.

### REQ-5: Thread Lifecycle - Worktree Cleanup
When an agent thread is archived, the user **must** be given the option to also delete or keep the associated worktree. Deleted threads should also trigger cleanup prompts.

### REQ-6: Thread Persistence After Restart
Between subsequent Zed restarts, a thread that previously was associated with a worktree **must** continue to use the same worktree (not create a new one).

### REQ-7: Project Context When Thread-Worktree Exists
When a user opens a thread that has an associated worktree, Zed **must** add that worktree to the project context, allowing the agent to edit files in the worktree.

### REQ-8: No Worktree for Non-Git Projects
For projects not under git version control, Zed **must NOT** attempt to create a worktree; the thread operates directly in the project directory.

### REQ-9: No Worktree When Disabled
Users **must** be able to disable this feature via Zed settings. When disabled, threads behave exactly as they do today with no automatic worktree creation.

### REQ-10: Worktree on Read-Only or Remote
When a project is on a remote server or in read-only mode, Zed **must** gracefully degrade and not attempt to create a worktree.

### REQ-11: Thread Switching in Same Zed Window
When a user switches from Thread A to Thread B, the project's active worktree **must** switch accordingly. The editor shows the files of whichever thread is active, and any open file tabs not relevant to the new worktree gracefully close or update.

### REQ-12: Legacy Thread Backward Compatibility
If a thread does not have a worktree associated (either because the feature is disabled or because the thread predates this feature), the thread **must** operate in the main project directory, exactly as it does today.

### REQ-13: Default Setting
By default, this feature **must** be enabled to provide isolation out of the box. Users can opt out via settings.

## Non-Functional Requirements

### NFR-1: Performance
Worktree creation (running `git worktree add`) should add no more than 2-3 seconds to thread creation time for typical repositories (< 1GB).

### NFR-2: User Feedback
During worktree creation, the thread UI should show a loading indicator with text like "Setting up workspace..." to inform the user.

### NFR-3: Error Handling
If worktree creation fails (e.g., disk space, permissions, git error), the thread should still be created, but a non-blocking notification should inform the user of the failure, and the thread should fall back to operating in the main project directory.

### NFR-4: Disk Space Awareness
Worktrees can consume significant disk space. Zed should expose a setting for a maximum number of retained worktrees. This is a future enhancement; for the initial release, simply documenting this in the settings is acceptable.

### NFR-5: Isolation
The worktree must be truly independent -- changes in one worktree should not affect another, and git operations within a worktree should be scoped to that worktree.

## Out of Scope

- Automatic branch naming/pushing of the worktree content. The worktree is a scratch space for the agent.
- Automatic cleanup of old/stale worktrees beyond the archive flow.
- Integration with external CI/CD or remote repositories.
