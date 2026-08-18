---
title: Rules (Replaced by Skills)
description: Rules have been replaced by Skills and Instructions in Zed.
---

# Rules {#rules}

Rules have been replaced by [Skills](./skills.md) and [Instructions](./instructions.md).

> **Note:** Starting in Zed v1.4.0, on-demand Rules and the Rules Library have been replaced by [Skills](./skills.md). Skills are the recommended way to package reusable agent instructions.

Use [Skills](./skills.md) for reusable task instructions that can be invoked by name or selected by the model. Use [Instructions](./instructions.md) for always-on personal and project context.

## `.rules` Files {#rules-files}

Project `.rules` files remain supported as compatibility project instruction files. See [Instructions](./instructions.md#project-instructions).

Other instruction filenames are also supported for compatibility with other agents. The first matching file is used:

- `.rules`
- `.cursorrules`
- `.windsurfrules`
- `.clinerules`
- `.github/copilot-instructions.md`
- `AGENT.md`
- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`

## Migrating to Skills {#migrating-to-skills}

Existing Rules migrate automatically:

- Non-default Rules become global Skills in `~/.agents/skills/`, each with `disable-model-invocation: true`. They remain user-invocable by slash command or `@`-mention.
- Default Rules are appended to your global `AGENTS.md` file (`~/.config/zed/AGENTS.md` on macOS and Linux, `%APPDATA%\Zed\AGENTS.md` on Windows).
- Git commit prompt customizations are also appended to the global `AGENTS.md` file.

Rules Library content is not deleted, so downgrading to an earlier version of Zed leaves your Rules intact.
