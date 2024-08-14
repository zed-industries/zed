# AI Assistant Slash Commands

The [Zed Assistant Panel](./language-model-integration.md) supports a number of slash commands to include data from your Zed workspace as context for the AI Assistant.

This is under active development and subject to change. Please download [Zed Preview](https://zed.dev/releases/preview) to see the latest improvements.

## Prompts

- `/default` inserts your default system prompts
- `/prompt` inserts a prompt from your Prompt Library

## Workspace

- `/tab` insert the contents of an open tab
- `/file` insert the contents of a file or directory
- `/symbols` insert symbols for active tab
- `/diagnostics` insert errors from the diagnostics panel
- `/diagnostics --include-warnings` insert errors and warnings from the diagnostic panel
- `/terminal` insert the contents of a Zed terminal

## Workflow

- `/workflow` begin a workflow

## Other Commands

- `/now` inserts the current date and time, e.g. `Today is Tue, 20 Aug 2024 13:05:06 -0400.`
- `cmd + >` inserts the active editor buffer selection as a quoted code block
