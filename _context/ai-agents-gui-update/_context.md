# Session: Agent Panel GUI Updates — Thread Editor, Terminal Fix, Message Editing Plan
Date: 2026-03-22

## Goal

Three main objectives this session:

1. **Thread Content Editor improvements** — Filter the JSONL editor to only show USER and ASSISTANT messages (hiding noise like session_meta, event_msg, turn_context). Change click behavior to "truncation point" selection (click a message = select it + everything below for deletion).

2. **Terminal tab fix for Codex sessions** — When double-clicking a Codex session in the history panel to resume it, the terminal tab showed a play/rerun button, a blue notification dot, and couldn't be renamed. Root cause: sessions were opened as "task terminals" via `spawn_task()` instead of regular terminals.

3. **Message editing plan for ACP agents** — Design and plan how to enable the disabled pencil/edit button on user messages in the agent panel (right dock) for Codex CLI and other ACP-based agents.

---

## Files Changed

### Modified

#### `crates/agent_ui/src/thread_content_editor.rs`
**What changed:** Major rewrite of the thread content editor.

- **Added `visible_indices: Vec<usize>` field** — Maps display rows to actual entry indices. Only USER and ASSISTANT entries are included, hiding all noise (session_meta, event_msg, turn_context, progress, unknown, etc.).
- **Added `compute_visible_indices()` method** — Computes the filtered list of indices.
- **Replaced `toggle_message()` and `check_from()` with `select_truncation_point()`** — New click behavior: clicking a visible message selects it + all hidden entries above (until previous visible message) + ALL entries below to the end. Clicking the truncation point again clears all selections. This gives "truncate from here" semantics.
- **Removed right-click context menu** (`right_click_menu`, `ContextMenu`, `ContextMenuEntry` imports removed) — No longer needed since click behavior handles selection.
- **Simplified `render_message_row()`** — Uses `display_ix` → `visible_indices[display_ix]` mapping. Checkbox + click both trigger `select_truncation_point()`.
- **Updated `render_toolbar()`** — Shows "X entries selected" only when count > 0 (was showing "X/Y selected for deletion" always).
- **Updated `Render::render()`** — Uses `visible_indices.len()` instead of `entries.len()` for uniform_list count.
- **Updated `delete_checked()`** — Recomputes `visible_indices` after deletion.

**Note:** The user's linter subsequently modified this file further, adding:
- `EntryType::System` variant for system/instruction messages
- `is_system_content()` function to detect system messages by content patterns
- `toggle_message()` / `delete_from_here()` / `select_all()` / `deselect_all()` methods replacing `select_truncation_point()`
- Right-click context menu restored with "Delete from here", "Select all", "Deselect all" options
- Tool-use items filtered out of `extract_codex_content()` and `extract_content_text()`
- Empty user/assistant messages reclassified as `EntryType::Other`

#### `crates/terminal_view/src/terminal_panel.rs`
**What changed:** Made `add_terminal_shell` method public.

- Line 821: Changed `fn add_terminal_shell(` to `pub fn add_terminal_shell(`
- **Why:** Needed to call this from `agents_panel.rs` to create regular terminals (not task terminals) for Codex session resume.

#### `crates/agent_ui_v2/src/agents_panel.rs:452-496`
**What changed:** Rewrote `resume_cli_session()` to use regular terminals instead of task terminals.

**Before:** Used `task::SpawnInTerminal` with `spawn_task()` — created task terminals with play button, blue dot, no rename support.

**After:** Uses `panel.add_terminal_shell(cwd, RevealStrategy::Always, window, cx)` to create a regular terminal, then:
1. Sets title override via `terminal.set_title_override(Some(label), cx)`
2. Writes the resume command to stdin via `terminal.input(command.into_bytes())`
   - Codex: `codex resume <session_id>\n`
   - Claude: `claude-code --resume <session_id>\n`

**Result:** No play button, no blue dot, rename works normally.

#### `_plans/jiggly-sparking-sutton.md` (plan file)
**What changed:** Complete rewrite of the message editing plan from "not feasible" to a concrete implementation plan.

**Final plan:** Enable the pencil button on user messages in the agent panel by implementing `truncate()` on `AcpConnection` as a no-op. The existing `rewind()` + `send_impl()` flow already handles everything — we just need to enable it. Background file truncation of the `.jsonl` session file keeps persistence in sync.

---

## Problems & Solutions

### Problem 1: Thread Content Editor showed ALL entries including noise
**Issue:** The editor displayed session_meta, event_msg, turn_context, unknown entries — making it hard to find actual user/assistant messages.
**Cause:** No filtering was applied; all JSONL lines were shown.
**Fix:** Added `visible_indices` field that only includes USER and ASSISTANT entries. The uniform_list renders only these filtered entries.

### Problem 2: Click behavior was individual toggle (wrong UX)
**Issue:** Each checkbox toggled independently. User wanted "truncation point" behavior — click a message to select everything from there down.
**Cause:** Original design used per-item toggle.
**Fix:** Replaced with `select_truncation_point()` — clicking sets a truncation point (selects that message + hidden entries above until previous visible + ALL below). Clicking again clears.

