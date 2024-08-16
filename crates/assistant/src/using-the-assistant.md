## Assistant Panel

Once you have configured a provider, you can interact with the provider's language models in a context editor.

To create a new context editor, use the menu in the top right of the assistant panel and select the `New Context` option.

In the context editor, select a model from one of the configured providers, type a message in the `You` block, and submit with `cmd-enter` (or `ctrl-enter` on Linux).

### Adding Prompts

You can customize the default prompts used in new context editors by opening the `Prompt Library`.

Open the `Prompt Library` using either the menu in the top right of the assistant panel and choosing the `Prompt Library` option, or by using the `assistant: deploy prompt library` command when the assistant panel is focused.

### Viewing past contexts

You can view all previous contexts by opening the `History` tab in the assistant panel.

Open the `History` using the menu in the top right of the assistant panel and choosing `History`.

### Slash commands

Slash commands enhance the assistant's capabilities. Begin by typing a `/` at the beginning of the line to see a list of available commands:

- default: Inserts the default prompt into the context
- diagnostics: Injects errors reported by the project's language server into the context
- fetch: Pulls the content of a webpage and inserts it into the context
- file: Pulls a single file or a directory of files into the context
- now: Inserts the current date and time into the context
- prompt: Adds a custom-configured prompt to the context (see Prompt Library)
- search: Performs semantic search for content in your project based on natural language
- symbols: Pulls the current tab's active symbols into the context
- tab: Pulls in the content of the active tab or all open tabs into the context
- terminal: Pulls in a select number of lines of output from the terminal

## Inline assistant

You can use `ctrl-enter` to open the inline assistant in both a normal editor and within the assistant panel.

The inline assistant allows you to send the current selection (or the current line) to a language model and modify the selection with the language model's response.

The inline assistant pulls its context from the assistant panel, allowing you to provide additional instructions or rules for code transformations.
