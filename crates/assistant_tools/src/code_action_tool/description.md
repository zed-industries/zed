# Code Action Tool

This tool performs code actions on text ranges in the project, such as:

- **Rename**: Renames a text range (typically a symbol) across the codebase
- **ListAvailable**: Lists all available code actions for a specific text range

To use this tool, you need to:
1. Specify the path to the file containing the text range you want to run an action on
2. Choose the code action type (Rename or ListAvailable)
3. For both action types: provide context around the text range to identify it uniquely
4. For Rename actions: provide the new name

The tool will apply the requested code action to all relevant occurrences in the project, or list all available actions for the specified text range.
