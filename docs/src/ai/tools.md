# Tools

Zed's built-in agent has access to a variety of tools that allow it to interact with your codebase and perform tasks.

## Read & Search Tools

### `diagnostics`

Gets errors and warnings for either a specific file or the entire project.

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

Reads the content of a specified file in the project.

This tool supports two primary ways of reading text:

- **Line range mode** (best when you already have line numbers, e.g. from an outline):
  - Provide `start_line` and/or `end_line` (1-based, inclusive end).
- **Byte window mode** (best for paging efficiently and avoiding repeated small reads):
  - Provide `start_byte` (0-based) and/or `max_bytes`.
  - The returned content is **rounded to whole line boundaries** so it doesn’t start or end mid-line.

For large files, calling `read_file` without a range may return a **file outline** with line numbers instead of full content. To read large files efficiently without line-number paging, use **byte window mode**.

#### Byte window paging recommendations

- Prefer setting a larger `max_bytes` when you expect to read multiple adjacent sections, to reduce tool calls and avoid rate limiting.
- Use `start_byte` to page forward/backward deterministically.

When using byte window mode, the output includes a small header describing the effective returned window (requested/rounded/returned byte ranges and line ranges). Use the returned byte range to pick the next `start_byte` for continued paging.

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

### `terminal`

Executes shell commands and returns the combined output, creating a new shell process for each invocation.
