# Plan: `test_env_file` per-language setting

## Goal

Per-language setting that injects environment variables from a `.env` file into the test process before running tests via Zed's runnable code-lenses (gutter Run, "Run test", "Run package", LSP-driven runnables).

```json
{ "languages": { "Go": { "test_env_file": "$ZED_WORKTREE_ROOT/.env" } } }
```

DAP-driven debug runs are **already** covered by Zed's existing per-debug-config envFile mechanism (`crates/dap_adapters/src/go.rs:598`) and are out of scope here.

---

## Architecture (locked)

The setting is wired through `TaskContext.project_env`. When the editor builds a `TaskContext` for a buffer, we read the buffer-language's `test_env_file`, parse and resolve it, and **extend** `project_env` with the file contents. The existing `resolve_task()` merge order in `crates/task/src/task_template.rs:251-269` then handles precedence automatically:

```
inherited_shell_env  ◀ extended by ▶  .env file
                                            │
                         resolve_task ──── extend(template.env)
                                            │
                                expand_$ZED_*_variables
                                            │
                         resolve_task ──── extend(task_variables)
```

Final precedence: `inherited_shell < .env file < tasks.json template.env < $ZED_* task variables`.

No changes to the `task` crate. No changes to `workspace::tasks::schedule_resolved_task`. All wiring lives in the editor crate where `TaskContext` is constructed.

**Tag filter is not applied.** Because `TaskContext` is shared across all runnables of a buffer, the file is loaded for every runnable of a language with `test_env_file` set, including `go-main` etc. This is an accepted leak — adding a tag filter is a follow-up if it bites in practice.

---

## Implementation steps

### 1. Settings field — `crates/settings_content/src/language.rs`

Add to `LanguageSettingsContent` (line 397+), following the existing `FormatterList` pattern (line 949) and `PathHyperlinkRegex` pattern (`terminal.rs:486`):

```rust
#[serde(default)]
pub test_env_file: Option<TestEnvFilePaths>,

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(untagged)]
pub enum TestEnvFilePaths {
    Single(String),
    Multiple(Vec<String>),
}
```

Mirror in the resolved settings type at `crates/language/src/language_settings.rs`. JSON schema is auto-generated.

### 2. `.env` parser module — `crates/task/src/test_env_file.rs` (new)

```rust
pub fn load_env_files(
    paths: &[String],
    worktree_root: &Path,
    inherited_env: &HashMap<String, String>,
) -> (HashMap<String, String>, Vec<EnvFileWarning>);
```

- Resolve each path:
  - Expand `$ZED_WORKTREE_ROOT` to `worktree_root`.
  - Expand `~` to home.
  - Relative paths resolve against `worktree_root`.
  - Allow `..` and follow symlinks (no sandboxing).
- Skip non-existent paths silently.
- Parse via `dotenvy::from_read_iter()` — same call already used in `dap_adapters/src/go.rs:598`. `${VAR}` expansion against `inherited_env ∪ already-parsed-keys` happens in dotenvy.
- Layered files: iterate paths in order, later entries overwrite earlier (`HashMap::extend`).
- Collect parse errors as `EnvFileWarning { path, line, message }` — never panic, never abort.

### 3. Hook helper — `crates/editor/src/runnables.rs` (private fn)

```rust
fn extend_project_env_with_test_env_file(
    project_env: &mut HashMap<String, String>,
    buffer: &Buffer,
    cx: &App,
) -> Vec<EnvFileWarning>;
```

- Read `language_settings(buffer.language(), buffer.file(), cx).test_env_file`.
- If `None` → return immediately, no-op.
- Resolve worktree root for the buffer.
- Call `task::test_env_file::load_env_files(...)`.
- `project_env.extend(file_env)` — file > shell.
- Return warnings for the caller to surface (or drop, see step 4).

### 4. Wire into 3 `TaskContext` construction sites

| File | Line | Site |
|---|---|---|
| `crates/editor/runnables.rs` | 311 | `build_tasks_context` (gutter Run, code-lens) |
| `crates/editor/code_lens.rs` | 101 | LSP code-lens-derived contexts |
| `crates/editor/lsp_ext.rs` | 86-94 | LSP runnable contexts (rust-analyzer "Run test" etc.) |

