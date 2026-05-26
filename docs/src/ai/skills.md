---
title: Agent Skills - Zed
description: Use Zed Agent Skills for reusable task instructions with SKILL.md files, slash commands, and model-invoked skill loading.
---

# Skills

Skills are reusable task instructions for the Zed Agent. A skill is a folder containing a `SKILL.md` file and optional bundled resources such as `scripts/`, `references/`, and `assets/`.

Skills are different from [Instructions](./instructions.md). Use skills for reusable workflows that should be invoked by name or selected by the model. Use instructions for always-on personal or project context.

## Locations {#locations}

Zed loads skills from two locations:

| Scope         | Location                                    | Applies to                   |
| ------------- | ------------------------------------------- | ---------------------------- |
| Global        | `~/.agents/skills/<name>/SKILL.md`          | Every project                |
| Project-local | `<worktree>/.agents/skills/<name>/SKILL.md` | The current trusted worktree |

Project-local skills only load from [trusted worktrees](../worktree-trust.md).

## Skill File Format {#skill-file-format}

Each skill needs `name` and `description` frontmatter.

```markdown
---
name: code-review
description: Review code changes for correctness, test coverage, and maintainability.
---

Review the current changes. Prioritize correctness issues, regressions, and missing tests.
```

Skill names must be lowercase letters, numbers, and hyphens.

## Invocation {#invocation}

Skills can be used in two ways:

- You invoke a skill with a slash command such as `/code-review`.
- The model invokes a skill with the `skill` tool when the user's request matches the skill description.

Model-invoked skills use the normal [Tool Permissions](./tool-permissions.md) flow. User-invoked slash commands do not prompt again because you explicitly invoked the skill.

## Slash-Only Skills {#slash-only-skills}

Set `disable-model-invocation: true` when a skill should be available as a slash command but hidden from the model's skill catalog.

```markdown
---
name: release-checklist
description: Run the release checklist.
disable-model-invocation: true
---

Follow the release checklist for this repository.
```

Use this for workflows where you want the user to decide when the skill runs.

## Security {#security}

Skills are persistent instructions. Agent edits to `SKILL.md` files and bundled skill resources require explicit authorization.

Project-local skills are only loaded from [trusted worktrees](../worktree-trust.md). This prevents a newly cloned project from injecting skill descriptions into the Zed Agent before you trust the worktree.

## Agent Path Boundaries {#agent-path-boundaries}

Zed Skills apply to the Zed Agent. External Agents and Terminal Threads may have their own native skills, prompts, or instruction systems. Configure those in the External Agent or CLI.

## Migrating from Rules {#migrating-from-rules}

Rules have been replaced by Skills for reusable task instructions.

- Non-default Rules migrate to global Skills in `~/.agents/skills/` with `disable-model-invocation: true`.
- Default Rules and customized built-in prompts migrate to personal `AGENTS.md`.
- See [Instructions](./instructions.md) for always-on context files.
