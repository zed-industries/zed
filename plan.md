# Plan: Auto-run a command when opening a new agent terminal thread

## Goal

Add a setting, surfaced in the Settings UI under **AI**, that lets users specify a
command which is automatically executed inside the shell every time a new terminal
thread is opened in the agent panel (e.g. `claude`, `npm run dev`, `source .env`).

Today, `agent_ui::AgentPanel::new_terminal` → `spawn_terminal` →
`Project::create_terminal_shell` just spawns a plain interactive shell. We will keep
that exact spawn path and, once the terminal exists, write the configured command to
the PTY — the same mechanism already used for Python venv activation scripts — so the
shell itself interprets the command. This is what makes the feature trivially
compatible with macOS/Linux/Windows (PowerShell/cmd) and remote projects (SSH/WSL),
because we never parse, quote, or translate the command ourselves.

## New setting

- **Name:** `agent.terminal_init_command`
- **Type:** `Option<String>` (unset/empty = current behavior, no command)
- **Scope:** user settings (`files: USER`), matching the other `agent.*` items on the AI page
- **Behavior:** runs only for *newly created* terminal threads, not for terminals
  restored on workspace load (a restore reconnects a fresh shell; re-running a
  long-lived command like `npm run dev` on every restart is surprising — see Open
  Questions if we want this configurable later)

## Implementation steps

### 1. Settings content (`crates/settings_content/src/agent.rs`)

Add to `AgentSettingsContent` (next to `expand_terminal_card` etc.):

```rust
/// Command to automatically run in new terminal threads in the agent panel.
/// The command is sent to the shell as if typed, so it is interpreted by your
/// configured shell (including on Windows and remote/WSL projects).
///
/// Default: none
pub terminal_init_command: Option<String>,
```

No `#[serde(default)]` needed for `Option` fields. Add a commented entry to
`assets/settings/default.json` in the `"agent"` block (default `null`), mirroring
how other optional agent settings are documented there.

### 2. Resolved settings (`crates/agent_settings/src/agent_settings.rs`)

- Add `pub terminal_init_command: Option<String>` to `AgentSettings` (follow the
  existing `commit_message_instructions: Option<String>` precedent — no `.unwrap()`).
- In `Settings::from_settings`: `terminal_init_command: agent.terminal_init_command`.
- Update the struct-literal constructions in tests:
  - `crates/agent/src/tool_permissions.rs` (`test_agent_settings`, ~line 597)
  - `crates/agent_ui/src/agent_ui.rs` (`test_agent_command_palette_visibility`, ~line 964)

### 3. Settings UI (`crates/settings_ui/src/page_data.rs`, `ai_page()`)

Add a `SettingsPageItem::SettingItem` to `agent_configuration_section()` (near
"Expand Terminal Card", which is the other terminal-thread-adjacent item):

```rust
SettingsPageItem::SettingItem(SettingItem {
    title: "Terminal Thread Init Command",
    description: "Command to automatically run when opening a new terminal thread in the agent panel. Runs in your configured shell.",
    field: Box::new(SettingField {
        organization_override: None,
        json_path: Some("agent.terminal_init_command"),
        pick: |settings_content| {
            settings_content.agent.as_ref()?.terminal_init_command.as_ref()
        },
        write: |settings_content, value, _| {
            settings_content.agent.get_or_insert_default().terminal_init_command = value;
        },
    }),
    metadata: Some(Box::new(SettingsFieldMetadata {
        placeholder: Some("e.g. claude"),
        ..Default::default()
    })),
    files: USER,
}),
```

No new renderer work is needed: `init_renderers` in
`crates/settings_ui/src/settings_ui.rs` already registers
`add_basic_renderer::<String>(render_text_field)`, so an `Option<String>` field
automatically renders as a text input (empty input clears the setting back to `None`).

### 4. Runtime wiring (`crates/agent_ui/src/agent_panel.rs`)

`spawn_terminal` is shared by three callers: `new_terminal` (user action /
`NewTerminalThread`), `spawn_initial_terminal` (panel opens with terminal as last
entry kind), and `restore_terminal` (workspace load). Thread an explicit flag through
so restores are excluded:

1. Add a parameter to `spawn_terminal`, e.g. `run_init_command: bool`
   (or `initial_input: Option<String>` resolved by the caller — flag preferred so the
   setting is read at spawn-completion time).
   - `new_terminal` → `true`
   - `spawn_initial_terminal` (the `#[cfg(not(test))]` one) → `true`
   - `restore_terminal` / `restore_terminal_for_panel_load` → `false`
2. In `spawn_terminal`'s `cx.spawn_in` continuation, after `terminal_task.await`
   succeeds and before/after creating the `TerminalView`:

```rust
if run_init_command
    && let Some(command) = AgentSettings::get_global(cx)
        .terminal_init_command
        .as_deref()
        .map(str::trim)
        .filter(|command| !command.is_empty())
{
    terminal.update(cx, |terminal, _| {
        let mut input = command.as_bytes().to_vec();
        // CR, not "\r\n": "\r\n" puts PowerShell into continuation mode
        // (same convention as the activation-script writes in
        // crates/terminal/src/terminal.rs, TerminalBuilder::new).
        input.push(b'\x0d');
        terminal.input(input);
    });
}
```

