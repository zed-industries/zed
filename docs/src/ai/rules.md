---
title: AI Rules in Zed - .rules, .cursorrules, CLAUDE.md
description: Configure AI behavior in Zed with .rules files, .cursorrules, CLAUDE.md, AGENTS.md, and the Rules Library for project-level instructions.
---

# Rules {#rules}

Rules are prompts that can be inserted either automatically at the beginning of each [Agent Panel](./agent-panel.md) interaction, through `.rules` files available in your project's file tree, or on-demand, through @-mentioning, via the Rules Library.

> **Note:** Starting in Zed v1.4.0, on-demand rules (and the rules library) have been replaced by [Skills](./skills.md). Skills are the recommended way to package reusable agent instructions. Learn more about [the rules -> skills migration](#migrating-to-skills).

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

> **Note:** Starting in Zed v1.4.0, the rules library has been replaced by [Skills](./skills.md). Skills are the recommended way to package reusable agent instructions. Learn more about [the rules -> skills migration](#migrating-to-skills).

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

## Migrating to Skills {#migrating-to-skills}

As of Zed v1.4.0, your existing Rules are migrated to Skills automatically:

- **Non-default Rules** become global skills in `~/.agents/skills/`, each with `disable-model-invocation: true`. They remain user-invocable via `/skill-name` or `@`-mention.
- **Default Rules** are appended to your global `AGENTS.md` file (`~/.config/zed/AGENTS.md` on macOS and Linux, `%APPDATA%\Zed\AGENTS.md` on Windows), preserving their behavior of being included in every conversation.
- **Git Commit** prompt customizations are also appended to the global `AGENTS.md` file.

Lastly, note that all of the content you had available in the Rules Library hasn't been deleted, so downgrading to an earlier version of Zed leaves your Rules intact.
