---
name: create-skill
description: Helps users create new agent skills for Zed. Use this when a user wants to create a skill, asks about SKILL.md structure, or wants to package reusable agent instructions.
---

# Creating a Zed Agent Skill

Use this skill when the user wants to create, edit, or understand agent skills in Zed.

## What is a Skill?

A skill is a reusable set of instructions that an agent can load on demand. Each skill lives in its own directory and is defined by a `SKILL.md` file with YAML frontmatter.

## Where Skills Live

Skills can be placed in two locations:

| Scope | Path | When to use |
|-------|------|-------------|
| Global | `~/.agents/skills/<skill-name>/SKILL.md` | Personal skills, available in all projects |
| Project-local | `<project>/.agents/skills/<skill-name>/SKILL.md` | Project-specific skills, shared with collaborators through version control |

Prefer project-local when the skill is specific to a repository. Prefer global when the skill is a personal workflow the user wants everywhere.

## SKILL.md Format

Every `SKILL.md` must start with YAML frontmatter between `---` delimiters:

```markdown
---
name: my-skill-name
description: A clear, specific description of what this skill does and when to use it.
---

# Skill Title

Instructions for the agent go here. Write them as if you're telling the agent
what to do when this skill is activated.
```

### Required Frontmatter Fields

- **`name`** (required): Must be 1–64 characters, lowercase alphanumeric with single-hyphen separators. Must match the containing directory name exactly. Regex: `^[a-z0-9]+(-[a-z0-9]+)*$`
- **`description`** (required): Must be 1–1024 characters. This is what the agent sees when deciding whether to use the skill — make it specific and actionable.

### Optional Frontmatter Fields

- **`disable-model-invocation`**: When set to `true`, the skill is hidden from the agent's automatic catalog. The user can still invoke it manually via the `/` slash command menu. Useful for skills that should only run when explicitly requested.

## Naming Rules

The skill name must:
- Be lowercase letters and numbers only, with single hyphens as separators
- Not start or end with `-`
- Not contain consecutive `--`
- Match the directory name that contains the `SKILL.md`

Good: `git-release`, `pr-review`, `rust-patterns`
Bad: `Git-Release`, `pr--review`, `-my-skill`, `my_skill`

## Writing Good Skill Instructions

The body of the SKILL.md (after the frontmatter) contains the instructions the agent will follow. Guidelines:

1. **Be direct**: Write instructions as if talking to the agent. "Do X", "Check Y", "Ask the user about Z".
2. **Be specific**: Include concrete file paths, commands, formats, and patterns.
3. **Include when-to-use guidance**: Help the agent understand the right context for this skill.
4. **Reference supporting files**: Skills can include additional files in their directory. Reference them with relative paths (e.g., `templates/component.tsx`). The agent can read these files when the skill is activated.
5. **Keep descriptions actionable**: The `description` field is the agent's primary signal for whether to load this skill. "Helps with code" is too vague. "Generate React components following the project's design system patterns" is specific.

## Supporting Files

A skill directory can contain additional files beyond `SKILL.md`:

```
~/.agents/skills/react-component/
├── SKILL.md
├── templates/
│   ├── component.tsx
│   └── test.tsx
└── examples/
    └── button.tsx
```

Reference these in the skill body. The agent can read them using the file path shown in the `<directory>` tag of the skill envelope.

## Step-by-Step: Creating a Skill

1. Decide on scope (global vs project-local) based on the user's needs.
2. Choose a descriptive, hyphenated name.
3. Create the directory structure.
4. Write the `SKILL.md` with frontmatter and instructions.
5. Optionally add supporting files (templates, examples, references).

After creating the skill, it will be automatically discovered by Zed's agent on the next conversation (no restart needed for global skills if the `~/.agents/skills/` directory already exists).
