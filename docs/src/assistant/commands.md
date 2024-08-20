# Assistant Commands

## Slash Commands

Slash commands enhance the assistant's capabilities. Begin by typing a `/` at the beginning of the line to see a list of available commands:

- `/default`: Inserts the default prompt into the context
- `/diagnostics`: Injects errors reported by the project's language server into the context
- `/fetch`: Inserts the content of a webpage and inserts it into the context
- `/file`: Inserts a single file or a directory of files into the context
- `/now`: Inserts the current date and time into the context
- `/prompt`: Adds a custom-configured prompt to the context (see Prompt Library)
- `/search`: Performs semantic search for content in your project based on natural language
- `/symbols`: Inserts the current tab's active symbols into the context
- `/tab`: Inserts the content of the active tab or all open tabs into the context
- `/terminal`: Inserts a select number of lines of output from the terminal

> **Note:** Remember, commands are only evaluated when the context is created or when the command is inserted, so a command like `/now` won't continuously update, or `/file` commands won't keep their contents up to date.

## `/default`

Read more about `/default` in the [Prompting: Editing the Default Prompt](/assistant/prompting.md#default-prompt) section.

## `/diagnostics`

The diagnostics command injects errors reported by the project's language server into the context. Optionally, the `--include-warnings` flag can be used to include warnings in the context as well.

## Extensibility

The Zed team plans for assistant commands to be extensible, but this isn't quite ready yet. Stay tuned!
