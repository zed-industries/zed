# Code Action Tool

This tool performs code actions on text ranges in the project, such as:

- **Rename**: Renames a text range (typically a symbol) across the codebase
- **ListAvailable**: Lists all available code actions for a specific text range
- **ExecuteAction**: Executes a specific code action matching a regex pattern

To use this tool, you need to:
1. Specify the path to the file containing the text range you want to run an action on
2. Choose the code action type (Rename, ListAvailable, or ExecuteAction)
3. For all action types: provide context around the text range to identify it uniquely
4. For Rename actions: provide the new name
5. For ExecuteAction: provide a regex pattern that uniquely identifies the action to execute, and optional arguments

Typical workflow:
1. Use the ListAvailable action to discover available code actions for a text range
2. Choose one of the listed actions by creating a regex pattern that uniquely matches it
3. Execute that specific action using ExecuteAction with the pattern

The regex pattern must match exactly one code action title. If it matches zero or multiple actions, the tool will return an error with appropriate guidance.

Examples:
- Use `^Extract function$` to match an action titled exactly "Extract function"
- Use `Rename to '.*'` to match any action that starts with "Rename to '" 

You can also use the tool to directly guess code actions without first listing them, by providing a regex pattern that might match common code actions like "Extract function", "Extract variable", etc.
