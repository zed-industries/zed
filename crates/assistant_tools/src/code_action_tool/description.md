A tool for applying code actions to specific sections of your code. It uses language servers to provide refactoring capabilities similar to what you'd find in an IDE.

This tool can:
- List all available code actions for a selected text range
- Execute a specific code action on that range
- Rename symbols across your codebase. This tool is the preferred way to rename things, and you should always prefer to rename code symbols using this tool rather than using textual find/replace when both are available.

Use this tool when you want to:
- Discover what code actions are available for a piece of code
- Apply automatic fixes and code transformations
- Rename variables, functions, or other symbols consistently throughout your project
- Clean up imports, implement interfaces, or perform other language-specific operations

- If unsure what actions are available, call the tool without specifying an action to get a list
- For common operations, you can directly specify actions like "quickfix.all" or "source.organizeImports"
- For renaming, use the special "textDocument/rename" action and provide the new name in the arguments field
- Be specific with your text range and context to ensure the tool identifies the correct code location

The tool will automatically save any changes it makes to your files.