Notes:
- `Terminal::input` already exists, scrolls to bottom, and (in tests) records into
  `input_log` — no terminal-crate changes required.
- Writing happens right after the PTY is created; bytes are buffered by the PTY, so
  there is no need to wait for the prompt (this is exactly how the venv
  `activation_script` lines are delivered in `TerminalBuilder::new`).
- The `#[cfg(test)]` `spawn_initial_terminal` uses display-only terminals
  (`new_display_only_with_bounds`), where `write_to_pty` is a no-op — tests stay safe.

### 5. Cross-platform / remote compatibility (why this design works everywhere)

- **Local Windows:** `create_terminal_shell` spawns the user's configured shell
  (PowerShell, cmd, etc.); our bytes are typed into that shell. The `\x0d`
  terminator follows the existing PowerShell-safe convention.
- **WSL / SSH remotes:** for remote projects `create_terminal_shell_internal` wraps
  the remote shell via `create_remote_shell` and the local PTY runs the
  ssh/wsl proxy process. PTY writes are forwarded to the remote shell unchanged, so
  the command executes remotely with remote paths/semantics — no extra work.
- **No quoting/parsing by Zed:** the string is interpreted verbatim by the user's
  shell, so users write it in their own shell's syntax (same contract as typing it).

### 6. Tests (`crates/agent_ui/src/agent_panel.rs` test module + terminal test-support)

The display-only test terminals make PTY assertions impossible in `agent_panel.rs`'s
existing test harness, so split coverage:

- **Settings plumbing test:** set `agent.terminal_init_command` via
  `SettingsStore::update_user_settings`, assert `AgentSettings::get_global` resolves it.
- **PTY input test:** an integration-style test that uses a real
  `project.create_terminal_shell` (see `crates/terminal_view/src/terminal_view.rs`
  tests for the pattern) and asserts via `Terminal::take_input_log`
  (`test-support` feature) that the command + `\x0d` was written when spawning
  through the agent panel path. If wiring a real PTY into the agent-panel test
  harness is too heavy, factor the "resolve command from settings" logic into a
  small pure helper (`fn terminal_init_input(cx) -> Option<Vec<u8>>`) and unit-test
  that, plus a test asserting `restore_terminal` passes `run_init_command: false`.
- **Restore test:** extend `test_active_terminal_serialize_and_load_round_trip` (or
  add a sibling) to assert restored terminals do not receive the init command.

### 7. Docs

Mention the new setting in the agent panel docs
(`docs/src/ai/agent-panel.md` / `docs/src/ai/agent-settings.md` — wherever terminal
threads are documented) with a JSON example:

```json
{
  "agent": {
    "terminal_init_command": "claude"
  }
}
```

## Alternatives considered

1. **Append the command to `activation_script` in
   `Project::create_terminal_shell`** (new optional parameter). Pros: single write
   path, doesn't set `keyboard_input_sent`. Cons: touches ~10 call sites of a shared
   project API for an agent-panel-only feature; rejected in favor of the call-site
   `Terminal::input` approach. (If we later want the same feature for the regular
   terminal panel, promoting it to a `create_terminal_shell` option is the natural
   refactor.)
2. **Spawn via task infra (`create_terminal_task` / `SpawnInTerminal`)** — runs
   `shell -c <command>`; the terminal dies when the command exits and it becomes a
   "task" terminal, losing interactive use. Not what we want.
3. **Per-project setting (`files: USER | PROJECT`)** — useful for project-specific
   dev servers, but all other `agent.*` items are user-scoped today; start with USER
   and extend later if requested.

## Open questions

- Should restored terminal threads also re-run the command? (Plan says no; flip the
  `run_init_command` flag in `restore_terminal` if product decides otherwise.)
- Single command vs. `Vec<String>` of commands — start with a single string (users
  can chain with `&&`/`;` in their shell); a list can be added backward-compatibly
  by accepting both via an untagged enum later.

## Touched files summary

| File | Change |
| --- | --- |
| `crates/settings_content/src/agent.rs` | new `terminal_init_command` field |
| `assets/settings/default.json` | documented default (`null`) |
| `crates/agent_settings/src/agent_settings.rs` | resolved field + `from_settings` |
| `crates/settings_ui/src/page_data.rs` | new AI-page setting item |
| `crates/agent_ui/src/agent_panel.rs` | `run_init_command` flag + PTY write + tests |
| `crates/agent/src/tool_permissions.rs`, `crates/agent_ui/src/agent_ui.rs` | update `AgentSettings` literals in tests |
| docs | document the setting |

## Validation

- `cargo check -p settings_content -p agent_settings -p settings_ui -p agent_ui`
- `cargo test -p agent_ui agent_panel` (new + existing terminal tests)
- `./script/clippy`
- Manual: set the command in Settings UI → AI, open a terminal thread on
  macOS/Linux, Windows (PowerShell), and a WSL/SSH project; verify the command is
  echoed and executed; verify restored terminals don't re-run it.
