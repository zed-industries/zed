---
title: Agent Skills - Zed
description: Extend Zed's AI agent with reusable, on-demand skill files for specialized tasks.
---

# Skills {#skills}

Skills are reusable instruction packages that give the agent specialized knowledge for specific tasks: test-driven development workflows, document processing, database integrations, or your team's internal coding standards.

A skill is a folder containing a `SKILL.md` file with metadata and instructions. The agent sees a catalog of all installed skills and can load one on demand, or you can invoke any skill directly from the message editor with a slash command.

## Adding Skills {#adding-skills}

### Create your own {#create-your-own}

Zed includes a built-in `create-skill` skill — invoke it with `/create-skill` and the agent walks you through the process.

You can also open the Skills Manager from the Agent Panel using {#kb agent::ManageSkills}, or by clicking `...` and selecting **Skills**. Outside the panel, use the {#action agent::OpenSkillCreator} action from the command palette, or click **Create Skill** on the **AI > Skills** settings page. The creator opens as a page in the settings window where you fill in the skill's name, description, body, and optionally toggle `disable-model-invocation`. The skill is saved to the scope of the settings file selected in the settings window — the **User** tab creates a global skill, while a **Project** tab creates a project-local skill — and the form shows exactly where the file will be written.

Lastly, it's also possible to add a skill through importing it from an existing GitHub Markdown file. Open the command palette and look for the {#action agent::CreateSkillFromUrl} action. If your clipboard contains a supported GitHub `.md` URL, Zed pre-fills and fetches it automatically.

See [Skill format](#skill-format) below for the full format reference.

### From the skills.sh Registry {#from-the-registry}

[skills.sh](https://skills.sh) is a community registry of open-source skills. You'll find skills for popular frameworks, tools, workflows, and more:

- [`find-skills`](https://skills.sh/vercel-labs/skills/find-skills): discover and install skills from the open ecosystem
- [`frontend-design`](https://skills.sh/anthropics/skills/frontend-design): production-grade frontend interfaces with design polish
- [`pdf`](https://skills.sh/anthropics/skills/pdf): PDF text extraction, merging, splitting, form filling, and OCR

To install a skill, copy the skill's folder into `~/.agents/skills/` for global use, or into your project's `.agents/skills/` folder for project-local use.

## Managing Skills {#managing-skills}

Open the Settings Editor (`Cmd+,` on macOS, `Ctrl+,` on Linux/Windows) and navigate to **AI > Skills**, or go directly to [agent.skills](zed://settings/agent.skills).

The **User** tab shows your global skills, and each **Project** tab shows the skills for that project.

For each skill you can:

- **Copy Share Link** — copies a `zed://skill` link that embeds the skill, ready to send to someone else (see [Sharing Skills](#sharing-skills))
- **Open** — opens the skill's `SKILL.md` file in the editor
- **Delete** — removes the skill folder from disk

In the skills page, you'll see a **Create Skill** button that opens the settings window, providing the ability to create a skill directly through the UI.

## Sharing Skills {#sharing-skills}

You can hand a skill to a teammate without hosting it anywhere. In the Skills settings page, click the **link** icon on a skill row to copy a `zed://skill?data=…` link to your clipboard.
The link is self-contained: it embeds the full `SKILL.md` contents (base64url-encoded), so the recipient doesn't need access to your project or any registry.

When someone opens that link (for example by pasting it into their browser or clicking it in a chat), Zed opens the "Create Skill" page in the settings window, pre-filled with the shared skill.
The recipient can review the name, description, and full body, choose a scope by selecting the **User** tab (global) or a **Project** tab, and click **Save** to install it.
Nothing is written to disk until they explicitly save, so a shared link can never silently install instructions into someone's agent.

## Using Skills {#using-skills}

By default, the agent picks up skills autonomously. It sees a catalog of every installed skill (name and description) in its system prompt, and calls the `skill` tool when a task matches a skill's description.

When the agent invokes a skill you created or installed, Zed prompts you to allow or deny it, using the same permission flow as other tools. Skills built into Zed do not prompt. You can set per-skill defaults in [Tool Permissions](./tool-permissions.md) so you're not prompted for skills you always trust.

### Manual Invocation {#manual-invocation}

You can also load a skill manually:

- **Slash command**: type `/` in the message editor and select a skill by name
- **@-mention**: type `@skill` in the message editor and select a skill from the completion menu

Both inject the skill's instructions as context. The loaded skill appears as a crease button in the thread, which you can click to open the skill file.

### Preventing Autonomous Invocation {#disable-model-invocation}

Add `disable-model-invocation: true` to a skill's frontmatter to stop the agent from picking it up autonomously.
The skill still appears as a slash command, so you stay in control of when it runs.

This is useful for workflows you don't want the agent triggering automatically, like deploy or release procedures.

```yaml
---
name: deploy
description: Deploy the current branch to production.
disable-model-invocation: true
---
```

## Skill Format {#skill-format}

### Folder Structure {#folder-structure}

A skill is a named folder containing a `SKILL.md` file:

```
my-skill/
├── SKILL.md          # Required: metadata and instructions
├── scripts/          # Optional: scripts the agent can run
├── references/       # Optional: additional documentation
└── assets/           # Optional: templates and static files
```

By convention, the folder name should match the `name` field in `SKILL.md`.

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

#### Frontmatter Fields {#frontmatter-fields}

| Field                      | Required | Description                                                                                                                       |
| -------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Yes      | Lowercase letters, numbers, and hyphens only. Max 64 characters. Should match the folder name.                                    |
| `description`              | Yes      | What the skill does and when to use it. Keep it under 1024 bytes; skills with longer descriptions still load, but with a warning. |
| `disable-model-invocation` | No       | Set to `true` to hide from the agent's catalog (invocable via slash command or @-mention only).                                   |

> **Tip:** Write descriptions that help the agent recognize when a skill is relevant. Include specific task types and trigger phrases: "Use when handling PDFs, extracting text, or filling forms" is better than "Helps with PDFs."

We plan to include other fields promoted by [the Agent Skills specification](https://agentskills.io/specification) in the near future.

#### Name Validation {#name-validation}

The `name` field must:

- Contain only lowercase letters (`a-z`), numbers, and hyphens
- Not start or end with a hyphen
- Not contain consecutive hyphens (`--`)
- Be 1 to 64 characters

Skills with invalid names fail to load and surface an error in the UI.

### Bundled Resources {#bundled-resources}

Keep the body of `SKILL.md` under 500 lines. Move detailed material to reference files and link to them from the body:

```markdown
See [reference guide](references/REFERENCE.md) for complete API details.

Run the extraction script:
scripts/extract.py
```

The agent loads these files on demand using the `read_file` and `list_directory` tools. Global skills under `~/.agents/skills/` are accessible to the agent even though they're outside your project.

### Writing Effective Instructions {#writing-instructions}

Skills use [progressive disclosure](https://agentskills.io/specification#progressive-disclosure): the agent sees only the name and description until it activates a skill, then loads the full body. Structure your skill to take advantage of this:

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

### Project-local Skills and Trust {#project-local-trust}

Project-local skills only load from [trusted worktrees](../worktree-trust.md). Skills from a freshly cloned or untrusted project are excluded from the catalog and slash commands until you grant trust.

This prevents a malicious project from injecting instructions into your agent's system prompt before you've reviewed what the project ships.

### Override Behavior {#override-behavior}

If a global and a project-local skill share the same name, the project-local skill takes precedence. This lets a project customize or replace a global skill for its own context.

### Editing Skill Files {#editing-skill-files}

The agent cannot edit `SKILL.md` files or their bundled resources without your explicit authorization, even in a trusted project. This prevents a compromised conversation from modifying the skills that govern future conversations.

## Agent Path Boundaries {#agent-path-boundaries}

Zed Skills apply to the Zed Agent. External Agents and Terminal Threads may have their own native skills, prompts, or instruction systems. Configure those in the External Agent or CLI.

## Limitations {#limitations}

- **Flat layout only.** Skills must be direct children of the skills root. Nested folders like `~/.agents/skills/group/my-skill/` are not discovered.
- **50KB catalog budget.** The total size of all skill names and descriptions is capped at 50KB. Skills that don't fit are dropped from the catalog with a warning in the UI. Keep descriptions concise.
- **No remote registry.** Zed does not discover or load skills from remote locations at runtime, and custom search paths are not supported. (You can still import a skill once from a GitHub URL — see [Create your own](#create-your-own).) Skills are loaded from `~/.agents/skills/` and `<worktree>/.agents/skills/` only. Use a symlink if you need to point at another location.
- **Live reload.** Adding, removing, or editing a `SKILL.md` takes effect immediately without restarting your session. Changes to a skill's `name` or `description` invalidate the model's prompt cache for the current session.

## See also

- [Agent Panel](./agent-panel.md)
- [Tool Permissions](./tool-permissions.md)
- [Agent Skills specification](https://agentskills.io/specification)
