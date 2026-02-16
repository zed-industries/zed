# Zed Documentation Conventions

This document covers structural conventions for Zed documentation: what to document, how to organize it, and when to create new pages.

For voice, tone, and writing style, see the [brand-voice/](./brand-voice/) directory, which contains:

- `SKILL.md` — Core voice principles and workflow
- `rubric.md` — 8-point scoring criteria for quality
- `taboo-phrases.md` — Patterns and phrases to avoid
- `voice-examples.md` — Before/after transformation examples

---

## What Needs Documentation

### Document

- **New user-facing features** — Anything users interact with directly
- **New settings or configuration options** — Include the setting key, type, default value, and example
- **New keybindings or commands** — Use `{#action ...}` and `{#kb ...}` syntax
- **All actions** — Completeness matters; document every action, not just non-obvious ones
- **New AI capabilities** — Agent tools, providers, workflows
- **New providers or integrations** — LLM providers, MCP servers, external agents
- **New tools** — Agent tools, MCP tools, built-in tools
- **New UI panels or views** — Any new panel, sidebar, or view users interact with
- **Public extension APIs** — For extension developers
- **Breaking changes** — Even if the fix is simple, document what changed
- **Version-specific behavior changes** — Include version callouts (e.g., "In Zed v0.224.0 and above...")

### Skip

- **Internal refactors** — No user-visible change, no docs
- **Bug fixes** — Unless the fix reveals that existing docs were wrong
- **Performance improvements** — Unless user-visible (e.g., startup time)
- **Test changes** — Never document tests
- **CI/tooling changes** — Internal infrastructure

---

## Page vs. Section Decisions

### Create a new page when:

- Introducing a **major feature** with multiple sub-features (e.g., Git integration, Vim mode)
- The topic requires **extensive configuration examples**
- Users would search for it **by name** (e.g., "Zed terminal", "Zed snippets")
- It's a **new category** (e.g., a new AI provider type)

### Add to an existing page when:

- Adding a **setting** to a feature that already has a page
- Adding a **keybinding** to an existing feature
- The change is a **minor enhancement** to existing functionality
- It's a **configuration option** for an existing feature

### Examples

| Change                               | Action                                 |
| ------------------------------------ | -------------------------------------- |
| New "Stash" feature for Git          | Add section to `git.md`                |
| New "Remote Development" capability  | Create `remote-development.md`         |
| New setting `git.inline_blame.delay` | Add to existing Git config section     |
| New AI provider (e.g., "Ollama")     | Add section to `llm-providers.md`      |
| New agent tool category              | Potentially new page, depends on scope |

---

## Document Structure

### Frontmatter

Every doc page needs YAML frontmatter:

```yaml
---
title: Feature Name - Zed
description: One sentence describing what this page covers. Used in search results.
---
```

- `title`: Feature name, optionally with "- Zed" suffix for SEO
- `description`: Concise summary for search engines and link previews

### Section Ordering

1. **Title** (`# Feature Name`) — Clear, scannable
2. **Opening paragraph** — What this is and why you'd use it (1-2 sentences)
3. **Getting Started / Usage** — How to access or enable it
4. **Core functionality** — Main features and workflows
5. **Configuration** — Settings, with JSON examples
6. **Keybindings / Actions** — Reference tables
7. **See Also** — Links to related docs

### Section Depth

- Use `##` for main sections
- Use `###` for subsections
- Avoid `####` unless absolutely necessary — if you need it, consider restructuring

### Anchor IDs

Add explicit anchor IDs to sections users might link to directly:

```markdown
## Getting Started {#getting-started}

### Configuring Models {#configuring-models}
```

Use anchor IDs when:

- The section is a common reference target
- You need a stable link that won't break if the heading text changes
- The heading contains special characters that would create ugly auto-generated anchors

---

## Formatting Conventions

### Code Formatting

Use inline `code` for:

- Setting names: `vim_mode`, `buffer_font_size`
- Keybindings: `cmd-shift-p`, `ctrl-w h`
- Commands: `:w`, `:q`
- File paths: `~/.config/zed/settings.json`
- Action names: `git::Commit`
- Values: `true`, `false`, `"eager"`

### Action and Keybinding References

Use Zed's special syntax for dynamic rendering:

- `{#action git::Commit}` — Renders the action name
- `{#kb git::Commit}` — Renders the keybinding for that action

This ensures keybindings stay accurate if defaults change.

### JSON Examples

Always use the `[settings]` or `[keymap]` annotation:

```json [settings]
{
  "vim_mode": true
}
```

```json [keymap]
{
  "context": "Editor",
  "bindings": {
    "ctrl-s": "workspace::Save"
  }
}
```

