# Editing Code

Zed provides tools to help you write and modify code efficiently. This section covers the core editing features that work alongside your language server.

## What's in This Section

- **[Code Completions](./completions.md)** — Autocomplete from language servers and AI-powered edit predictions
- **[Snippets](./snippets.md)** — Insert reusable code templates with tab stops
- **[Formatting & Linting](./configuring-languages.md#formatting-and-linting)** — Configure automatic code formatting and linter integration
- **[Diagnostics & Quick Fixes](./diagnostics.md)** — View errors, warnings, and apply fixes from your language server
- **[Multibuffers](./multibuffers.md)** — Edit multiple files simultaneously with multiple cursors

## How These Features Work Together

When you're editing code, Zed combines input from multiple sources:

1. **Language servers** provide completions, diagnostics, and quick fixes based on your project's types and structure
2. **Edit predictions** suggest multi-character or multi-line changes as you type
3. **Multibuffers** let you apply changes across files in one operation

For example, you might:

- Rename a function using your language server's rename refactor
- See the results in a multibuffer showing all affected files
- Use multiple cursors to make additional edits across all locations
- Get immediate diagnostic feedback if something breaks

## Related Features

- [Configuring Languages](./configuring-languages.md) — Set up language servers for your project
- [Key Bindings](./key-bindings.md) — Customize keyboard shortcuts for editing commands
