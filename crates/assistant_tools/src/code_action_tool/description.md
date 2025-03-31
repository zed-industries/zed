# Code Action Tool

This tool performs code actions on symbols in the project, such as:

- **Rename**: Renames a symbol across the codebase
- **Quickfix**: Applies automatic fixes to code problems

To use this tool, you need to:
1. Specify the path to the file containing the code you want to run an action on
2. Choose the code action type (Rename or Quickfix)
3. For Rename actions: provide context around the symbol to identify it uniquely, along with the new name
4. For Quickfix actions: provide context around the code that needs fixing

The tool will apply the requested code action to all relevant occurrences in the project.
