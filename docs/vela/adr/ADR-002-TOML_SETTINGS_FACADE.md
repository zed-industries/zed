# ADR-002: Use TOML as the CodeIDE user configuration facade

- Status: Accepted
- Date: 2026-07-23
- Zed baseline: `6297c88f428a99741a7bfb33f31dfe98123bb8e4`

## Context

CodeIDE configuration must be editable through UI and persisted in a comment-friendly non-JSON format. Zed's existing settings implementation is mature but strongly coupled to JSON/JSONC:

- `crates/settings` loads and updates `settings.json`;
- `crates/settings_content` provides typed setting content;
- `crates/settings_ui` reads and writes through `SettingsStore`;
- project overrides use `.zed/settings.json`;
- schemas, completion, errors, tests, and UI labels assume JSON.

Replacing every layer at once would create a large fork and make upstream synchronization difficult.

## Decision

TOML is the only user-visible CodeIDE configuration format. CodeIDE will initially implement a facade that parses typed TOML and applies it to the existing `SettingsStore` without generating a user-visible `settings.json`.

The first implementation covers CodeIDE-specific settings and selected high-frequency editor settings. Coverage expands incrementally after round-trip and source-attribution tests exist.

Recommended paths:

```text
~/.config/codeide/config.toml
~/.config/codeide/keybindings.toml
~/.config/codeide/permissions.toml
<workspace>/.codeide/config.toml
<workspace>/.codeide/tasks.toml
<workspace>/.codeide/debug.toml
```

## Layer order

From lowest to highest precedence:

1. Built-in defaults;
2. global TOML;
3. workspace TOML;
4. session-only UI overrides.

Every UI field must expose its effective value and source. Reset removes the value from the selected layer rather than writing a copied default.

## Persistence rules

- Use `toml_edit` so UI mutations preserve comments, order, formatting, and unknown keys.
- Parse into strong Rust types before applying changes.
- Write a temporary file, flush and sync it, then atomically replace the target.
- Watch files for external changes and show conflicts instead of silently overwriting.
- Include `schema_version` and create a backup before migration.
- Store credentials in the platform credential provider; TOML stores only credential references.
- Keep sessions and high-frequency state in the existing database, not TOML.

## Compatibility

External project files retain their required formats, including `package.json` and `tsconfig.json`. Existing Zed JSON settings may be offered as a one-time import source, but CodeIDE does not continue writing them.

Zed's bundled JSON assets can remain internal during the facade phase. This requirement concerns CodeIDE-managed user configuration.

## Consequences

### Positive

- Users get comment-friendly, reviewable configuration.
- Existing typed Zed settings and runtime consumers remain reusable.
- Migration can happen setting group by setting group.

### Negative

- Source mapping between TOML and the existing store adds complexity.
- Unsupported legacy settings need clear UI and import behavior.
- Upstream Settings UI changes require mapping regression tests.

## Initial implementation slices

1. Define TOML schema for Agent, model providers, Git Diff, permissions, and keybindings.
2. Implement parse, validation, layer merge, and source attribution as a focused module in `crates/settings` unless ownership analysis requires a new crate.
3. Add atomic comment-preserving edits.
4. Add global and workspace file watchers.
5. Connect one Settings UI page end to end before expanding coverage.
