---
title: Agent Skills - Zed
description: Extend Zed's AI agent with reusable, on-demand skill files for specialized tasks.
---

# Skills {#skills}

Skills are reusable instruction packages that give the agent specialized knowledge for specific tasks: test-driven development workflows, document processing, database integrations, or your team's internal coding standards.

A skill is a folder containing a `SKILL.md` file with metadata and instructions. The agent sees a catalog of all installed skills and can load one on demand, or you can invoke any skill directly from the message editor with a slash command.

## Finding Skills {#finding-skills}

[skills.sh](https://skills.sh) is a community registry of open-source skills. You'll find skills for popular frameworks, tools, workflows, and more:

- [`find-skills`](https://skills.sh/vercel-labs/skills/find-skills): discover and install skills from the open ecosystem
- [`frontend-design`](https://skills.sh/anthropics/skills/frontend-design): production-grade frontend interfaces with design polish
- [`vercel-react-best-practices`](https://skills.sh/vercel-labs/agent-skills/vercel-react-best-practices): React and Next.js performance patterns across 8 categories
- [`web-design-guidelines`](https://skills.sh/vercel-labs/agent-skills/web-design-guidelines): audit UI code for design, accessibility, and UX compliance
- [`pdf`](https://skills.sh/anthropics/skills/pdf): PDF text extraction, merging, splitting, form filling, and OCR

## Adding Skills {#adding-skills}

### From the registry {#from-the-registry}

Skills are folders on disk. To install a skill from the registry, copy or clone its folder into your global or project-local skills folder.

For example, to install the `frontend-design` skill from GitHub globally:

```sh
cd ~/.agents/skills
git clone --filter=blob:none --sparse https://github.com/anthropics/skills
cd skills
git sparse-checkout set frontend-design
```

For a project-local install, do the same inside your project's `.agents/skills/` folder.

### Create your own {#create-your-own}

Zed includes a built-in `create-skill` skill that guides the agent through creating a new skill. Invoke it with `/create-skill`, or the agent will pick it up automatically when you ask it to help create a skill.

See [Skill format](#skill-format) below for the folder structure and `SKILL.md` reference.

## Using Skills {#using-skills}

By default, the agent loads skills autonomously. It sees a catalog of every installed skill (name and description) in its system prompt, and calls the `skill` tool when a task matches a skill's description.

When the agent invokes a skill, Zed prompts you to allow or deny it, using the same permission flow as other tools. You can set per-skill defaults in [Tool Permissions](./tool-permissions.md) so you're not prompted for skills you always trust.

### Manual invocation {#manual-invocation}

You can also load a skill manually:

- **Slash command**: type `/` in the message editor and select a skill by name
- **@-mention**: type `@skill` in the message editor and select a skill from the completion menu

Both inject the skill's instructions as context. The loaded skill appears as a collapsible crease in the thread. Click it to open the skill file.

### Preventing autonomous invocation {#disable-model-invocation}

Add `disable-model-invocation: true` to a skill's frontmatter to hide it from the agent's catalog entirely. The skill still appears as a slash command, so you stay in control of when it runs.

This is useful for workflows you don't want the agent triggering automatically, like deploy or release procedures.

```yaml
---
name: deploy
description: Deploy the current branch to production.
disable-model-invocation: true
---
```

## Skill Format {#skill-format}

### Folder structure {#folder-structure}

A skill is a named folder containing a `SKILL.md` file:

```
my-skill/
├── SKILL.md          # Required: metadata and instructions
├── scripts/          # Optional: scripts the agent can run
├── references/       # Optional: additional documentation
└── assets/           # Optional: templates and static files
```

The folder name must match the `name` field in `SKILL.md`.

### SKILL.md format {#skill-md-format}

`SKILL.md` starts with YAML frontmatter, followed by Markdown instructions.

**Minimal example:**

```markdown
---
name: my-skill
description: What this skill does and when to use it.
---

## Instructions

Step-by-step instructions for the agent...
```

#### Frontmatter fields {#frontmatter-fields}

| Field                      | Required | Description                                                                                  |
| -------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| `name`                     | Yes      | Lowercase letters, numbers, and hyphens only. Max 64 characters. Must match the folder name. |
| `description`              | Yes      | What the skill does and when to use it. Max 1024 characters.                                 |
| `disable-model-invocation` | No       | Set to `true` to hide from the agent's catalog (slash command only).                         |

> **Tip:** Write descriptions that help the agent recognize when a skill is relevant. Include specific task types and trigger phrases: "Use when handling PDFs, extracting text, or filling forms" is better than "Helps with PDFs."

#### Name validation {#name-validation}

The `name` field must:

- Contain only lowercase letters (`a-z`), numbers, and hyphens
- Not start or end with a hyphen
- Not contain consecutive hyphens (`--`)
- Be 1 to 64 characters

Skills with invalid names fail to load and surface an error in the UI.

### Bundled resources {#bundled-resources}

Keep the body of `SKILL.md` under 500 lines. Move detailed material to reference files and link to them from the body:

```markdown
See [reference guide](references/REFERENCE.md) for complete API details.

Run the extraction script:
scripts/extract.py
```

The agent loads these files on demand using the `read_file` and `list_directory` tools. Global skills under `~/.agents/skills/` are accessible to the agent even though they're outside your project.

### Writing effective instructions {#writing-instructions}

Skills use [progressive disclosure](https://agentskills.io/specification): the agent sees only the name and description until it activates a skill, then loads the full body. Structure your skill to take advantage of this:

- Put the most important instructions near the top of the body
- Keep `SKILL.md` under 500 lines; move detailed references to `references/`
- Scripts that the agent needs to run go in `scripts/`

See the [Agent Skills specification](https://agentskills.io/specification) for the full format reference.

## Where Skills Live {#where-skills-live}

Zed loads skills from two locations:

| Scope         | Path                         | When it applies          |
| ------------- | ---------------------------- | ------------------------ |
| Global        | `~/.agents/skills/`          | Every project            |
| Project-local | `<worktree>/.agents/skills/` | Only the current project |

Each skill is a direct child of the skills root. Nesting skills inside subfolders is not supported.

### Project-local skills and trust {#project-local-trust}

Project-local skills only load from [trusted worktrees](../worktree-trust.md). Skills from a freshly cloned or untrusted project are excluded from the catalog and slash commands until you grant trust.

This prevents a malicious project from injecting instructions into your agent's system prompt before you've reviewed what the project ships.

### Override behavior {#override-behavior}

If a global and a project-local skill share the same name, the project-local skill takes precedence. This lets a project customize or replace a global skill for its own context.

### Editing skill files {#editing-skill-files}

The agent cannot edit `SKILL.md` files or their bundled resources without your explicit authorization, even in a trusted project. This prevents a compromised conversation from modifying the skills that govern future conversations.

## Limitations {#limitations}

- **Flat layout only.** Skills must be direct children of the skills root. Nested folders like `~/.agents/skills/group/my-skill/` are not discovered.
- **50KB catalog budget.** The total size of all skill names and descriptions is capped at 50KB. Skills that don't fit are dropped from the catalog with a warning in the UI. Keep descriptions concise.
- **No remote registry.** Zed does not fetch skills from URLs or support custom search paths. Skills come from `~/.agents/skills/` and `<worktree>/.agents/skills/` only. Use a symlink if you need to point at another location.
- **Live reload.** Adding, removing, or editing a `SKILL.md` takes effect immediately without restarting your session. Changes to a skill's `name` or `description` invalidate the model's prompt cache for the current session.

## See also

- [Agent Panel](./agent-panel.md)
- [Tool Permissions](./tool-permissions.md)
- [Agent Skills specification](https://agentskills.io/specification)
