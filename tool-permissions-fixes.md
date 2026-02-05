# Tool Permissions: Fixes Plan

**You are responsible for addressing every item in this document.** Each section contains all the context you need—file paths, line numbers, code references, and the reasoning behind the fix. Line numbers are accurate as of the `always-allow-revision` branch after the latest merge from `origin/main`.

---

## Table of Contents

1. [Race Condition: `copy_path_tool` and `move_path_tool` Execute Before Authorization](#1-race-condition-copy_path_tool-and-move_path_tool-execute-before-authorization)
2. [Hardcoded Security Regex Bypassed by Extra Flags](#2-hardcoded-security-regex-bypassed-by-extra-flags)
3. [Hardcoded Security Regex Doesn't Catch `rm -rf /*`](#3-hardcoded-security-regex-doesnt-catch-rm--rf-)
4. [ACP `write_text_file` Bypasses All Permission Checks](#4-acp-write_text_file-bypasses-all-permission-checks)
5. [ACP `request_permission` Ignores Pattern Rules](#5-acp-request_permission-ignores-pattern-rules)
6. [Shell Parser: Redirect Targets Excluded from Normalized Command String](#6-shell-parser-redirect-targets-excluded-from-normalized-command-string)
7. [Settings UI Preview Diverges from Engine on Parse Failure](#7-settings-ui-preview-diverges-from-engine-on-parse-failure)
8. [Invalid Regex Patterns Silently Vanish from Settings UI](#8-invalid-regex-patterns-silently-vanish-from-settings-ui)
9. [`copy_path` and `move_path` Pass Only Source Path as Permission `input_value`](#9-copy_path-and-move_path-pass-only-source-path-as-permission-input_value)
10. [`StreamingEditFileTool` Is Missing Several Features vs `EditFileTool`](#10-streamingeditfiletool-is-missing-several-features-vs-editfiletool)
11. [Wrong Default Documented for `commit_message_model`](#11-wrong-default-documented-for-commit_message_model)
12. [Wrong Default Documented for `single_file_review`](#12-wrong-default-documented-for-single_file_review)
13. [`always_allow` and `always_confirm` Don't Document Cross-Layer Accumulation](#13-always_allow-and-always_confirm-dont-document-cross-layer-accumulation)
14. [Silent Error Discarding in Multiple Places](#14-silent-error-discarding-in-multiple-places)
15. [Test Harness Silently Drops Invalid Test Regexes](#15-test-harness-silently-drops-invalid-test-regexes)
16. [Significant Test Gaps](#16-significant-test-gaps)
17. [Settings UI: Computed Decision Never Displayed (Graceful Disagreement Recovery)](#17-settings-ui-computed-decision-never-displayed-graceful-disagreement-recovery)

---

## 1. Race Condition: `copy_path_tool` and `move_path_tool` Execute Before Authorization

### Problem

Both tools create their filesystem tasks **before** entering the async block where `authorize.await` runs. GPUI tasks begin executing immediately when created (not when first polled), so the file operation can complete before the user even sees the permission prompt. If the user denies, the damage is already done.

### Affected Files

- `crates/agent/src/tools/copy_path_tool.rs` — `run` method
- `crates/agent/src/tools/move_path_tool.rs` — `run` method

### Current Broken Pattern (copy_path_tool.rs)

In `copy_path_tool.rs`, the `authorize` future is created around L103–L113, and then `copy_task` is created at L115–L130 via `self.project.update(cx, |project, cx| { project.copy_entry(...) })`. This `copy_entry` call returns a `Task` that is **immediately scheduled** on the background thread pool. The `authorize.await` at L133–L135 happens inside a `cx.background_spawn` block, but by then the copy is already running.

The same pattern exists in `move_path_tool.rs`: `rename_task` is created at L125–L140 via `project.rename_entry(...)` before `authorize.await` at L143–L145.

### Correct Pattern (delete_path_tool.rs)

`delete_path_tool.rs` does this correctly. In its `run` method (L77–L165), the `authorize` future is created in a `match` block (L82–L96), and then in the `cx.spawn` async closure, `authorize.await` happens at L133–L135 **before** the deletion task is created at L147+. The filesystem operation only begins after authorization succeeds.

### Fix

For both `copy_path_tool.rs` and `move_path_tool.rs`:

1. Move the `self.project.update(cx, ...)` call that creates the filesystem task **inside** the async block (`cx.background_spawn` or `cx.spawn`), **after** the `authorize.await?` call.
2. Since `self.project` won't be available inside the async block, clone the project handle before entering the async block, then use `project.update(&mut cx, ...)` inside the async context (this requires changing to `cx.spawn` which provides an `AsyncApp`).
3. Follow the `delete_path_tool.rs` pattern as your template.

### Tests

Add tests that verify: when a user denies copy/move permission, the filesystem is unchanged. You can model these after the existing `test_delete_path_tool_deny_rule_blocks_deletion` test.

---

## 2. Hardcoded Security Regex Bypassed by Extra Flags

### Problem

The five hardcoded `rm` patterns in `HARDCODED_SECURITY_RULES` (tool_permissions.rs L18–L39) use the character class `(-[rf]+\s+)*` to match flags. This only matches the letters `r` and `f`. Any additional flag character causes the group to fail.

**Bypass examples (all are realistic LLM outputs):**
- `rm -rfv /` — the `v` in `-rfv` breaks `[rf]+`
- `rm -v -rf /` — `-v` is the first flag group, `v ∉ [rf]`
- `rm -rfi /` — the `i` breaks it
- `rm --recursive --force /` — long-form flags don't match at all

### Affected File

- `crates/agent/src/tool_permissions.rs` L18–L39

### Fix

Change the flag-matching group from `(-[rf]+\s+)*` to something that matches any single-dash flags. For example: `(-[a-zA-Z]+\s+)*`. This is consistent with the existing design (the patterns don't require both `-r` and `-f` to be present—`rm -r /` already matches).

Also consider adding a pattern for long-form flags: `rm\s+(--[a-z-]+\s+|(-[a-zA-Z]+\s+))*/\s*$` or similar. The key requirement is that `rm --recursive --force /` should be caught.

Apply this fix to all five `CompiledRegex::new` calls in the `terminal_deny` vec.

### Tests

Add test cases for:
- `rm -rfv /` → should be denied
- `rm -v -rf /` → should be denied
- `rm -rfi /` → should be denied
- `rm --recursive --force /` → should be denied
- `rm -rfv ~/somedir` → should NOT be denied (the path is a subdirectory, not root)

---

## 3. Hardcoded Security Regex Doesn't Catch `rm -rf /*`

### Problem

All five hardcoded patterns are `$`-anchored after the path portion (e.g., `rm\s+(-[rf]+\s+)*/\s*$`). The shell glob `rm -rf /*` expands to every top-level directory entry and is equally catastrophic to `rm -rf /`, but the `*` after `/` prevents `\s*$` from matching.

Same gap for `rm -rf ~/*` and `rm -rf $HOME/*`.

### Affected File

- `crates/agent/src/tool_permissions.rs` L18–L39

### Fix

For each of the five patterns, allow an optional glob `*` before the `$` anchor. For example, change:
- `r"rm\s+(-[rf]+\s+)*/\s*$"` → `r"rm\s+(-[a-zA-Z]+\s+)*/\*?\s*$"`
- `r"rm\s+(-[rf]+\s+)*~/?\s*$"` → `r"rm\s+(-[a-zA-Z]+\s+)*~/?(\*)?\s*$"`

And so on for the `$HOME`, `.`, and `..` variants.

**Note:** This fix should be combined with the fix from item #2 (the flag character class widening) since both touch the same patterns.

### Tests

Add test cases for:
- `rm -rf /*` → should be denied
- `rm -rf ~/*` → should be denied
- `rm -rf $HOME/*` → should be denied
- `rm -rf ./*` → should be denied
- `rm -rf ../*` → should be denied

---

## 4. ACP `write_text_file` Bypasses All Permission Checks

### Problem

The `write_text_file` method in `crates/agent_servers/src/acp.rs` (L1226–L1240) has zero permission checks. It calls directly through to `thread.write_text_file(path, content, cx)` with only a basic "is the path inside a worktree" check.

Compare with `EditFileTool::authorize()` (edit_file_tool.rs L158–L228), which:
- Checks `tool_permissions` deny rules via `decide_permission_from_settings`
- Detects `.zed` local settings paths and forces confirmation
- Detects global config dir paths and forces confirmation
- Checks whether the path is inside the project

An external ACP agent can currently:
- Write to `.zed/settings.json` without confirmation
- Write to any file matching a user's `always_deny` patterns without being blocked
- Write to files outside the project

### Affected File

- `crates/agent_servers/src/acp.rs` L1226–L1240

### Fix

Add permission checks to `write_text_file` that mirror what `EditFileTool::authorize()` does:

1. Look up the tool name in `tool_permissions` settings and call `decide_permission_from_settings` (or the equivalent ACP-level check).
2. If the decision is `Deny`, reject the write.
3. If the path contains the local settings folder name or is inside the global config dir, require confirmation.
4. If the path is outside all worktrees, require confirmation.

For the "require confirmation" cases in ACP context, you'll need to go through the existing `request_tool_call_authorization` flow, similar to how `request_permission` already works.

---

## 5. ACP `request_permission` Ignores Pattern Rules

### Problem

The `request_permission` handler in `crates/agent_servers/src/acp.rs` (L1144–L1224) only reads the `default` mode:

```
let effective_default = tool_name
    .and_then(|name| settings.tool_permissions.tools.get(name))
    .and_then(|rules| rules.default)
    .unwrap_or(settings.tool_permissions.default);
```

It never calls `ToolPermissionDecision::from_input()`, so `always_deny`, `always_allow`, and `always_confirm` regex patterns are completely ignored for ACP external agents.

A user who configures `always_deny: ["sensitive_pattern"]` for an external tool would expect it to block matching inputs, but it won't.

### Affected File

- `crates/agent_servers/src/acp.rs` L1144–L1224

### Fix

Instead of just checking `effective_default`, call the full `ToolPermissionDecision::from_input()` with the tool's input/arguments. The challenge is that ACP `request_permission` receives structured permission options rather than a raw input string. You'll need to:

1. Extract the relevant input string from the tool call arguments (the `arguments.tool_call` likely contains the input data in its fields).
2. Call `ToolPermissionDecision::from_input(tool_name, input_str, &settings.tool_permissions, shell_kind)`.
3. Handle the result: `Deny` → reject, `Allow` → auto-approve, `Confirm` → fall through to the existing UI prompt.

If extracting a meaningful input string isn't possible for all ACP tools, at minimum the `always_deny` and `always_allow` patterns with the tool's `default` should be respected.

---

## 6. Shell Parser: Redirect Targets Excluded from Normalized Command String

### Problem

In `crates/agent/src/shell_parser.rs`, the `extract_commands_from_simple_command` function (L81–L119) builds the normalized command string only from `Word` items in the command suffix (L98–L103). `IoRedirect` items are skipped entirely.

This means `echo hello > /etc/passwd` normalizes to just `echo hello`. A user's `always_deny: ["/etc/passwd"]` pattern for the terminal tool won't catch writes via redirects.

### Affected File

- `crates/agent/src/shell_parser.rs` L98–L103

### Fix

When iterating over `CommandPrefixOrSuffixItem` entries, also extract the target path from `IoRedirect` items and include it in the normalized command string. The redirect item contains a target word—normalize it the same way other words are normalized, and append it (along with the redirect operator like `>`, `>>`, etc.) to the command string.

For example, `echo hello > /etc/passwd` should normalize to something like `echo hello > /etc/passwd` so that patterns matching `/etc/passwd` will catch it.

### Tests

Add test cases for:
- `echo hello > /etc/passwd` — should include `/etc/passwd` in normalized output
- `cat file >> /tmp/log` — should include `/tmp/log`
- `cmd 2>&1` — file descriptor redirects should be handled gracefully (not crash)

---

## 7. Settings UI Preview Diverges from Engine on Parse Failure

### Problem

In `crates/settings_ui/src/pages/tool_permissions_setup.rs`, the `find_matched_patterns` function (L472–L533) handles terminal tool commands like this (around L483–L488):

```
let inputs_to_check: Vec<String> = if tool_id == TerminalTool::NAME {
    extract_commands(input).unwrap_or_else(|| vec![input.to_string()])
} else {
    vec![input.to_string()]
};
```

When `extract_commands` returns `None` (parse failure), the preview falls back to matching the raw input against **all** pattern types including `always_allow`. But the real engine in `from_input` (tool_permissions.rs around L200–L215) sets `allow_enabled = false` on parse failure, meaning `always_allow` patterns are ignored.

**Example:** User types `ls &&` (invalid syntax), has `^ls\b` as `always_allow`. The preview shows the allow pattern as matching. The real engine ignores it and returns Confirm.

### Affected File

- `crates/settings_ui/src/pages/tool_permissions_setup.rs` L472–L533

### Fix

`find_matched_patterns` needs to track whether parsing succeeded. When `extract_commands` returns `None` for a terminal tool command:
1. Set a flag like `allow_disabled = true`
2. Skip matching against `always_allow` patterns (or mark them as "overridden" in the returned data)
3. The returned `Vec<MatchedPattern>` should either exclude allow patterns or include them with a flag indicating they're disabled due to parse failure

The UI rendering code that consumes these matched patterns should then show a note like "Allow patterns disabled (command could not be parsed)" when this situation occurs.

---

## 8. Invalid Regex Patterns Silently Vanish from Settings UI

### Problem

The `ToolRulesView` struct (tool_permissions_setup.rs L825–L830) only contains successfully compiled patterns:

```
struct ToolRulesView {
    default: ToolPermissionMode,
    always_allow: Vec<String>,
    always_deny: Vec<String>,
    always_confirm: Vec<String>,
}
```

There is no field for `invalid_patterns`. The `ToolPermissions` struct has `invalid_patterns()` helpers (agent_settings.rs), but they're never called in the settings UI.

**User experience:** If a user types a malformed regex (e.g., `[bad(`), `save_pattern` writes it to `settings.json`. On the next settings reload, it gets moved to `invalid_patterns` during compilation. The UI re-renders showing only valid patterns—the bad one silently disappears. The user has no way to see, fix, or delete it through the UI.

### Affected Files

- `crates/settings_ui/src/pages/tool_permissions_setup.rs` L825–L863

### Fix

1. Add an `invalid_patterns: Vec<InvalidPatternView>` field to `ToolRulesView` (where `InvalidPatternView` contains the pattern string, rule type, and error message).
2. In `get_tool_rules` (L832–L863), populate this field from `rules.invalid_patterns`.
3. In the UI rendering, show invalid patterns with an error indicator (red highlight, error icon, or inline error message showing the compilation error).
4. Provide a way to edit or delete invalid patterns from the UI.

---

## 9. `copy_path` and `move_path` Pass Only Source Path as Permission `input_value`

### Problem

Both `copy_path_tool.rs` and `move_path_tool.rs` create their `ToolPermissionContext` with only the source path:

```
let context = crate::ToolPermissionContext {
    tool_name: "copy_path".to_string(),
    input_value: input.source_path.clone(),
};
```

(copy_path_tool.rs L106–L109, move_path_tool.rs L116–L119)

The destination path is never included. This means:
- Permission patterns that should protect certain destination paths won't match
- The "Always allow for `<pattern>`" button generates a pattern from the source path only, which is misleading

### Affected Files — Code

- `crates/agent/src/tools/copy_path_tool.rs` L106–L109
- `crates/agent/src/tools/move_path_tool.rs` L116–L119
- `crates/agent/src/thread.rs` — the `ToolPermissionContext` struct and `build_permission_options` method
- `crates/agent/src/tool_permissions.rs` — `from_input` and `decide_permission_from_settings`

### Affected Files — Code Documentation

Review and update doc comments in:
- `crates/agent/src/tool_permissions.rs` — the doc comment on `from_input` (L91–L127) which describes how pattern matching works. It says "For file tools: matches path" but doesn't clarify what happens for dual-path tools.
- `crates/agent/src/thread.rs` — the `ToolPermissionContext` struct doc comments
- `crates/settings_content/src/agent.rs` — the doc comments on `always_allow`, `always_deny`, `always_confirm` fields (L575–L587). These say "For file tools: matches path" which is ambiguous for copy/move.

### Affected Files — User-Facing Documentation

- `docs/src/ai/agent-settings.md` — update the tool permissions section to explain how copy/move paths are matched
- `docs/src/ai/agent-panel.md` — if it references tool permission patterns, update there too
- `docs/src/ai/mcp.md` — if it references tool permission patterns, update there too

### Affected Files — Settings UI

- `crates/settings_ui/src/pages/tool_permissions_setup.rs` — the test input section for `copy_path` and `move_path` tools needs to accept two path inputs (source and destination) and show pattern matching against both.

### Fix

**Code changes:**

1. Change `ToolPermissionContext::input_value` to accommodate multiple values. Options:
   - Make it a `Vec<String>` and update all consumers
   - Or: for dual-path tools, join source and destination with a separator (e.g., `"{source} -> {destination}"`) and document this format

2. For `copy_path_tool` and `move_path_tool`, run `decide_permission_from_settings` against **both** paths (you already do this for deny checks—the source and destination are checked independently at the top of `run`). The `ToolPermissionContext` should include both paths so the "Always allow" button can generate appropriate patterns.

3. Update `build_permission_options` in thread.rs to handle the dual-path case. The pattern extraction and display should show both paths.

**Documentation changes:**

4. Update the `from_input` doc comment to explicitly state: "For `copy_path` and `move_path`, patterns are matched against both the source and destination paths independently. A deny match on either path blocks the operation."

5. Update `settings_content/src/agent.rs` doc comments on `always_allow`/`always_deny`/`always_confirm` to say: "For `copy_path` and `move_path`: matches against both source and destination paths."

6. Update `docs/src/ai/agent-settings.md` to document the dual-path behavior.

7. Update the settings UI test input for copy/move tools to accept two paths and show matching for both.

---

## 10. `StreamingEditFileTool` Is Missing Several Features vs `EditFileTool`

### Problem

Although currently disabled (`use_streaming_edit_tool = false` in thread.rs), `StreamingEditFileTool` is missing several features that `EditFileTool` has. These will become bugs when the feature is enabled.

| Feature | `EditFileTool` | `StreamingEditFileTool` |
|---|---|---|
| Format on save | ✅ L478–L502 | ❌ Skips directly to `project.save_buffer` at L439 |
| User cancellation | ✅ `futures::select!` with `cancelled_by_user()` in `run` | ❌ No cancellation check anywhere |
| `rebind_thread` for subagents | ✅ L585–L590 | ❌ Returns `None` (default) |
| Overlapping edit detection | N/A | ❌ `apply_edits` (L488–L593) doesn't validate ranges |
| Duplicated `authorize()` logic | L158–L228 | L166–L228 (nearly identical copy) |

### Affected File

- `crates/agent/src/tools/streaming_edit_file_tool.rs`

### Fix

1. **Format on save:** After applying edits and before saving, check `format_on_save` settings and call `project.format(...)` the same way `EditFileTool::run` does at L478–L502.

2. **User cancellation:** Add a `futures::select!` race between the edit stream processing and `event_stream.cancelled_by_user()`, similar to `EditFileTool::run`.

3. **`rebind_thread`:** Implement `rebind_thread` following the same pattern as `EditFileTool::rebind_thread` (L585–L590).

4. **Overlapping edit detection:** In `apply_edits` (L488–L593), after resolving all edits in the first pass and sorting them, validate that no two resolved ranges overlap before applying them. If overlaps are detected, return an error.

5. **Deduplicate `authorize`:** Extract the shared `authorize` logic into a standalone function that both `EditFileTool` and `StreamingEditFileTool` call. This prevents security fixes to one from being missed in the other.

---

## 11. Wrong Default Documented for `commit_message_model`

### Problem

In `crates/settings_content/src/agent.rs` L52–L55:

```rust
/// Model to use for generating git commit messages.
///
/// Default: true
pub commit_message_model: Option<LanguageModelSelection>,
```

The field is `Option<LanguageModelSelection>`, not a bool. `"Default: true"` was copy-pasted from the `inline_assistant_use_streaming_tools` field directly above it.

### Fix

Change the doc comment to match `thread_summary_model` (L56):

```rust
/// Model to use for generating git commit messages. Defaults to default_model when not specified.
pub commit_message_model: Option<LanguageModelSelection>,
```

---

## 12. Wrong Default Documented for `single_file_review`

### Problem

In `docs/src/ai/agent-settings.md` L183–L184:

```
Control whether to display review actions (accept & reject) in single buffers after the agent is done performing edits.
The default value is `false`.
```

Three other sources say the default is `true`:
- `assets/settings/default.json`: `"single_file_review": true,`
- `crates/settings_content/src/agent.rs` L79–L81: doc comment says `/// Default: true`
- The example in the same markdown file (L190) shows it set to `true`

### Fix

Change line 184 of `docs/src/ai/agent-settings.md` from:

```
The default value is `false`.
```

to:

```
The default value is `true`.
```

---

## 13. `always_allow` and `always_confirm` Don't Document Cross-Layer Accumulation

### Problem

`always_deny` in `crates/settings_content/src/agent.rs` (L580–L583) explicitly warns about cross-layer behavior:

```rust
/// Regexes for inputs to auto-reject.
/// **SECURITY**: These take precedence over ALL other rules, across ALL settings layers.
```

But `always_allow` (L575–L578) and `always_confirm` (L585–L587) use the same `ExtendingVec` type and thus also accumulate across settings layers, yet their doc comments don't mention this. A user reading these comments would have no idea that project-level patterns **add to** (not replace) user-level patterns.

### Fix

Update the doc comments for `always_allow` and `always_confirm` to mention accumulation:

```rust
/// Regexes for inputs to auto-approve.
/// For terminal: matches command. For file tools: matches path. For fetch: matches URL.
/// Patterns accumulate across settings layers (user, project, profile) and cannot be
/// removed by a higher-priority layer—only new patterns can be added.
/// Default: []
pub always_allow: Option<ExtendingVec<ToolRegexRule>>,
```

```rust
/// Regexes for inputs that must always prompt.
/// Takes precedence over always_allow but not always_deny.
/// Patterns accumulate across settings layers (user, project, profile) and cannot be
/// removed by a higher-priority layer—only new patterns can be added.
/// Default: []
pub always_confirm: Option<ExtendingVec<ToolRegexRule>>,
```

Also update `always_deny` to use the same "accumulate across settings layers" language for consistency.

---

## 14. Silent Error Discarding in Multiple Places

### Problem

Per the project rules: "Never silently discard errors with `let _ =` on fallible operations. Always handle errors appropriately."

Multiple places in the codebase violate this:

### `crates/agent/src/thread.rs`

- **`cx.update(...)` results silently dropped** in `authorize_third_party_tool` (around L3142, L3155) and `authorize` (around L3216, L3226, L3248, L3265). These `cx.update()` calls return `Result` on `AsyncApp` but the result is ignored.

- **`.ok()` on `unbounded_send`** at various points in the authorization flow (around L2873, L2879, L2885, L2905, L2930, L2933, etc.). If the receiver is dropped, the send fails silently and the spawned task hangs forever.

### `crates/zed/src/visual_test_runner.rs`

Numerous `.ok()` and `let _ =` usages throughout the file (see lines listed in the table of contents research). These are in test infrastructure, so the severity is lower, but they should still use `.log_err()` for visibility.

### Fix

- Replace `.ok()` with `.log_err()` on all fallible operations where the error should be visible but not fatal.
- Replace `let _ = expr` with `expr.log_err()` where the expression returns a `Result`.
- For `unbounded_send` failures specifically, use `.log_err()` so that if the channel is disconnected, there's a log entry explaining why a task might hang.

---

## 15. Test Harness Silently Drops Invalid Test Regexes

### Problem

In `crates/agent/src/tool_permissions.rs`, the `PermTest::run` method (around L428–L461) uses `filter_map` when compiling test regex patterns:

```rust
always_allow: self
    .allow
    .iter()
    .filter_map(|(p, cs)| CompiledRegex::new(p, *cs))
    .collect(),
```

`CompiledRegex::new` returns `Option`, and `filter_map` silently discards `None`. If a test accidentally provides a malformed regex string, the pattern is silently dropped rather than causing the test to fail. The test would then pass or fail for the wrong reason.

### Fix

Change `filter_map` to `map` with an `expect` or `unwrap` (acceptable in test code):

```rust
always_allow: self
    .allow
    .iter()
    .map(|(p, cs)| CompiledRegex::new(p, *cs)
        .unwrap_or_else(|| panic!("Test regex failed to compile: {}", p)))
    .collect(),
```

Apply this to all three pattern lists (`always_allow`, `always_deny`, `always_confirm`) in the `run` method.

---

## 16. Significant Test Gaps

### Problem

Several important code paths have no test coverage.

### Tests to Add

1. **`always_deny:` response in `authorize`** (thread.rs around L3221–L3231): The `test_tool_authorization` test (tests/mod.rs L743–L879) sends `"allow"`, `"deny"`, and `"always_allow:tool_requiring_permission"`, but never `"always_deny:tool_requiring_permission"`. Add a test case that exercises this path.

2. **`always_allow_pattern:` / `always_deny_pattern:` response parsing** (thread.rs around L3237–L3275): These handlers use `splitn(3, ':')` to parse the response ID. Zero integration test coverage. Add tests with valid and malformed response IDs.

3. **MCP tool authorization flow** (thread.rs L3071–L3172): The `test_mcp_tools` test (tests/mod.rs L1208) bypasses authorization by setting `"default": "allow"`. Add a test that exercises the actual `authorize_third_party_tool` flow with `"default": "confirm"`.

4. **Settings persistence via `update_settings_file`**: `ToolCallEventStream::test()` always passes `fs: None`, so every `if let Some(fs) = fs` guard in `authorize` evaluates to false. Add at least one test with a real `FakeFs` that verifies the "Always allow" button actually persists the setting.

5. **`create_directory_tool` permission rules**: Every other permission-using tool has a deny-rule integration test. `CreateDirectoryTool` is missing one. Add `test_create_directory_tool_deny_rule_blocks_creation`.

6. **Hardcoded rule bypass variants**: Add unit tests for `rm -rfv /`, `rm -v -rf /`, `rm -rf /*`, `rm --recursive --force /` (these will initially fail and then pass after fixes #2 and #3).

7. **`always_confirm` patterns on non-terminal tools**: Only tested end-to-end for the terminal tool. Add integration tests verifying `always_confirm` works for `edit_file`, `fetch`, `delete_path`, etc.

---

## 17. Settings UI: Computed Decision Never Displayed (Graceful Disagreement Recovery)

### Problem

In `crates/settings_ui/src/pages/tool_permissions_setup.rs`, the test input section computes two things (around L413–L419):
1. `decision` — the authoritative `ToolPermissionDecision` from `evaluate_test_input` (which calls the real `from_input` engine)
2. `matched_patterns` — the list of patterns that matched, from `find_matched_patterns` (a separate, simpler implementation used only for display)

Currently, `decision` is only used as an `is_some()` gate (L449). The actual verdict (Allow/Deny/Confirm) is never rendered. The user sees which patterns matched but never the final authoritative verdict.

This is dangerous because `find_matched_patterns` can disagree with `evaluate_test_input` (see issue #7 for one example: parse failure causes the preview to show allow patterns as active while the real engine ignores them). When they disagree, the user sees a misleading preview.

### Affected Files

- `crates/settings_ui/src/pages/tool_permissions_setup.rs` L411–L461

### Fix — Graceful Disagreement Recovery

The goal is: if `matched_patterns` and `decision` agree, show today's UI unchanged. If they **disagree**, override the display with the authoritative verdict and log an error.

**Step 1: Determine what `matched_patterns` implies.**

After computing `matched_patterns`, derive what verdict the matched patterns imply:
- If any deny pattern matched → implied verdict is `Deny`
- Else if any confirm pattern matched → implied verdict is `Confirm`
- Else if any allow pattern matched (and allow is enabled) → implied verdict is `Allow`
- Else → implied verdict is the default mode (from `get_tool_rules`)

**Step 2: Compare implied verdict with `decision`.**

Map the `ToolPermissionDecision` to the same categories:
- `ToolPermissionDecision::Allow` → `Allow`
- `ToolPermissionDecision::Deny(_)` → `Deny`
- `ToolPermissionDecision::Confirm` → `Confirm`

**Step 3: If they agree, render today's UI unchanged.**

No changes to the current rendering path when the implied verdict matches the authoritative `decision`.

**Step 4: If they disagree, override the display.**

When the implied verdict does **not** match the authoritative `decision`:

1. Log an error to the console: `log::error!("Tool permission preview disagreement for {tool_id}: preview implies {implied:?} but engine says {decision:?}")`. This gives developers visibility into the mismatch.

2. Instead of rendering the `matched_patterns` list, render a single entry showing only the final authoritative verdict:
   - Use a distinct visual style (e.g., a warning-colored label) so it's clear this is an override
   - Show text like: **"Result: Deny"** or **"Result: Allow"** or **"Result: Confirm"**
   - Do **not** show a reason or matched pattern list, since the pattern list is what was wrong
   - Optionally show a small note like "(pattern preview unavailable)" to explain why the usual detail is missing

3. Do **not** show the `matched_patterns` list at all when in disagreement mode. The whole point is to prevent the user from seeing inaccurate pattern-match information.

**Step 5: Also show the verdict when they agree (but keep the pattern list).**

As a bonus improvement (not strictly required for the disagreement recovery, but improves UX): when they agree, show a small verdict badge/label alongside the matched patterns, e.g., "Result: Confirm" at the bottom of the pattern list. This gives users the authoritative answer they're looking for when testing inputs.

### Implementation Notes

- The `evaluate_test_input` function (L577–L588) already calls the real `ToolPermissionDecision::from_input`. You don't need to change the engine—just use its result.
- The comparison logic should account for the fact that `Deny` carries a message string. Compare only the variant, not the message.
- Be careful with the "implied verdict" derivation: it must account for `allow_enabled` being false on parse failure (this is the main source of disagreement today). You may want to track this in the `find_matched_patterns` return value.
- The hardcoded security rules (e.g., `rm -rf /`) are another source of disagreement: `evaluate_test_input` catches them but `find_matched_patterns` doesn't check them. The implied verdict derivation should fall through to "default" for these cases, which will disagree with the `Deny` from the engine, triggering the override display correctly.