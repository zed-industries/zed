# Using Rules {#using-rules}

A rule is essentially a prompt that is inserted at the beginning of each interaction with the Agent.
Currently, Zed supports adding rules through files inserted directly in the worktree or through the Rules Library, which allows you to store multiple rules for constant or on-demand usage.

## `.rules` files

Zed supports including `.rules` files at the top level of worktrees, and they act as project-level instructions that are included in all of your interactions with the Agent Panel.

### Single Rules File

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

### Multiple Rules Files with `.rules.d`

Zed also supports a `.rules.d` directory at the top level of worktrees. When this directory exists, **all** markdown (`.md`) and text (`.txt`) files within it are read and combined with any existing `.rules` file as project-level instructions. This allows you to:

- **Organize rules by topic**: Split large rule sets into multiple files (e.g., `coding-style.md`, `testing-guidelines.md`, `architecture.md`)
- **Share common rules**: Reuse rule files across multiple projects
- **Manage rules with Git**: Track individual rule files separately for clearer history

Files in `.rules.d` are read in alphabetical order and concatenated with blank lines between them. If both a `.rules` file and a `.rules.d` directory exist, the `.rules` file content is included first, followed by the combined content from `.rules.d`.

Example structure:
```
project-root/
├── .rules.d/
│   ├── 01-coding-style.md
│   ├── 02-testing.md
│   └── 03-documentation.md
└── src/
```

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

To create a rule file, simply open the `Rules Library` and click the `+` button. Rules files are stored locally as markdown files in `~/.config/zed/prompts/rules/` (on Linux/FreeBSD and macOS) or `%APPDATA%\Zed\prompts\rules\` (on Windows), and can be accessed from the library at any time.

#### File-Based Storage

Global rules are stored as markdown files with YAML frontmatter in the rules directory. This allows you to:

- **Track rules with Git**: Since rules are plain text files, you can version control them
- **Share rules easily**: Copy rule files between machines or share with team members
- **Edit externally**: Edit rules in any text editor - changes are automatically detected and reloaded
- **Organize with subdirectories**: Rules can be organized into subdirectories within the rules folder
- **Backup and sync**: Use your preferred backup or sync solution for rule files

Each rule file follows this format:

```markdown
---
id: 550e8400-e29b-41d4-a716-446655440000
title: My Rule
default: true
saved_at: 2024-01-01T00:00:00Z
---

Rule content goes here...
```

The frontmatter contains:
- `id`: A unique UUID for the rule
- `title`: Optional display name for the rule
- `default`: Whether this rule is included in all new threads by default
- `saved_at`: Timestamp of when the rule was last saved

When you create or edit rules through the Rules Library UI, these files are automatically managed for you. You can also create rule files manually by placing `.md` files anywhere in the rules directory (including subdirectories) - they will be automatically detected and loaded.

#### Automatic File Watching

The Rules Library automatically watches for changes to rule files on disk. When you edit a rule file externally (in another text editor or via Git pull), the changes are immediately reflected in Zed without needing to reload.

Having a series of rules files specifically tailored to prompt engineering can also help you write consistent and effective rules. Since rules are now file-based, you can organize them in git repositories or sync them across machines.

Here are a couple of helpful resources for writing better rules:

- [Anthropic: Prompt Engineering](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/overview)
- [OpenAI: Prompt Engineering](https://platform.openai.com/docs/guides/prompt-engineering)

### Editing the Default Rules {#default-rules}

Zed allows you to customize the default rules used when interacting with LLMs.
Or to be more precise, it uses a series of rules that are combined to form the default rules.

Default rules are included in the context of every new thread automatically.
You can also manually add other rules (that are not flagged as default) as context using the `@rule` command.

## Migrating from Prompt Library

Previously, the Rules Library was called the "Prompt Library" and stored rules in a database. The new rules system uses file-based storage exclusively and replaces the Prompt Library except in a few specific cases, which are outlined below.

### Automatic Migration

When you first launch Zed with the file-based rules system, any existing rules stored in the database will be automatically migrated to markdown files in the rules directory. This is a one-time migration that:

1. Detects the old database if it exists
2. Migrates all user rules to markdown files
3. Preserves all metadata (title, default status, timestamps)
4. Moves the old database to a `.bak` backup directory after successful migration

The migration happens automatically in the background and you'll see log messages indicating the progress. After migration, all rules will be loaded from files.

### Slash Commands in Rules

Previously, it was possible to use slash commands (now @-mentions) in custom prompts (now rules).
There is currently no support for using @-mentions in rules files, however, slash commands are supported in rules files when used with text threads.
See the documentation for using [slash commands in rules](./text-threads.md#slash-commands-in-rules) for more information.

### Prompt templates

Zed maintains backwards compatibility with its original template system, which allows you to customize prompts used throughout the application, including the inline assistant.
While the Rules Library is now the primary way to manage prompts, you can still use these legacy templates to override default prompts.
For more details, see the [Rules Templates](./text-threads.md#rule-templates) section under [Text Threads](./text-threads.md).
