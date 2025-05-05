# Tools

Zed's Agent has access to a variety of tools that allow it to interact with your codebase and perform tasks:

- **`copy_path`**: Copies a file or directory recursively in the project, more efficient than manually reading and writing files when duplicating content.
- **`create_directory`**: Creates a new directory at the specified path within the project, creating all necessary parent directories (similar to `mkdir -p`).
- **`create_file`**: Creates a new file at a specified path with given text content, the most efficient way to create new files or completely replace existing ones.
- **`delete_path`**: Deletes a file or directory (including contents recursively) at the specified path and confirms the deletion.
- **`diagnostics`**: Gets errors and warnings for either a specific file or the entire project, useful after making edits to determine if further changes are needed.
- **`edit_file`**: Edits files by replacing specific text with new content.
- **`fetch`**: Fetches a URL and returns the content as Markdown. Useful for providing docs as context.
- **`list_directory`**: Lists files and directories in a given path, providing an overview of filesystem contents.
- **`move_path`**: Moves or renames a file or directory in the project, performing a rename if only the filename differs.
- **`now`**: Returns the current date and time.
- **`find_path`**: Quickly finds files by matching glob patterns (like "**/*.js"), returning matching file paths alphabetically.
- **`read_file`**: Reads the content of a specified file in the project, allowing access to file contents.
- **`grep`**: Searches file contents across the project using regular expressions, preferred for finding symbols in code without knowing exact file paths.
- **`terminal`**: Executes shell commands and returns the combined output, creating a new shell process for each invocation.
- **`thinking`**: Allows the Agent to work through problems, brainstorm ideas, or plan without executing actions, useful for complex problem-solving.
- **`web_search`**: Searches the web for information, providing results with snippets and links from relevant web pages, useful for accessing real-time information.
