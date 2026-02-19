# Terminal Permissions Pipeline UI — Handoff Notes

## What This Is

We're updating the terminal permissions UI in the agent panel so that when an agent runs a pipeline command like `cargo test 2>&1 | tail`, the user can selectively "always allow" individual commands in the pipeline (e.g. `cargo` and `tail` separately), rather than only being offered a blanket "always allow" for the first command.

## Designer Mockup

The mockup shows a dropdown menu with:
1. **"Always for terminal"** — radio-style, sets blanket always-allow for all terminal commands
2. **"Only this time"** — radio-style (default), allow/deny just this invocation
3. **Separator + "Select Options" header**
4. **Per-command checkboxes** — e.g. "Always for `cargo` commands ✓" and "Always for `tail` commands ✓" (all checked by default)
5. **"Apply" button** — closes the dropdown, confirming selection

The main **Allow** / **Deny** buttons are outside the dropdown. Per the designer: the dropdown just picks options; Allow/Deny is what actually commits them. The "Apply" button simply closes the dropdown.

## Current Implementation State

The feature is functionally implemented across 6 files. It compiles cleanly with no warnings and all 547 tests pass (across `agent`, `agent_ui`, and `acp_thread` crates). No tests were broken.

### What was changed, by layer

#### 1. Pattern Extraction — `crates/agent/src/pattern_extraction.rs`

New function `extract_all_terminal_patterns(command: &str) -> Vec<(String, String)>` (around line 57):
- Parses a shell command into individual pipeline segments using `extract_commands`
- Extracts the command name (first token) from each segment
- Filters out path-based commands and redirects using the same validation as `extract_command_name`
- Deduplicates by command name
- Returns `Vec<(regex_pattern, display_name)>` e.g. `[("^cargo\\b", "cargo"), ("^tail\\b", "tail")]`
- 5 new unit tests

#### 2. Data Model — `crates/acp_thread/src/connection.rs`

- New `CommandPattern` struct (line ~447) with `pattern: String` and `display_name: String`
- New `PermissionOptions::DropdownWithPatterns` variant (line ~457) with fields:
  - `choices: Vec<PermissionOptionChoice>` — the granularity radio options
  - `command_patterns: Vec<CommandPattern>` — per-command patterns for checkboxes
  - `tool_name: String`
- All match arms on `PermissionOptions` updated (`is_empty`, `first_option_of_kind`)

#### 3. Permission Building — `crates/agent/src/thread.rs` (`build_permission_options`)

Early return (around line 713) when:
- Tool is `TerminalTool::NAME`
- Shell supports POSIX chaining
- `extract_all_terminal_patterns` returns ≥2 patterns

Returns `DropdownWithPatterns` with "Always for terminal" and "Only this time" as choices, plus the per-command patterns. Single-command terminals fall through to the existing `Dropdown` path unchanged.

#### 4. Authorization Handlers — `crates/agent/src/thread.rs` (`authorize` method)

New handlers (around line 3408) for multi-pattern response format:
- `always_allow_patterns:<tool>\n<pat1>\n<pat2>` → calls `add_tool_allow_pattern` for each
- `always_deny_patterns:<tool>\n<pat1>\n<pat2>` → calls `add_tool_deny_pattern` for each
- Placed BEFORE the singular `always_allow_pattern:` handlers (ordering matters for `strip_prefix`)
- Empty patterns filtered out via `.filter(|s| !s.is_empty())`

#### 5. UI Actions — `crates/agent_ui/src/agent_ui.rs`

- New `ToggleCommandPattern` action (line ~180) with `tool_call_id: String` and `pattern_index: usize`
- `ApplyCommandPatterns` was removed (Apply just closes the dropdown)

#### 6. UI State & Rendering — `crates/agent_ui/src/acp/thread_view/active_thread.rs`

**State** (line ~222):
- New field `selected_command_patterns: HashMap<ToolCallId, HashSet<usize>>` — tracks checked patterns per tool call. Missing entry = all checked (default).
- Initialized to `HashMap::default()` in constructor (line ~413)

**Action handler** (line ~1324):
- `handle_toggle_command_pattern` — on first interaction, lazily initializes the `HashSet` with all indices (so toggling OFF works correctly), then toggles the specified index
- Registered at line ~7714