### Tables

Use tables for:

- Action/keybinding reference lists
- Setting options with descriptions
- Feature comparisons

Keep tables scannable — avoid long prose in table cells.

### Paragraphs

- Keep paragraphs short (2-3 sentences max)
- One idea per paragraph
- Use bullet lists for multiple related items

### Pronouns

Minimize vague pronouns like "it", "this", and "that". Repeat the noun so readers know exactly what you're referring to.

**Bad:**

> The API creates a token after authentication. It should be stored securely.

**Good:**

> The API creates a token after authentication. The token should be stored securely.

This improves clarity for both human readers and AI systems parsing the documentation.

### Callouts

Use blockquote callouts for tips, notes, and warnings:

```markdown
> **Note:** This feature requires signing in.

> **Tip:** Hold `cmd` when submitting to automatically follow the agent.

> **Warning:** This action cannot be undone.
```

### Version-Specific Notes

When behavior differs by version, be explicit:

```markdown
> **Note:** In Zed v0.224.0 and above, tool approval is controlled by `agent.tool_permissions.default`.
```

Include the version number and what changed. This helps users on older versions understand why their behavior differs.

---

## Cross-Linking

### Internal Links

Link to other docs using relative paths:

- `[Vim mode](./vim.md)`
- `[AI configuration](./ai/configuration.md)`

### External Links

- Link to `zed.dev` pages when appropriate
- Link to upstream documentation (e.g., Tree-sitter, language servers) when explaining integrations

### "See Also" Sections

End pages with related links when helpful:

```markdown
## See also

- [Agent Panel](./agent-panel.md): Agentic editing with file read/write
- [Inline Assistant](./inline-assistant.md): Prompt-driven code transformations
```

---

## Language-Specific Documentation

Language docs in `src/languages/` follow a consistent structure:

1. Language name and brief description
2. Installation/setup (if needed)
3. Language server configuration
4. Formatting configuration
5. Language-specific settings
6. Known limitations (if any)

Keep language docs focused on Zed-specific configuration, not general language tutorials.

---

## Settings Documentation

When documenting settings:

1. **Show the Settings Editor (UI) approach first** — Most settings have UI support
2. **Then show JSON** as "or add to your settings file:"
3. **State the setting key** in code formatting
4. **Describe what it does** in one sentence
5. **Show the type and default** if not obvious
6. **Provide a complete JSON example**

Example:

> Configure inline blame in Settings ({#kb zed::OpenSettings}) by searching for "inline blame", or add to your settings file:
>
> ```json [settings]
> {
>   "git": {
>     "inline_blame": {
>       "enabled": false
>     }
>   }
> }
> ```

For JSON-only settings (complex types without UI support), note this and link to instructions:

> Add the following to your settings file ([how to edit](./configuring-zed.md#settings-files)):

### Settings File Locations

- **macOS/Linux:** `~/.config/zed/settings.json`
- **Windows:** `%AppData%\Zed\settings.json`

### Keymap File Locations

- **macOS/Linux:** `~/.config/zed/keymap.json`
- **Windows:** `%AppData%\Zed\keymap.json`

---

## Terminology

Use consistent terminology throughout:

| Use             | Instead of                             |
| --------------- | -------------------------------------- |
| folder          | directory                              |
| project         | workspace                              |
| Settings Editor | settings UI                            |
| command palette | command bar                            |
| panel           | sidebar (be specific: "Project Panel") |

---

## Formatting Requirements

All documentation must pass **Prettier** formatting (80 character line width):

```sh
cd docs && npx prettier --check src/
```

Before any documentation change is considered complete:

1. Run Prettier to format: `cd docs && npx prettier --write src/`
2. Verify it passes: `cd docs && npx prettier --check src/`

---

## Quality Checklist

Before finalizing documentation:

- [ ] Frontmatter includes `title` and `description`
- [ ] Opening paragraph explains what and why
- [ ] Settings show UI first, then JSON examples
- [ ] Actions use `{#action ...}` and `{#kb ...}` syntax
- [ ] All actions are documented (completeness matters)
- [ ] Anchor IDs on sections likely to be linked
- [ ] Version callouts where behavior differs by release
- [ ] No orphan pages (linked from somewhere)
- [ ] Passes Prettier formatting check
- [ ] Passes brand voice rubric (see `brand-voice/rubric.md`)

---

## Gold Standard Examples

See `../.doc-examples/` for curated examples of well-documented features. Use these as templates when writing new documentation.

---

## Reference

For automation-specific rules (safety constraints, change classification, output formats), see `docs/AGENTS.md`.
