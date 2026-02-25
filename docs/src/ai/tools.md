---
title: AI Agent Tools - Zed
description: Built-in tools for Zed's AI agent including file editing, code search, terminal commands, web search, and diagnostics.
---

# Tools

Zed's built-in agent has access to these tools for reading, searching, and editing your codebase. These tools are used in the [Agent Panel](./agent-panel.md) during conversations with AI agents.

You can configure permissions for tool actions, including situations where they are automatically approved, automatically denied, or require your confirmation on a case-by-case basis. See [Tool Permissions](./tool-permissions.md) for the list of permission-gated tools and details.

To add custom tools beyond these built-in ones, see [MCP servers](./mcp.md).

## Read & Search Tools

### `diagnostics`

Gets errors and warnings for either a specific file or the entire project, useful after making edits to determine if further changes are needed.
When a path is provided, shows all diagnostics for that specific file.
When no path is provided, shows a summary of error and warning counts for all files in the project.

### `fetch`

Fetches a URL and returns the content as Markdown. Useful for providing docs as context.

### `find_path`

Quickly finds files by matching glob patterns (like "\*_/_.js"), returning matching file paths alphabetically.

### `grep`

Searches file contents across the project using regular expressions, preferred for finding symbols in code without knowing exact file paths.

### `list_directory`

Lists files and directories in a given path, providing an overview of filesystem contents.

### `now`

Returns the current date and time.

### `open`

Opens a file or URL with the default application associated with it on the user's operating system.

### `read_file`

Reads the content of a specified file in the project, allowing access to file contents.

### `thinking`

Allows the Agent to work through problems, brainstorm ideas, or plan without executing actions, useful for complex problem-solving.

### `web_search`

Searches the web for information, providing results with snippets and links from relevant web pages, useful for accessing real-time information.

## Edit Tools

### `copy_path`

Copies a file or directory recursively in the project, more efficient than manually reading and writing files when duplicating content.

### `create_directory`

Creates a new directory at the specified path within the project, creating all necessary parent directories (similar to `mkdir -p`).

### `delete_path`

Deletes a file or directory (including contents recursively) at the specified path and confirms the deletion.

### `edit_file`

Edits files by replacing specific text with new content.

### `move_path`

Moves or renames a file or directory in the project, performing a rename if only the filename differs.

### `restore_file_from_disk`

Discards unsaved changes in open buffers by reloading file contents from disk. Useful for resetting files to their on-disk state before retrying an edit.

### `save_file`

Saves files that have unsaved changes. Used when files need to be saved before further edits can be made.

### `terminal`

Executes shell commands and returns the combined output, creating a new shell process for each invocation.

## Other Tools

### `subagent`

> **Preview:** This feature is available in Zed Preview. It will be included in the next Stable release.

Spawns an independent agent to handle a delegated task. Use it for:

- Running multiple tasks in parallel that would take significantly longer sequentially
- Completing self-contained tasks where you need the outcome but not intermediate steps
- Investigations where you need the conclusion but not the research trail

Avoid using subagents for simple operations you could accomplish with one or two direct tool calls—each agent has startup overhead.

**Context isolation**: Spawned agents cannot see your conversation history or attached context. Include all necessary information (file paths, requirements, constraints) in the task description.

**Parameters**:

- `label`: Short description shown in the UI (e.g., "Researching alternatives")
- `task`: Complete task description with all context the agent needs
- `timeout_secs`: Optional maximum runtime in seconds

When spawning multiple agents that write to the filesystem, provide guidance to avoid conflicts (e.g., assign different directories).

#### Context Window Management

> **Changed in Preview (v0.225).** See [release notes](/releases#0.225).

When a subagent approaches 80% of its context window, Zed automatically stops it to prevent running out of space. You'll see this message:

```
The agent is nearing the end of its context window and has been stopped.
You can prompt the thread again to have the agent wrap up or hand off its work.
```

To continue:

- Send another message to the thread
- The subagent can wrap up its current work
- Or it can hand off to a new subagent with a fresh context window

#### Error Messages

Subagents may stop with specific errors:

**Maximum tokens**: The agent exhausted its available tokens. Prompt again to continue in a new turn.

**Maximum turn requests**: Too many operations occurred in a single turn. Send another message to continue.

**Refusal**: The model declined to process your prompt. Rephrase and try again.

**No response**: The agent didn't return output. Send your message again.
