# Terminal Permissions Pipeline UI — Continuation Prompt

## Branch & PR

- **Branch**: `terminal-pipeline-permissions`
- **PR**: https://github.com/zed-industries/zed/pull/49547 (draft)
- Rebased onto latest `origin/main`

## What This Feature Does

When an agent runs a pipeline command like `cargo test 2>&1 | tail`, the terminal permissions UI lets the user selectively "always allow" individual commands in the pipeline (e.g. `cargo` and `tail` separately), rather than only offering a blanket "always allow" for the first command.

## Current State

The feature is functionally complete and compiles cleanly with all tests passing. A designer has also pushed UI polish commits on top of the initial implementation.

### Commits on the branch (oldest first)

1. **Terminal permissions: per-command pipeline UI** — the main implementation
2. **Refine UI and behavior** — designer polish
3. **Avoid layout shift by anchoring the menu to the right-side of the button** — designer fix
4. **Improve UI for Apply button** — designer polish
5. **Fix keybinding panicking because of double-update** — designer bugfix

### Files changed (7 files, ~815 lines added)

| File | Role |
|------|------|
| `crates/agent/src/pattern_extraction.rs` | `extract_all_terminal_patterns()` — parses pipelines into per-command regex patterns |
| `crates/acp_thread/src/connection.rs` | `CommandPattern` struct + `PermissionOptions::DropdownWithPatterns` variant |
| `crates/agent/src/thread.rs` | `build_permission_options` — pipelines with ≥2 commands produce `DropdownWithPatterns` |
| `crates/agent_ui/src/agent_ui.rs` | `ToggleCommandPattern` action definition |
| `crates/agent_ui/src/acp/thread_view.rs` | Import updates |
| `crates/agent_ui/src/acp/thread_view/active_thread.rs` | UI state, rendering, and authorization logic (bulk of the work) |
| `crates/agent/src/tests/mod.rs` | 4 integration tests for permission option construction |

### UI Interaction Model

The dropdown menu has three mutually-exclusive radio states:

1. **"Always for terminal"** — blanket always-allow for all terminal commands
2. **"Only this time"** — one-shot allow/deny (default, checked by default)
3. **"Select options…"** — plain entry (no checkmark); when selected, expands to show per-command checkboxes below a separator, all checked by default

The **Allow** / **Deny** buttons are outside the dropdown. The dropdown just picks the mode; Allow/Deny commits it. An **Apply** button inside the dropdown simply closes it.

When "Select options…" is active, the dropdown trigger button reads **"Allow selected commands"**.

### Key Implementation Detail: Direct Entity Updates

The persistent context menu (`ContextMenu::build_persistent`) stays open after clicks and calls `rebuild()` synchronously in the `on_click` handler. But `Window::dispatch_action()` is **deferred** via `cx.defer()`. This caused toggles to be "one click behind" — `rebuild()` would read stale state.

**Fix**: The toggle and radio handlers update the `AcpThreadView` entity directly via the captured `WeakEntity<AcpThreadView>` instead of dispatching actions. This ensures state is current when `rebuild()` reads it. The non-pipeline dropdown (`ContextMenu::build`, non-persistent) doesn't have this issue since it dismisses on click.

### Authorization Flow (`authorize_with_granularity`)

- **"Always for terminal"** or **"Only this time"**: uses the corresponding `PermissionOptionChoice` allow/deny option directly.
- **"Select options…"** + Allow: collects checked patterns, encodes as `always_allow_patterns:terminal\n^cargo\\b\n^tail\\b`, authorizes with `AllowAlways`.
- **"Select options…"** + Deny: falls back to deny-once.

## Outstanding Work

### Testing strategy (high priority)
We discussed adding more durable test coverage beyond the existing 9 unit/integration tests. Manual QA has been sufficient during iteration but we want automated tests before merging. This is the next thing to discuss.

### Potential UX iterations
- **Deny + patterns behavior**: Currently deny with "Select options" active just denies once. Should it persist per-command deny rules?
- **Subagent support**: `DropdownWithPatterns` in the subagent path silently degrades to a plain dropdown. Reasonable for now.
- **Visual polish**: The Apply button could show a `⌘↵` key binding hint per the designer mockup.
- **Filtering already-allowed patterns**: We decided NOT to filter — showing all subcommands is less confusing, even if some are already allowed.

### Notes directory
There is a `.notes/` directory in the repo root with ephemeral working notes. **Do not commit anything under `.notes/`** — it's not in `.gitignore`, we're just being careful not to include it in commits.