# Code Action Tool

This tool performs code actions on text ranges in the project, such as:

- **List Actions**: Lists all available code actions for a specific text range
- **Execute Action**: Executes a specific code action matching a regex pattern
- **Rename**: Renames a text range (typically a symbol) across the codebase

## Basic Usage

To use this tool, you need to:
1. Specify the path to the file containing the text range
2. Provide context around the text range to identify it uniquely
3. Optionally specify an action to execute

## Modes of Operation

### List Available Actions
To list all available code actions for a text range, simply omit the `action` field:

```json
{
  "path": "src/main.rs",
  "text_range": "my_variable",
  "context_before_range": "let ",
  "context_after_range": " = 42;"
}
```

### Execute a Specific Action
To execute a specific code action, provide a regex pattern in the `action` field:

```json
{
  "path": "src/main.rs",
  "text_range": "my_variable",
  "context_before_range": "let ",
  "context_after_range": " = 42;",
  "action": "Extract function"
}
```

The regex pattern must match exactly one code action title. If it matches zero or multiple actions, the tool will return an error with appropriate guidance.

### Rename Operation
To perform a rename operation, set the `action` field to "textDocument/rename" and provide the new name in the `arguments` field:

```json
{
  "path": "src/main.rs",
  "text_range": "my_variable",
  "context_before_range": "let ",
  "context_after_range": " = 42;",
  "action": "textDocument/rename",
  "arguments": "new_variable_name"
}
```

## Typical Workflow

1. First call the tool without an `action` field to list all available code actions
2. From the list, choose a desired action and execute it in a second call by providing its name (or a regex that uniquely matches it) in the `action` field

You can also directly guess common code actions without first listing them by providing a regex pattern that might match actions like "Extract function", "Extract variable", etc.
