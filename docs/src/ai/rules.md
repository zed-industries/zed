---
title: AI Rules in Zed - .rules, .cursorrules, CLAUDE.md
description: Configure AI behavior in Zed with .rules files, .cursorrules, CLAUDE.md, AGENTS.md, and the Rules Library for project-level instructions.
---

# Using Rules {#using-rules}

Rules are prompts that can be inserted either automatically at the beginning of each [Agent Panel](./agent-panel.md) interaction, through `.rules` files available in your project's file tree, or on-demand, through @-mentioning, via the Rules Library.

## `.rules` files

Zed supports including `.rules` files at the root of a project's file tree, and they act as project-level instructions that are auto-included in all of your interactions with the Agent Panel.

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

The Rules Library is an interface for writing and managing rules.
It's a full editor with syntax highlighting and all standard keybindings.

You can also use the inline assistant right in the rules editor, allowing you to get quick LLM support for writing rules.

### Opening the Rules Library

1. Open the Agent Panel.
2. Click on the Agent menu (`...`) in the top right corner.
3. Select `Rules...` from the dropdown.

You can also open it by running the {#action agent::OpenRulesLibrary} action or through the {#kb agent::OpenRulesLibrary} keybinding.

### Managing Rules

Once a rules file is selected, you can edit it directly in the built-in editor.
Its title can be changed from the editor title bar as well.

Rules can be duplicated, deleted, or added to the default rules using the buttons in the rules editor.

### Creating Rules {#creating-rules}

To create a rule file, simply open the `Rules Library` and click the `+` button.
Rules files are stored locally and can be accessed from the library at any time.

For guidance on writing effective rules:

- [Anthropic: Prompt Engineering](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/overview)
- [OpenAI: Prompt Engineering](https://platform.openai.com/docs/guides/prompt-engineering)

### Using Rules

You can @-mention every rule created through the Rules Library.
This allows you to quickly reach for reusable prompts, saving the time to type them out every time you need to use them.

#### Default Rules {#default-rules}

All rules in the Rules Library can be set as a default rule, which means they’re automatically inserted into context for every new Agent Panel interaction.

You can set any rule as the default by clicking the paper clip icon button in the top-right of the rule editor in the Rules Library.

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
