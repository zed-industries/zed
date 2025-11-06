# Using Rules {#using-rules}

A rule is essentially a prompt that is inserted at the beginning of each interaction with the Agent.
Currently, Zed supports adding rules through files inserted directly in the worktree or through the Rules Library, which allows you to store multiple rules for constant or on-demand usage.

## `.rules` files

Zed supports including `.rules` files at the top level of worktrees, and they act as project-level instructions that are included in all of your interactions with the Agent Panel.
Other names for this file are also supported for compatibility with other agents, but note that the first file which matches in this list will be used:

- `.rules`
- `.cursorrules`
- `.windsurfrules`
- `.clinerules`
- `.github/copilot-instructions.md`
- `AGENT.md`
- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`

## Rules Library {#rules-library}

The Rules Library is an interface for writing and managing rules. Like other text-driven UIs in Zed, it is a full editor with syntax highlighting, keyboard shortcuts, etc.

You can use the inline assistant right in the rules editor, allowing you to automate and rewrite rules.

### Opening the Rules Library

1. Open the Agent Panel.
2. Click on the Agent menu (`...`) in the top right corner.
3. Select `Rules...` from the dropdown.

You can also reach it by running {#action agent::OpenRulesLibrary} in the command palette or through the {#kb agent::OpenRulesLibrary} keybinding.

### Managing Rules

Once a rules file is selected, you can edit it directly in the built-in editor. Its title can be changed from the editor title bar as well.

Rules can be duplicated, deleted, or added to the default rules using the buttons in the rules editor.

### Creating Rules {#creating-rules}

To create a rule file, simply open the `Rules Library` and click the `+` button. Rules files are stored locally and can be accessed from the library at any time.

Having a series of rules files specifically tailored to prompt engineering can also help you write consistent and effective rules.

Here are a couple of helpful resources for writing better rules:

- [Anthropic: Prompt Engineering](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/overview)
- [OpenAI: Prompt Engineering](https://platform.openai.com/docs/guides/prompt-engineering)

### Editing the Default Rules {#default-rules}

Zed allows you to customize the default rules used when interacting with LLMs.
Or to be more precise, it uses a series of rules that are combined to form the default rules.

Default rules are included in the context of every new thread automatically.
You can also manually add other rules (that are not flagged as default) as context using the `@rule` command.

## Migrating from Prompt Library

Previously, the Rules Library was called the "Prompt Library".
The new rules system replaces the Prompt Library except in a few specific cases, which are outlined below.

### Slash Commands in Rules

Previously, it was possible to use slash commands (now @-mentions) in custom prompts (now rules).
There is currently no support for using @-mentions in rules files, however, slash commands are supported in rules files when used with text threads.
See the documentation for using [slash commands in rules](./text-threads.md#slash-commands-in-rules) for more information.

### Prompt templates

Zed maintains backwards compatibility with its original template system, which allows you to customize prompts used throughout the application, including the inline assistant.
While the Rules Library is now the primary way to manage prompts, you can still use these legacy templates to override default prompts.
For more details, see the [Rules Templates](./text-threads.md#rule-templates) section under [Text Threads](./text-threads.md).
