---
title: Windows & Projects
description: "How Zed handles multiple projects in windows, including the threads sidebar and options for opening in new windows."
---

# Windows & Projects

Zed lets you work on multiple projects in a single window. Projects appear in the threads sidebar on the left, and you can switch between them while keeping your context intact.

## How Projects Open

By default, when you open a folder in Zed, it opens as a new project in your current window's threads sidebar rather than creating a new window. This keeps related work together and preserves your agent threads and layout.

| Action             | Result                                    |
| ------------------ | ----------------------------------------- |
| File > Open        | Opens in current window (threads sidebar) |
| File > Open Recent | Opens in current window (threads sidebar) |
| Drag folder to Zed | Opens in current window (threads sidebar) |
| `zed ~/project`    | Opens in current window (threads sidebar) |

## Working with Multiple Projects

When you have multiple projects open:

- Click a project header to collapse or expand its threads; Cmd+click (macOS) or Ctrl+click (Linux/Windows) to switch to that project
- Each project has its own file tree, git state, and search scope
- Agent threads are tied to their project context
- Your workspace layout (splits, tabs) is preserved per project

Think of projects in the threads sidebar like browser tabs, but for repositories.

## Opening in a New Window

Sometimes you want a completely separate window. Here's how:

### From Open Recent

When using File > Open Recent ({#kb projects::OpenRecent}):

- **Enter** or **click** opens in the current window (threads sidebar)
- **Cmd+Enter** or **Cmd+click** (macOS) / **Ctrl+Enter** or **Ctrl+click** (Linux/Windows) opens in a new window

### From the CLI

Use the `-n` flag to force a new window:

```sh
zed -n ~/projects/other-project
```

Other CLI options for controlling window behavior:

| Flag            | Behavior                                           |
| --------------- | -------------------------------------------------- |
| `-n`, `--new`   | Always open in a new window                        |
| `-a`, `--add`   | Add to the current window's threads sidebar        |
| `-r`, `--reuse` | Replace the current project in the existing window |

See [CLI Reference](./reference/cli.md) for full details.

### Via Settings

You can change the default CLI behavior with the `cli_default_open_behavior` setting:

```json [settings]
{
  "cli_default_open_behavior": "new_window"
}
```

Options:

- `existing_window` (default): Open folders in the current window's threads sidebar
- `new_window`: Open folders in a new window

This setting affects CLI and double-click behavior, not the File > Open menu.

## Adding Folders to a Project

If you want to add a folder to your current project (not as a separate project in the threads sidebar), you have several options:

- **File menu**: File > Add Folder to Project
- **Project panel**: Right-click in the project panel and choose "Add Folders to Project"
- **Open Recent**: Select a recent project and click the "Add Folder to this Project" button

This adds the folder as an additional root in your current project's file tree, similar to VS Code's multi-root workspaces.

## See Also

- [Threads Sidebar](./ai/parallel-agents.md#threads-sidebar): Managing threads across projects
- [Getting Started](./getting-started.md): Essential commands and setup
- [VS Code Migration](./migrate/vs-code.md): How Zed's project model differs from VS Code