### Problem 3: Codex terminal had play button, blue dot, couldn't rename
**Issue:** Double-clicking a Codex session in history opened a terminal with task-specific UI elements.
**Cause:** `resume_cli_session()` used `spawn_task()` which creates task terminals. Task terminals show `show_rerun` button, blue dot when `TaskStatus::Running`, and have task-specific tab rendering.
**Fix:** Switched to `add_terminal_shell()` (regular terminal) + writing command to stdin + setting title override. Made `add_terminal_shell` public in `terminal_panel.rs`.

### Problem 4: Compile error — `?` on `()` return type
**Issue:** `terminal.update(cx, ...)` in async context — tried to use `?` on a call that returned `()`.
**Cause:** After `terminal.upgrade()`, `Entity::update` returns the closure's return type directly. But `WeakEntity::update` with `AsyncApp` wraps in `Result`.
**Fix:** Kept `terminal` as `WeakEntity<Terminal>` (from `task.await?`) instead of upgrading to `Entity`, so `.update(cx, ...)` returns `Result<()>`.

### Problem 5: Message editing pencil button disabled for all ACP agents
**Issue:** Pencil button shows "Editing previous messages is not available for [agent] yet."
**Cause:** `AcpConnection` doesn't implement `truncate()` — uses default trait method returning `None`. This makes `message_id = None` at `acp_thread.rs:1871`, which disables the pencil at `thread_view.rs:2495`.
**Fix (planned, not yet implemented):** Implement `truncate()` on `AcpConnection` as a no-op that returns `Some(AcpSessionTruncator)`. The existing `rewind()` + `regenerate()` flow handles everything else. Background file truncation keeps the `.jsonl` file in sync.

---

## Current Status

### Working
- Thread Content Editor only shows USER and ASSISTANT messages (noise filtered)
- Click/selection behavior updated (linter modified to individual toggle + context menu with "Delete from here")
- Codex session resume opens clean regular terminal (no play button, no blue dot, rename works)
- Both `agent_ui` and `agent_ui_v2` crates compile cleanly

### Planned (Not Yet Implemented)
- **Message editing for ACP agents** — Plan approved, implementation not started. Key changes needed:
  1. `crates/agent_servers/src/acp.rs` — Add `AcpSessionTruncator` struct (no-op) + implement `truncate()` on `AcpConnection` (~15 lines)
  2. `crates/acp_thread/src/acp_thread.rs` — Add `session_file_path()` method + background file truncation in `rewind()`
  3. `crates/agent/src/claude_code_sessions.rs` — Make `sessions_dir_for_project()` public

### Notes for Next Session
- The linter modified `thread_content_editor.rs` after our changes — the linter's version is the current truth (has System entry type, restored context menu, individual toggle instead of truncation point)
- The message editing plan is at `/Users/alesloas/.claude/plans/jiggly-sparking-sutton.md`
- The pencil button will only be enabled for NEW messages (sent after the code change). Old messages already have `message_id = None` and can't be retroactively made editable.
- The background file truncation needs `session_file_path()` which requires knowing whether the agent is Claude Code or Codex to derive the correct `.jsonl` file location
- For Claude Code: `~/.claude/projects/{path-with-slashes-replaced}/{session_id}.jsonl`
- For Codex: file path stored in session meta as `codex_file_path`, or search `~/.codex/sessions/`

---

## Architecture Reference

### Key Files
- `crates/agent_ui/src/thread_content_editor.rs` — JSONL session file editor (workspace tab)
- `crates/agent_ui_v2/src/agents_panel.rs` — Left dock agents panel (v2, the one actually visible)
- `crates/agent_ui_v2/src/thread_history.rs` — Thread history with Claude CLI / Codex CLI tabs
- `crates/agent_ui/src/acp/thread_view.rs` — Agent panel conversation view (right dock)
- `crates/agent_servers/src/acp.rs` — ACP connection to external agents (Codex, Claude Code)
- `crates/acp_thread/src/acp_thread.rs` — Thread state management, rewind/submit logic
- `crates/acp_thread/src/connection.rs` — AgentConnection trait, AgentSessionTruncate trait
- `crates/terminal_view/src/terminal_panel.rs` — Terminal panel, task/shell terminal creation
- `crates/agent/src/claude_code_sessions.rs` — Session file indexing for Claude Code and Codex

### Key Concepts
- **`agent_ui` vs `agent_ui_v2`**: User sees `agent_ui_v2` (left dock). `agent_ui` contains shared components like ThreadContentEditor.
- **ACP protocol**: External agents communicate via stdin/stdout JSON-RPC. Server maintains conversation history per session. `prompt()` sends only the new user message — server has the rest.
- **`truncate()` gating**: The pencil button is enabled/disabled based on whether `connection.truncate()` returns `Some` or `None`. This check happens at message creation time (`acp_thread.rs:1871`).
- **Task vs shell terminals**: Task terminals (`spawn_task`) show rerun button + blue dot. Shell terminals (`add_terminal_shell`) are clean regular terminals.