**Authorization logic** (line ~1374):
- `authorize_pending_with_granularity` — finds first pending tool call, delegates to `authorize_with_granularity`
- `authorize_with_granularity` — the core decision method:
  - For `DropdownWithPatterns` + Allow: collects checked patterns, encodes as `always_allow_patterns:terminal\n^cargo\\b\n^tail\\b`, authorizes with `AllowAlways`
  - For `DropdownWithPatterns` + Deny: uses selected granularity (doesn't persist deny patterns for individual commands)
  - If no patterns checked: falls back to granularity choice
  - For regular `Dropdown`: same behavior as before

**Rendering** (line ~5455):
- `render_permission_buttons_with_patterns` — Allow/Deny buttons that call `authorize_with_granularity` (pattern-aware)
- `render_permission_granularity_dropdown_with_patterns` — persistent context menu (`ContextMenu::build_persistent`) with:
  - Granularity radio entries (toggleable, dispatching `SelectPermissionGranularity`)
  - Separator + "Select Options" header
  - Per-command pattern checkboxes (toggleable, dispatching `ToggleCommandPattern`)
  - "Apply" button that calls `permission_dropdown_handle.hide(cx)` to close the menu
  - Captures a `WeakEntity<AcpThreadView>` to read fresh state on each menu rebuild

**Match site updates**:
- `render_permission_buttons` (line ~5228) — routes `DropdownWithPatterns` to the new render method
- `render_subagent_permission_buttons` (line ~6365) — degrades `DropdownWithPatterns` to plain dropdown for subagents
- `authorize_pending_with_granularity` — handles both `Dropdown` and `DropdownWithPatterns`

**Import** in `crates/agent_ui/src/acp/thread_view.rs` (line 3):
- Added `CommandPattern` to the `acp_thread` import

### Tests added

In `crates/agent/src/tests/mod.rs`:
- `test_permission_options_terminal_pipeline_produces_dropdown_with_patterns` — verifies `cargo test 2>&1 | tail` produces `DropdownWithPatterns` with correct patterns
- `test_permission_options_terminal_single_command_stays_dropdown` — verifies single commands still use `Dropdown`
- `test_permission_options_terminal_pipeline_deduplicates_commands` — verifies `grep | grep` deduplicates
- `test_permission_options_terminal_pipeline_with_chaining` — verifies `npm install && npm test | tail` works

In `crates/agent/src/pattern_extraction.rs`:
- `test_extract_all_terminal_patterns_pipeline`
- `test_extract_all_terminal_patterns_single_command`
- `test_extract_all_terminal_patterns_chained`
- `test_extract_all_terminal_patterns_with_path_commands`
- `test_extract_all_terminal_patterns_all_paths`

## Known Areas for Iteration

### Deny + patterns behavior
Currently, clicking Deny with checked patterns uses the granularity selection (e.g. deny once) rather than persisting deny patterns per-command. This might need refinement — should deny also save per-command deny rules?

### Subagent support
`DropdownWithPatterns` in the subagent path silently degrades to a plain dropdown (no per-command checkboxes). This is a reasonable simplification for now but could be revisited.

### Visual polish
The Apply button currently renders as a plain `Button` inside a centered `h_flex`. The mockup shows it with a `⌘↵` key binding hint — this wasn't implemented because `KeyBinding::new` requires an action reference and we couldn't easily construct one inside the context menu closure. Could use `KeyBinding::for_action_in` if an appropriate action is available, or render a custom keystroke hint element.

### Interaction between radio selection and checkboxes
Currently the radio options ("Always for terminal", "Only this time") and per-command checkboxes are independent. The Allow button prioritizes checked patterns over the radio selection. There's a question of whether selecting "Always for terminal" should uncheck all patterns (since it's broader), or if they should remain independent. The current behavior is: if any patterns are checked, Allow persists those patterns; if no patterns are checked, Allow uses the radio selection.

### The dropdown trigger label
When the user has toggled some checkboxes and changed the granularity, the dropdown trigger button label only reflects the granularity selection (e.g. "Only this time"), not the checked patterns. It might be helpful to show something like "2 commands selected" when patterns are checked.