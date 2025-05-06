# Text Threads

Text threads in the agent panel functions similarly to any other editor. You can use custom key bindings and work with multiple cursors, allowing for seamless transitions between coding and engaging in discussions with the language models.

However, the text threads differ with the inclusion of message blocks. These blocks serve as containers for text that correspond to different roles within the context. These roles include:

- `You`
- `Assistant`
- `System`

To begin, type a message in a `You` block.

<!-- todo! update photos in this section, if we are keeping it -->

![Asking a question](https://zed.dev/img/assistant/ask-a-question.png)

As you type, the remaining tokens count for the selected model is updated.

Inserting text from an editor is as simple as highlighting the text and running `assistant: quote selection` ({#kb assistant::QuoteSelection}); Zed will wrap it in a fenced code block if it is code.

![Quoting a selection](https://zed.dev/img/assistant/quoting-a-selection.png)

To submit a message, use {#kb assistant::Assist}(`assistant: assist`). Unlike normal threads, where pressing <kbd>enter</kbd> would submit the message, in text threads, our goal is to make it feel as close to a regular editor as possible. So, pressing {#kb editor::Newline} simply inserts a new line.

After submitting a message, the response will be streamed below, in an `Assistant` message block.

![Receiving an answer](https://zed.dev/img/assistant/receiving-an-answer.png)

The stream can be canceled at any point with <kbd>escape</kbd>. This is useful if you realize early on that the response is not what you were looking for.

If you want to start a new conversation at any time, you can hit <kbd>cmd-n|ctrl-n</kbd> or use the `New Chat` menu option in the hamburger menu at the top left of the panel.

Simple back-and-forth conversations work well with the text threads. However, there may come a time when you want to modify the previous text in the conversation and steer it in a different direction.

## Editing a Context {#edit-context}

> **Note**: Wondering about Context vs. Conversation? [Read more here](./contexts.md).

Text threads give you the flexibility to have control over the context. You can freely edit any previous text, including the responses from the LLM. If you want to remove a message block entirely, simply place your cursor at the beginning of the block and use the `delete` key. A typical workflow might involve making edits and adjustments throughout the context to refine your inquiry or provide additional information. Here's an example:

1. Write text in a `You` block.
2. Submit the message with {#kb assistant::Assist}.
3. Receive an `Assistant` response that doesn't meet your expectations.
4. Cancel the response with <kbd>escape</kbd>.
5. Erase the content of the `Assistant` message block and remove the block entirely.
6. Add additional context to your original message.
7. Submit the message with {#kb assistant::Assist}.

Being able to edit previous messages gives you control over how tokens are used. You don't need to start up a new chats to correct a mistake or to add additional information, and you don't have to waste tokens by submitting follow-up corrections.

> **Note**: The act of editing past messages is often referred to as "Rewriting History" in the context of the language models.

Some additional points to keep in mind:

- You can cycle the role of a message block by clicking on the role, which is useful when you receive a response in an `Assistant` block that you want to edit and send back up as a `You` block.

## Commands Overview {#commands}

Slash commands enhance the assistant's capabilities. Begin by typing a `/` at the beginning of the line to see a list of available commands:

- `/default`: Inserts the default prompt into the context
- `/diagnostics`: Injects errors reported by the project's language server into the context
- `/fetch`: Fetches the content of a webpage and inserts it into the context
- `/file`: Inserts a single file or a directory of files into the context
- `/now`: Inserts the current date and time into the context
- `/prompt`: Adds a custom-configured prompt to the context ([see Rules Library](./rules.md#rules-library))
- `/symbols`: Inserts the current tab's active symbols into the context
- `/tab`: Inserts the content of the active tab or all open tabs into the context
- `/terminal`: Inserts a select number of lines of output from the terminal
- `/selection`: Inserts the selected text into the context

### Other Commands:

- `/search`: Performs semantic search for content in your project based on natural language
  - Not generally available yet, but some users may have access to it.

> **Note:** Remember, commands are only evaluated when the context is created or when the command is inserted, so a command like `/now` won't continuously update, or `/file` commands won't keep their contents up to date.

#### `/default`

Read more about `/default` in the [Rules: Editing the Default Rules](./rules.md#default-rules) section.

Usage: `/default`

#### `/diagnostics`

The `/diagnostics` command injects errors reported by the project's language server into the context. This is useful for getting an overview of current issues in your project.

Usage: `/diagnostics [--include-warnings] [path]`

- `--include-warnings`: Optional flag to include warnings in addition to errors.
- `path`: Optional path to limit diagnostics to a specific file or directory.

#### `/file`

The `/file` command inserts the content of a single file or a directory of files into the context. This allows you to reference specific parts of your project in your conversation with the assistant.

Usage: `/file <path>`

You can use glob patterns to match multiple files or directories.

Examples:

- `/file src/index.js` - Inserts the content of `src/index.js` into the context.
- `/file src/*.js` - Inserts the content of all `.js` files in the `src` directory.
- `/file src` - Inserts the content of all files in the `src` directory.

#### `/now`

The `/now` command inserts the current date and time into the context. This can be useful letting the language model know the current time (and by extension, how old their current knowledge base is).

Usage: `/now`

#### `/prompt`

The `/prompt` command inserts a prompt from the prompt library into the context. It can also be used to nest prompts within prompts.

Usage: `/prompt <prompt_name>`

Related: `/default`

#### `/symbols`

The `/symbols` command inserts the active symbols (functions, classes, etc.) from the current tab into the context. This is useful for getting an overview of the structure of the current file.

Usage: `/symbols`

#### `/tab`

The `/tab` command inserts the content of the active tab or all open tabs into the context. This allows you to reference the content you're currently working on.

Usage: `/tab [tab_name|all]`

- `tab_name`: Optional name of a specific tab to insert.
- `all`: Insert content from all open tabs.

Examples:

- `/tab` - Inserts the content of the active tab.
- `/tab "index.js"` - Inserts the content of the tab named "index.js".
- `/tab all` - Inserts the content of all open tabs.

#### `/terminal`

The `/terminal` command inserts a select number of lines of output from the terminal into the context. This is useful for referencing recent command outputs or logs.

Usage: `/terminal [<number>]`

- `<number>`: Optional parameter to specify the number of lines to insert (default is a 50).

#### `/selection`

The `/selection` command inserts the selected text in the editor into the context. This is useful for referencing specific parts of your code.

This is equivalent to the `assistant: quote selection` command ({#kb assistant::QuoteSelection}).

Usage: `/selection`

## Commands in the Rules Library (previously known as Prompt Libary) {#slash-commands-in-rules}

[Commands](#commands) can be used in rules to insert dynamic content or perform actions. For example, if you want to create a rule where it is important for the model to know the date, you can use the `/now` command to insert the current date.

> **Warn:** Slash commands in rules **only** work when they are used in text threads. Using them in non-text threads is not supported.

> **Note:** Slash commands in rules **must** be on their own line.

See the [list of commands](#commands) above for more information on commands, and what slash commands are available.

### Example:

```plaintext
You are an expert Rust engineer. The user has asked you to review their project and answer some questions.

Here is some information about their project:

/file Cargo.toml
```

In the above example, the `@file` command is used to insert the contents of the `Cargo.toml` file (or all `Cargo.toml` files present in the project) into the rule.

## Nesting Rules

Similar to adding rules to the default rules, you can nest rules within other rules with the `/prompt` command (only supported in Text Threads currently).

You might want to nest rules to:

- Create templates on the fly
- Break collections like docs or references into smaller, mix-and-matchable parts
- Create variants of a similar rule (e.g., `Async Rust - Tokio` vs. `Async Rust - Async-std`)

### Example:

```plaintext
Title: Zed-Flavored Rust

## About Zed

/prompt Zed: Zed (a rule about what Zed is)

## Rust - Zed Style

/prompt Rust: Async - Async-std (zed doesn't use tokio)
/prompt Rust: Zed-style Crates (we have some unique conventions)
/prompt Rust - Workspace deps (bias towards reusing deps from the workspace)
```

_The text in parentheses above are comments and are not part of the rule._

> **Note:** While you technically _can_ nest a rule within itself, we wouldn't recommend it (in the strongest of terms.) Use at your own risk!

By using nested rules, you can create modular and reusable rule components that can be combined in various ways to suit different scenarios.

> **Note:** When using slash commands to bring in additional context, the injected content can be edited directly inline in the text thread—edits here will not propagate to the saved rules.

## Extensibility

Additional slash commands can be provided by extensions.

See [Extension: Slash Commands](../extensions/slash-commands.md) to learn how to create your own.

## Advanced Concepts

### Rule Templates {#rule-templates}

Zed uses rule templates to power internal assistant features, like the terminal assistant, or the content rules used in the inline assistant.

Zed has the following internal rule templates:

- `content_prompt.hbs`: Used for generating content in the editor.
- `terminal_assistant_prompt.hbs`: Used for the terminal assistant feature.
- `suggest_edits.hbs`: Used for generating the model instructions for the XML Suggest Edits should return.

At this point it is unknown if we will expand templates further to be user-creatable.

### Overriding Templates

> **Note:** It is not recommended to override templates unless you know what you are doing. Editing templates will break your assistant if done incorrectly.

Zed allows you to override the default rules used for various assistant features by placing custom Handlebars (.hbs) templates in your `~/.config/zed/prompt_overrides` directory.

The following templates can be overridden:

1. [`content_prompt.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/content_prompt.hbs): Used for generating content in the editor.

2. [`terminal_assistant_prompt.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/terminal_assistant_prompt.hbs): Used for the terminal assistant feature.

3. [`suggest_edits.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/suggest_edits.hbs): Used for generating the model instructions for the XML Suggest Edits should return.

> **Note:** Be sure you want to override these, as you'll miss out on iteration on our built-in features. This should be primarily used when developing Zed.

You can customize these templates to better suit your needs while maintaining the core structure and variables used by Zed. Zed will automatically reload your prompt overrides when they change on disk.

Consult Zed's [assets/prompts](https://github.com/zed-industries/zed/tree/main/assets/prompts) directory for current versions you can play with.

### History {#history}

After you submit your first message in a text thread, a name for your context is generated by the language model, and the context is automatically saved to your file system in

- `~/.config/zed/conversations` (macOS)
- `~/.local/share/zed/conversations` (Linux)
- `%LocalAppData%\Zed\conversations` (Windows)

You can access and load previous contexts by clicking on the history button in the top-left corner of the agent panel.

![Viewing assistant history](https://zed.dev/img/assistant/assistant-history.png)
