# Parallel Agents

Zed supports running multiple agents in parallel across many projects at the same time. The Threads Panel organizes your running and historical threads by project, giving you a single view to monitor and switch between all of your active agent sessions. As usual in Zed, you can only have one project open at a time in any given window.

## Threads Panel {#threads-panel}

The Threads Panel is the control center for your parallel agents. It displays a list of projects and the threads running inside each one, so you can see at a glance what every agent is working on. Each project appears as a collapsible group, with its threads listed underneath. Active threads show their current status, and you can click any thread to jump into it.

## Managing Projects {#managing-projects}

### Window Scoping {#window-scoping}

The list of projects in the Threads Panel is scoped to the current window, just like everything else in Zed. If you have a set of threads running in one window, you can open additional windows with different projects also running their own agents in parallel.

Note that the threads themselves are global, and tied to your filesystem, but the list of projects that organize those threads are local to the window you're looking at.

Q: What about if I try to open the same folder in two different windows?
A: If you have that project opened in a different window then we will focus that window for you. If you do not have a project opened in any window, then we'll open it in this window.

Q: What happens if I open a new window?
A: We will start you with an empty Threads Panel, ready for you to add projects to.

Q: What happens if I use the CLI, e.g. `zed my-project/`?
A: If you have a window with that project opened, that window will be focused. Otherwise, it will open in a new window with a single project in the Threads Panel.

### Adding Projects {#adding-projects}

To add a new project to the Threads Panel, open a folder the way you normally would in Zed — through the recent projects picker or by opening a new folder, or via the CLI. The project will appear in the Threads Panel automatically, above your last opened project.

### Removing Projects {#removing-projects}

To remove a project from the Threads Panel, click the close button on its header. This removes the project from the panel's list but will not archive or delete the threads associated with that project. If you want to see those threads, you can re-open the project.

Q: What happens if I remove all of the projects in my window, while I have a project open?
A: You cannot remove the currently open project. However, if you do not have any projects open in the window, then we will show the empty state.

Q: What happens if I have a running thread and remove its project from my window?
A: That thread will stop running.

### Multi-Folder Projects {#multi-folder-projects}

When you add or remove folders from your current project, the Threads Panel updates to reflect the change. The new set of folders is added as a new project to the Threads Panel so that you can maintain any threads running in your project when you add the folder.

## Worktrees {#worktrees}

The Zed agent supports automatic worktree initialization across all agents. When you select "New Worktree" from the dropdown, a new Git worktree is created for each root repository in the project. See the worktree-specific documentation for how the worktrees are initialized.

For each thread started via this new worktree option, a corresponding thread entry is inserted underneath that project in the Threads Panel, with a chip indicating the worktree used. This lets you run isolated agent sessions on separate worktrees without them interfering with each other's work.

> [!NOTE]
> Worktrees are considered a separate project, but are only shown as part of their main worktree's project. You can have a window with the main worktree's project open and a window with the linked worktree open, even though normally you can only have one project open at a time.

> [!NOTE]
> Worktrees without threads will not show up in the Threads Panel, only their original repository will be shown.

Q: What about worktrees made outside of Zed? How do I get them into the Threads Panel?
A: Simply open each folder, like you would any other folder, and then click the agent panel and start typing. The new thread with the worktree chip will appear automatically in the Threads Panel once you submit your prompt.

Q: What about if I want a thread across multiple different worktrees of the same set of projects? (e.g. `zed/worktree-1` and `cloud/externally-created-worktree`)
A: Open the projects like you would in Zed normally (cmd-o to select the folder, then add folder to root project), and then start a new thread via the agent panel. The new worktree thread will appear automatically in the Threads Panel, with multiple worktree chips for each configured worktree.

Q: How do I open each worktree in a separate window?
A: Open a window and select the folders for those worktrees.
