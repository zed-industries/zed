---
title: Parallel Agents - Zed
description: Run multiple agent threads concurrently using the Threads Sidebar, manage them across projects, and isolate work using Git worktrees.
---

# Parallel Agents

Parallel Agents lets you run multiple agent threads at once, each working independently with its own agent, context window, and conversation history. The Threads Sidebar is where you start, manage, and switch between them.

Open the Threads Sidebar with {#kb multi_workspace::ToggleWorkspaceSidebar}.

> **Note:** From version 0.233.0 onward, the Agent Panel and Threads Sidebar are on the left by default. The Project Panel, Git Panel, and other panels move to the right, keeping the thread list and conversation next to each other. To rearrange panels, right-click any panel icon.

## Threads Sidebar {#threads-sidebar}

The sidebar shows your threads grouped by project. Each project gets its own section with a header. Threads appear below with their title, status indicator, and which agent is running them. Threads running in linked Git worktrees appear under the same project as their main worktree. See [Worktree Isolation](#worktree-isolation).

To focus the sidebar without toggling it, use {#kb multi_workspace::FocusWorkspaceSidebar}. To search your threads, press {#kb agents_sidebar::FocusSidebarFilter} while the sidebar is focused.

### Switching Threads {#switching-threads}

Click any thread in the sidebar to switch to it. The Agent Panel updates to show that thread's conversation.

For quick switching without opening the sidebar, use the thread switcher: press {#kb agents_sidebar::ToggleThreadSwitcher} to cycle forward through recent threads, or hold `Shift` while pressing that binding to go backward. This works from both the Agent Panel and the Threads Sidebar.

### Thread History {#threads-history}

To remove a thread from the sidebar, you can archive it by hovering over it and clicking the archive icon that appears. You can also select a thread and press {#kb agent::ArchiveSelectedThread}. Running threads cannot be moved to history until they finish.

The Thread History view holds all your threads, including ones that you have archived. Toggle it with {#kb agents_sidebar::ToggleThreadHistory} or by clicking the clock icon in the sidebar bottom bar, next to the sidebar toggle.

To restore a thread, open Thread History and click the thread you want to bring back. Zed moves it back to the thread list and opens it in the Agent Panel. If the thread was running in a Git worktree that was removed, Zed restores the worktree automatically.

To permanently delete a thread, open Thread History, hover over the thread, and click the trash icon. This removes the thread's conversation history and cleans up any associated worktree data. Deleted threads cannot be recovered.

You can search your threads in history; search will fuzzy match on thread titles.

### Importing External Agent Threads {#importing-threads}

If you have external agents installed, Zed will detect whether you have existing threads and invite you to import them into Zed. Once you open Thread History, you'll find an import icon button in the Thread History toolbar that lets you import threads at any time. Clicking on it opens a modal where you can select the agents whose threads you want to import.

## Running Multiple Threads {#running-multiple-threads}

Each thread runs independently, so you can send a prompt, open a second thread, and give it a different task while the first continues working. To scope a new thread to a specific project, hover over that project's header in the Threads Sidebar and click the `+` button, or use {#action agents_sidebar::NewThreadInGroup} from the keyboard. See [Creating New Threads](./agent-panel.md#new-thread) for the other entry points.

Each thread can use a different agent, so you can run Zed's built-in agent in one thread and an [external agent](./external-agents.md) like Claude Code or Codex in another.

## Multiple Projects {#multiple-projects}

The Threads Sidebar can hold multiple projects at once. Each project gets its own group with its own threads and conversation history.

To add another project to the sidebar, click the **Add Project** button (open-folder icon) in the sidebar bottom bar. The popover that opens lists your recent projects and also provides **Add Local Folders** and **Add Remote Folder** buttons at the bottom.

### Multi-Root Folder Projects {#multi-root-folder-projects}

A single project can contain multiple folders (a multi-root folder project). Agents can then read and write across all of those folders in a single thread. There are two ways to set one up:

- **From the sidebar:** Click the **Add Project** button, choose **Add Local Folders**, and select multiple folders in the file picker. They open together as one multi-root project.
- **From the title bar:** Click the project picker (the leftmost project name). For any local entry in the recent projects list, hover it and click the folder-with-plus icon (**Add Folder to this Project**) to merge that project's folders into the current project.

## Worktree Isolation {#worktree-isolation}

If two threads might edit the same files, start one in a new Git worktree to give it an isolated checkout.

Worktrees are managed from the title bar. Click the worktree picker (to the right of the project picker) to switch between existing worktrees or create a new one. New worktrees are created in a detached HEAD state, so you won't accidentally share a branch between worktrees.

Once you're in a new worktree, use the branch picker next to the worktree picker to create a new branch or check out an existing one. If the branch you pick is already checked out in another worktree, the current worktree stays in detached HEAD until you choose a different branch.

To automate setup steps whenever a new worktree is created use a [Task hook](../tasks.md#hooks). The `create_worktree` hook runs automatically after Zed creates a linked worktree, with `ZED_WORKTREE_ROOT` pointing at the new worktree and `ZED_MAIN_GIT_WORKTREE` pointing at the original repository.

After the agent finishes, review the diff and merge the changes through your normal Git workflow. If the thread was running in a linked worktree and no other active threads use it, moving the thread to Thread History saves the worktree's Git state and removes it from disk. Restoring the thread from history restores the worktree.

## See Also {#see-also}

- [Agent Panel](./agent-panel.md): Manage individual threads and configure the agent
- [External Agents](./external-agents.md): Use Claude Code, Gemini CLI, and other agents
- [Tools](./tools.md): Built-in tools available in each thread
