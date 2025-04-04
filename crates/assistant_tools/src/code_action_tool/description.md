A tool for applying code actions to specific sections of your code. It uses language servers to provide refactoring capabilities similar to what you'd find in an IDE.

This tool can:
- List all available code actions for a selected text range
- Execute a specific code action on that range
- Rename symbols across your codebase. This tool is the preferred way to rename things, and you should always prefer to rename code symbols using this tool rather than using textual find/replace when both are available.
- Find and replace specific text within a file with precise context matching. This is the preferred way to make text edits when no other code action is appropriate.

Use this tool when you want to:
- Discover what code actions are available for a piece of code
- Apply automatic fixes and code transformations
- Rename variables, functions, or other symbols consistently throughout your project
- Clean up imports, implement interfaces, or perform other language-specific operations
- Make precise text edits within a file

- If unsure what actions are available, call the tool without specifying an action to get a list
- For common operations, you can directly specify actions like "quickfix.all" or "source.organizeImports"
- For renaming, use the special "textDocument/rename" action and provide the new name in the arguments field
- For finding and replacing text, use the special "textDocument/findReplace" action, with text_range as the text to find and arguments as the replacement text
- Be specific with your text range and context to ensure the tool identifies the correct code location

The tool will automatically save any changes it makes to your files.

For find and replace operations, only use this tool when you want to edit a subset of a file's contents, but not the entire file. You should not use this when you want to replace the entire contents of a file with completely different contents. You also should not use this when you want to move or rename a file. You absolutely must NEVER use this to create new files from scratch.
