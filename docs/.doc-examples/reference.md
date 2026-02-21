<!--
  GOLD STANDARD EXAMPLE: Reference Documentation

  This example demonstrates documentation for API/reference content like tools,
  actions, or other enumerable items.

  Key patterns to note:
  - Anchor IDs on categories and individual items for deep-linking
  - Opening paragraph explains what these are and where they're used
  - Organized into logical categories
  - Each item has a clear, actionable description
  - Links to related configuration docs
  - "See Also" section for related topics
-->

---

title: AI Agent Tools - Zed
description: Built-in tools for Zed's AI agent including file editing, code search, terminal commands, web search, and diagnostics.

---

# Tools

Zed's built-in agent has access to these tools for reading, searching, and editing your codebase. These tools are used in the [Agent Panel](./agent-panel.md) during conversations with AI agents.

You can configure permissions for tool actions, including situations where they are automatically approved, automatically denied, or require your confirmation on a case-by-case basis. See [Tool Permissions](./tool-permissions.md) for the list of permission-gated tools and details.

To add custom tools beyond these built-in ones, see [MCP servers](./mcp.md).

## Read & Search Tools {#read-search-tools}

### `diagnostics` {#diagnostics}

Gets errors and warnings for either a specific file or the entire project, useful after making edits to determine if further changes are needed.
When a path is provided, shows all diagnostics for that specific file.
When no path is provided, shows a summary of error and warning counts for all files in the project.

### `fetch` {#fetch}

Fetches a URL and returns the content as Markdown. Useful for providing docs as context.

### `find_path` {#find-path}

Quickly finds files by matching glob patterns (like `**/*.js`), returning matching file paths alphabetically.

### `grep` {#grep}

Searches file contents across the project using regular expressions, preferred for finding symbols in code without knowing exact file paths.

### `list_directory` {#list-directory}

Lists files and directories in a given path, providing an overview of filesystem contents.

### `now` {#now}

Returns the current date and time.

### `open` {#open}

Opens a file or URL with the default application associated with it on the user's operating system.

### `read_file` {#read-file}

Reads the content of a specified file in the project, allowing access to file contents.

### `thinking` {#thinking}

Allows the Agent to work through problems, brainstorm ideas, or plan without executing actions, useful for complex problem-solving.

### `web_search` {#web-search}

Searches the web for information, providing results with snippets and links from relevant web pages, useful for accessing real-time information.

## Edit Tools {#edit-tools}

### `copy_path` {#copy-path}

Copies a file or directory recursively in the project, more efficient than manually reading and writing files when duplicating content.

### `create_directory` {#create-directory}

Creates a new directory at the specified path within the project, creating all necessary parent directories (similar to `mkdir -p`).

### `delete_path` {#delete-path}

Deletes a file or directory (including contents recursively) at the specified path and confirms the deletion.

### `edit_file` {#edit-file}

Edits files by replacing specific text with new content.

### `move_path` {#move-path}

Moves or renames a file or directory in the project, performing a rename if only the filename differs.

### `restore_file_from_disk` {#restore-file}

Discards unsaved changes in open buffers by reloading file contents from disk. Useful for resetting files to their on-disk state before retrying an edit.

### `save_file` {#save-file}

Saves files that have unsaved changes. Used when files need to be saved before further edits can be made.

### `terminal` {#terminal}

Executes shell commands and returns the combined output, creating a new shell process for each invocation.

## Other Tools {#other-tools}

### `subagent` {#subagent}

Spawns a subagent with its own context window to perform a delegated task. Useful for running parallel investigations, completing self-contained tasks, or performing research where only the outcome matters. Each subagent has access to the same tools as the parent agent.

## See Also {#see-also}

- [Agent Panel](./agent-panel.md) — Where you interact with AI agents
- [Tool Permissions](./tool-permissions.md) — Configure which tools require approval
- [MCP Servers](./mcp.md) — Add custom tools via Model Context Protocol
