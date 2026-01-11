# Agent 2 Architecture

## Executive Summary

This document outlines the architecture for Zed's Agent 2 vision, where **threads become the primary organizational unit** of the development experience. Each thread represents a self-contained context with its own workspace layout, terminals, and agent conversation, while sharing underlying infrastructure like worktrees and language servers.

## The Vision

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Window                                                                       │
│ ┌───────────┬───────────────────────────────────────────────────────────────┤
│ │ Threads   │  Active Thread Workspace                                       │
│ │ Sidebar   │  ┌─────────────────────────────────────────────────────────┐  │
│ │           │  │ Center Pane Group (per-thread layout)                   │  │
│ │ [Thread1] │  │  ┌──────────────────┬──────────────────┐                │  │
│ │ [Thread2] │  │  │   Editor Pane    │   Editor Pane    │                │  │
│ │ [------]  │  │  │                  │                  │                │  │
│ │ (grayed)  │  │  └──────────────────┴──────────────────┘                │  │
│ │           │  └─────────────────────────────────────────────────────────┘  │
│ │           │  ┌─────────────────────────────────────────────────────────┐  │
│ │           │  │ Agent Panel (conversation for this thread)              │  │
│ │           │  └─────────────────────────────────────────────────────────┘  │
│ └───────────┴───────────────────────────────────────────────────────────────┤
└─────────────────────────────────────────────────────────────────────────────┘
```

Key characteristics:
- **Thread as context**: Each thread captures a complete working context
- **Switching threads**: Changes the entire workspace view (editors, terminals, agent state)
- **Shared infrastructure**: Worktrees, LSP servers, and git repositories are shared within a window
- **Isolation where it matters**: Each thread has independent checkpoints, undo history, and conversation state
- **Worktree exclusivity**: Threads can only be opened in windows that have their required worktrees

---

## Fundamental Constraints

### 1. One Project Per MultiWorkspace

Each window has exactly one `Project`. All threads within that window share the same Project and its worktrees.

### 2. Threads Bound to One Window

A thread can only be open in one window at a time. This eliminates the split-brain problem entirely—there's no need for coordination because there's only ever one `AcpThread` entity per thread.

### 3. Worktree Exclusivity Across Windows

**Critical constraint**: If a thread requires worktrees that are open in Window A, it cannot be opened in Window B.

Example:
- Window A has worktrees: `~/zed/`, `~/cloud/`
- Window B has worktrees: `~/zed.dev/`
- Thread created with `~/zed/` is **grayed out** in Window B's thread list
- Tooltip: *"This thread uses folders open in another window"*

This constraint:
- Prevents conflicting git operations across windows
- Avoids LSP server duplication for the same folder
- Simplifies reasoning about file system state
- Matches mental model: "this window is for this project"

### 4. Thread ↔ Worktree Association

Threads store which worktrees they were created with. When listing threads, we check availability:

| State | Meaning | UI Treatment |
|-------|---------|--------------|
| **Available** | Thread's worktrees are in this window | Normal, clickable |
| **Blocked** | Thread's worktrees are in another window | Grayed out, tooltip explains |
| **Not Open** | Thread's worktrees aren't open anywhere | Grayed out, could offer to open |

Empty worktree list = "compatible with any workspace" (for pure conversation threads).

---

## Current vs Target Architecture

### Current State

- **Workspace** is 1:1 with Window
- **Project** is 1:1 with Workspace
- **AcpThread** takes `Entity<Project>` and is tightly coupled
- **HistoryStore** is global but threads are per-workspace (split-brain problem)
- All pane/editor state lives in `Workspace`

### Target State

- **MultiWorkspace** is the window root, contains multiple **ThreadContexts**
- **Project** remains 1:1 with window (unchanged!)
- **ThreadContext** holds per-thread state: pane layout, terminal associations, buffer associations
- **AgentThreadStore** (global) tracks thread metadata and availability
- Threads use the window's Project directly (no `SharedProject` abstraction needed)

---

## State Ownership Model

### Shared (One Per Window)

| Component | Rationale |
|-----------|-----------|
| Project | Contains all shared infrastructure |
| WorktreeStore | File system is singular |
| BufferStore | Buffers represent file content; threads can view same file |
| LspStore | Language servers are expensive; one per language per worktree |
| GitStore | Git repository is singular; checkpoints are snapshots from it |

### Per-Thread

| Component | Rationale |
|-----------|-----------|
| AcpThread | Conversation state is unique to each thread |
| PaneGroup (center layout) | Users may have different editor layouts per task |
| Terminal associations | Which terminals belong to this thread's context |
| Buffer associations | Which files this thread has opened/modified |
| Git checkpoints | Snapshots for restore functionality |

### Needs Decision

| Component | Question |
|-----------|----------|
| Docks | Shared across threads vs. per-thread dock state? |
| Navigation history | Per-thread or shared? |
| Active entry | Should clicking a thread restore the last active file? |

---

## Key Design Decisions

### 1. No Project Refactoring Needed

Threads use the window's `Project` directly. Since all threads in a window share the same worktrees (enforced by exclusivity), no `SharedProject` abstraction is needed. `AcpThread` continues taking `Entity<Project>` unchanged.

### 2. Buffers Are Shared, Associations Are Tracked

Buffers live in `BufferStore`. Threads don't own buffers—they track which buffers they've touched. The same `Entity<Buffer>` can be referenced by multiple threads.

### 3. AcpThread Keeps Terminal Ownership

`AcpThread` already owns its terminals. We don't need separate terminal tracking in `ThreadContext`. When switching threads, terminal visibility is a UI concern.

### 4. Git Checkpoints Are Snapshots

Checkpoints capture the full working tree state via `GitStore`. The `GitStoreCheckpoint` is stored per-thread (in conversation state), but the git operations happen on the shared `GitStore`.

---

## Architecture Pressure Test

Deep codebase analysis revealed these challenges:

### Challenge 1: Worktree Exclusivity Is NEW

Today you CAN open the same folder in two Zed windows. This constraint would be new behavior.

**Recommendation**: Enforce at thread selection only (gray out threads), not at folder-open time. This is less disruptive to existing workflows.

### Challenge 2: 290+ References to `active_pane()`

Extracting `PaneGroup` into `ThreadContext` would touch ~290 call sites across the codebase.

**Recommendation**: Keep `Workspace` mostly intact. Add `ThreadContext` as optional, with delegation. Minimize changes to existing code paths.

### Challenge 3: Thread Worktree Data Doesn't Exist

`DbThreadMetadata` has no `worktree_paths` field today. However, `DbThread.initial_project_snapshot` contains worktree paths in `TelemetryWorktreeSnapshot`.

**Recommendation**: Add `worktree_paths` to metadata. Existing threads migrate as "any workspace compatible" (empty list).

### Challenge 4: "Switching" Semantics Need Clarity

When switching threads:
- Is it full swap (hide all panes, show other thread's panes)?
- What about buffers open in both threads?
- Undo history is per-buffer (shared), navigation history should be per-thread

**Recommendation**: Full swap is cleanest. Buffers are shared (`Entity<Buffer>`), but panes are per-thread.

### Challenge 5: Docks and Panels

Agent panel content is per-thread, but docks are shared. Panels need to observe the active thread and update their content accordingly.

**Recommendation**: Add a "thread-aware panel" pattern where panels subscribe to active thread changes.

---

## Migration Path

### Phase 1: Thread Metadata + Availability (LOW RISK)

**Goal**: Thread availability filtering in UI only. No changes to core workspace/project.

- Add `worktree_paths` to `DbThreadMetadata`
- Track which worktrees are open in which windows
- Implement availability checking
- Gray out unavailable threads in UI with tooltips

**Estimated effort**: 1-2 weeks

### Phase 2: Thread-Aware Panels (MEDIUM RISK)

**Goal**: Panels respond to active thread changes.

- Agent panel subscribes to "active thread changed" events
- Multiple threads can exist conceptually, panel shows the active one
- Still single `PaneGroup`, but groundwork for multi-thread

**Estimated effort**: 1-2 weeks

### Phase 3: ThreadContext + Pane Ownership (HIGH RISK)

**Goal**: Extract per-thread state from `Workspace`.

- Create `ThreadContext` struct with pane management
- Add delegation from `Workspace` to active `ThreadContext`
- Handle the 290 `active_pane()` call sites

**Estimated effort**: 3-4 weeks

### Phase 4: Full Multi-Thread UI (HIGH RISK)

**Goal**: Multiple threads per window with switching.

- Thread sidebar
- Thread switching (swap active `ThreadContext`)
- Terminal visibility management
- Serialization/restoration of multiple thread contexts

**Estimated effort**: 3-4 weeks

---

## Risk Assessment

### High Risk

| Area | Risk | Mitigation |
|------|------|------------|
| Workspace refactoring | 290 call sites | Keep Workspace intact, delegate to ThreadContext |
| Serialization | Breaking changes to saved layouts | Version format, migrate on load |

### Medium Risk

| Area | Risk | Mitigation |
|------|------|------------|
| Thread metadata migration | Existing threads lack worktree data | Default to "any workspace compatible" |
| Git checkpoint conflicts | Two threads restore to different points | Warn before restore |

### Low Risk

| Area | Risk | Mitigation |
|------|------|------------|
| Thread availability UI | Just graying out items | Iterate on UX feedback |
| Project unchanged | No refactoring needed | N/A |

---

## Open Questions

### Q1: What happens when you close a thread?
Thread stops, conversation preserved in database, workspace layout optionally serialized for restoration.

### Q2: Can two threads edit the same file?
Yes—they share the same `Entity<Buffer>`. Undo is per-buffer (shared).

### Q3: What's the "no thread" state?
Can a window exist with zero ThreadContexts? Or is there always a "default" context?

### Q4: What triggers thread creation?
Does opening a new window create a thread? Or only explicit "New Thread" action?

### Q5: Are docks per-thread or shared?
Recommendation: Shared, but panels observe active thread for content.

---

## Key Simplifications from Worktree Exclusivity

| Problem | Old Solution | New Solution |
|---------|--------------|--------------|
| Split-brain (same thread in 2 windows) | Complex coordination | Can't happen—thread's worktrees only in one window |
| Background threads | Complex lifecycle | Don't exist—threads stop when window closes |
| Which Project for a thread? | SharedProject abstraction | The window's Project (unchanged) |

This means:
- **No SharedProject extraction needed**
- **No Project refactoring needed**
- **AcpThread keeps using `Entity<Project>` unchanged**
- **Clear ownership**: window owns Project, all threads in window share it

---

## Next Steps

### Immediate Actions

1. **Validate UX**: Prototype grayed-out threads, get feedback on exclusivity constraint
2. **Audit thread creation**: Find all places threads are created/saved
3. **Spike window tracking**: Prototype extending `WorkspaceStore` to track worktrees per window

### Questions for Product/Design

- When a thread is unavailable, should we offer "Open in new window"?
- Should threads show which worktrees they need?
- What's the visual design for the threads sidebar?
- How does thread creation flow work?

---

## Related Documents

- `cross-workspace-threads-plan.md` - Original AgentThreadStore plan (some concepts superseded)
- `crates/workspace/src/workspace.rs` - Current Workspace implementation
- `crates/project/src/project.rs` - Current Project implementation
- `crates/acp_thread/src/acp_thread.rs` - Current AcpThread implementation

---

## Appendix: Component Diagram

```
                          ┌─────────────────────┐
                          │  AgentThreadStore   │ (Global)
                          │  - thread metadata  │
                          │  - worktree paths   │
                          └─────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
            ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
            │  Window 1   │ │  Window 2   │ │  Window 3   │
            │  ~/zed/     │ │  ~/other/   │ │  ~/zed.dev/ │
            └─────────────┘ └─────────────┘ └─────────────┘
                    │
                    ▼
          ┌─────────────────────────────────────────────┐
          │ MultiWorkspace                              │
          │ ┌─────────────────────────────────────────┐ │
          │ │ Project (shared by all threads)         │ │
          │ │ - WorktreeStore → ~/zed/                │ │
          │ │ - BufferStore, LspStore, GitStore       │ │
          │ └─────────────────────────────────────────┘ │
          │ ┌─────────────────────────────────────────┐ │
          │ │ ThreadContext 1 (active)                │ │
          │ │ - AcpThread, PaneGroup, Terminals       │ │
          │ └─────────────────────────────────────────┘ │
          │ ┌─────────────────────────────────────────┐ │
          │ │ ThreadContext 2 (inactive)              │ │
          │ │ - AcpThread, PaneGroup, Terminals       │ │
          │ └─────────────────────────────────────────┘ │
          └─────────────────────────────────────────────┘
```
