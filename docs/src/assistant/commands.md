# Assistant Commands

## Overview

Slash commands enhance the assistant's capabilities. Begin by typing a `/` at the beginning of the line to see a list of available commands:

- `/default`: Inserts the default prompt into the context
- `/diagnostics`: Injects errors reported by the project's language server into the context
- `/fetch`: Fetches the content of a webpage and inserts it into the context
- `/file`: Inserts a single file or a directory of files into the context
- `/now`: Inserts the current date and time into the context
- `/prompt`: Adds a custom-configured prompt to the context ([see Prompt Library](./prompting#prompt-library))
- `/symbols`: Inserts the current tab's active symbols into the context
- `/tab`: Inserts the content of the active tab or all open tabs into the context
- `/terminal`: Inserts a select number of lines of output from the terminal
- `/selection`: Inserts the selected text into the context

### Other Commands:

- `/search`: Performs semantic search for content in your project based on natural language
  - Not generally available yet, but some users may have access to it.

> **Note:** Remember, commands are only evaluated when the context is created or when the command is inserted, so a command like `/now` won't continuously update, or `/file` commands won't keep their contents up to date.

## `/default`

Read more about `/default` in the [Prompting: Editing the Default Prompt](./prompting.md#default-prompt) section.

Usage: `/default`

## `/diagnostics`

The `/diagnostics` command injects errors reported by the project's language server into the context. This is useful for getting an overview of current issues in your project.

Usage: `/diagnostics [--include-warnings] [path]`

- `--include-warnings`: Optional flag to include warnings in addition to errors.
- `path`: Optional path to limit diagnostics to a specific file or directory.

## `/file`

The `/file` command inserts the content of a single file or a directory of files into the context. This allows you to reference specific parts of your project in your conversation with the assistant.

Usage: `/file <path>`

You can use glob patterns to match multiple files or directories.

Examples:

- `/file src/index.js` - Inserts the content of `src/index.js` into the context.
- `/file src/*.js` - Inserts the content of all `.js` files in the `src` directory.
- `/file src` - Inserts the content of all files in the `src` directory.

## `/now`

The `/now` command inserts the current date and time into the context. This can be useful letting the language model know the current time (and by extension, how old their current knowledge base is).

Usage: `/now`

## `/prompt`

The `/prompt` command inserts a prompt from the prompt library into the context. It can also be used to nest prompts within prompts.

Usage: `/prompt <prompt_name>`

Related: `/default`

## `/symbols`

The `/symbols` command inserts the active symbols (functions, classes, etc.) from the current tab into the context. This is useful for getting an overview of the structure of the current file.

Usage: `/symbols`

## `/tab`

The `/tab` command inserts the content of the active tab or all open tabs into the context. This allows you to reference the content you're currently working on.

Usage: `/tab [tab_name|all]`

- `tab_name`: Optional name of a specific tab to insert.
- `all`: Insert content from all open tabs.

Examples:

- `/tab` - Inserts the content of the active tab.
- `/tab "index.js"` - Inserts the content of the tab named "index.js".
- `/tab all` - Inserts the content of all open tabs.

## `/terminal`

The `/terminal` command inserts a select number of lines of output from the terminal into the context. This is useful for referencing recent command outputs or logs.

Usage: `/terminal [<number>]`

- `<number>`: Optional parameter to specify the number of lines to insert (default is a 50).

## `/selection`

The `/selection` command inserts the selected text in the editor into the context. This is useful for referencing specific parts of your code.

This is equivalent to the `assistant: quote selection` command ({#kb assistant::QuoteSelection}). See [Interacting with the Assistant](./assistant-panel.md#interacting-with-the-assistant)).

Usage: `/selection`

## Extensibility

Additional slash commands can be provided by extensions.

See [Extension: Slash Commands](../extensions/slash-commands.md) to learn how to create your own.
