# Tools

Zed's built-in agent has access to a variety of tools that allow it to interact with your codebase and perform tasks.

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

### `terminal`

Executes shell commands and returns the combined output, creating a new shell process for each invocation.