In each site: after `project_env` is populated from project shell-env, call the helper from step 3. Net change: ~3 lines per site.

Warnings: `code_lens.rs` and `lsp_ext.rs` paths drop them silently for the first cut. `runnables.rs` (the user-visible gutter Run) surfaces them via a non-blocking workspace notification — only if the warning vec is non-empty, only once per session per file path (dedupe by path).

### 5. Tests

#### Unit (`crates/task/src/test_env_file.rs`)

```rust
#[test] fn parses_basic_kv() { ... }
#[test] fn ignores_comments() { ... }
#[test] fn handles_export_prefix() { ... }
#[test] fn handles_quoted_values() { ... }
#[test] fn expands_var_against_inherited_env() { ... }
#[test] fn layered_files_later_overrides_earlier() { ... }
#[test] fn missing_file_returns_empty_no_error() { ... }
#[test] fn malformed_line_collected_as_warning() { ... }
#[test] fn file_overrides_shell() { ... }
```

#### Integration (`crates/editor/src/runnables.rs` test module, `#[gpui::test]` + `FakeFs`)

```rust
test_env_file_loads_into_resolved_task_env
test_no_setting_means_no_change
test_missing_file_silently_skipped
test_layered_files_apply_in_order
test_template_env_overrides_file
test_zed_task_variables_override_file
```

### 6. Docs — `docs/src/configuring-zed.md`

One paragraph in the language-specific settings section, with a JSON example. No security warning. No `${workspaceFolder}` mention. Note that DAP debug runs use a separate mechanism.

### 7. Release notes & PR

PR title: `Add test_env_file language setting to load .env into test runs`

```
Release Notes:

- Added the `test_env_file` per-language setting that loads a `.env` file into the environment of test runs.
```

---

## Decisions locked

| # | Decision | Choice |
|---|---|---|
| 1 | Scope | Per-language (`LanguageSettingsContent`) |
| 2 | Test-task filter | None — applied to all runnables of the language |
| 3 | Hook layer | Extend `TaskContext.project_env` at construction sites |
| 4a | Path variable | `$ZED_WORKTREE_ROOT` only, no native `${workspaceFolder}` |
| 4b | Relative paths | Resolved against buffer's worktree root |
| 4c | Multi-worktree | Per-buffer-worktree, automatic |
| 4d | `..` / symlinks | Allowed, no sandboxing |
| 5 | Precedence | `file > shell`, `template.env > file`, `$ZED_* > everything` |
| 6 | Setting shape | `Option<TestEnvFilePaths>` with untagged `Single | Multiple`, mirrors `FormatterList` |
| 7a | Settings layers | User + project-local (free via `LanguageSettingsContent`) |
| 7b | Merge | Replace (default `MergeFrom`) |
| 7c | `.gitignore` validation | None |

---

## Out of scope

- DAP debug runs — already have their own envFile via `dap_adapters/src/go.rs:598`.
- File watcher / hot reload — read-on-launch is sufficient, files are tiny.
- General `env_file` for non-test tasks (user-defined `tasks.json` already has `env`).
- `${workspaceFolder}` alias — Zed-native pattern is `$ZED_WORKTREE_ROOT`.
- Per-test-target overrides.
- Encrypted formats (sops, 1Password).
- Tag-based filter to restrict to test-tagged runnables — leak to `*-main`/`*-bench` is accepted.

---

## Order of work

Single PR containing all of: setting field, parser, hook helper, three call-site wirings, unit tests, integration tests, docs, release notes.

Suggested commit order inside the branch (for reviewer ergonomics, not required to merge separately):

1. Setting field in `LanguageSettingsContent` + mirrored resolved type.
2. Parser module `crates/task/src/test_env_file.rs` + unit tests.
3. Helper in `crates/editor/src/runnables.rs` + wire-up at 3 `TaskContext` construction sites.
4. Integration tests.
5. Docs + release notes.
