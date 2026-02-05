# Tool Permissions Fix Plan

> **⚠️ This planning document must NEVER be committed or pushed. Delete it when all fixes are complete.**

## Work Instructions

All fixes below must be worked on **sequentially, not in parallel subagents**. After completing each numbered fix, **commit and push** before moving to the next one. This ensures clean, reviewable history and avoids conflicts between related changes.

Commit messages should follow the pattern: `Fix #<number>: <short description>`

---

## Fix #1: `copy_path`/`move_path` "Always Allow" generates patterns that never match

### Problem

Permission checking in `copy_path_tool.rs` (L86–93) and `move_path_tool.rs` (L100–107) calls `decide_permission_from_settings` against each path **independently** (source, then destination). But the `ToolPermissionContext.input_value` stores the combined `"source -> destination"` string (copy_path_tool.rs L106, move_path_tool.rs L121).

When the user clicks "Always Allow," `build_permission_options` in `thread.rs` (L687–690) calls `extract_path_pattern(input_value)` on the combined string. This produces a regex like `^src/a\.rs -> dest/` which is saved to settings. On the next invocation, `decide_permission_from_settings` evaluates this pattern against `"src/a.rs"` and `"dest/b.rs"` independently — neither contains ` -> `, so **the saved pattern never matches**.

### Fix

**In `thread.rs` (L673–690):** Change the `build_permission_options` logic for `copy_path` and `move_path` so that the pattern is extracted from only the **destination** path (which is the more security-relevant half). The `input_value` contains `"source -> dest"`, so split on `" -> "` and use the destination part for `extract_path_pattern`. This way the saved "Always Allow" pattern matches future operations targeting the same directory.

Concretely, add a branch for `CopyPathTool::NAME` and `MovePathTool::NAME` before the general path-tool branch:

```rust
} else if tool_name == CopyPathTool::NAME || tool_name == MovePathTool::NAME {
    // input_value is "source -> destination"; extract pattern from destination
    // because that's the more security-relevant path (where data is being written).
    // Both paths are checked independently by decide_permission_from_settings,
    // so a pattern matching the destination directory covers the common case.
    let dest = input_value.split_once(" -> ").map(|(_, d)| d).unwrap_or(input_value);
    (
        extract_path_pattern(dest),
        extract_path_pattern_display(dest),
    )
}
```

Keep the existing `EditFileTool::NAME || DeletePathTool::NAME || ...` branch but **remove** `CopyPathTool::NAME` and `MovePathTool::NAME` from it.

**No changes needed** to `copy_path_tool.rs` or `move_path_tool.rs` — the `ToolPermissionContext.input_value` can stay as the combined string for display purposes. The fix is entirely in how `build_permission_options` extracts the regex pattern.

### Files to change

- `crates/agent/src/thread.rs` — L680–690: split copy/move into their own branch

### Verification

- Manually test: trigger a `copy_path` confirmation, click "Always Allow for `<pattern>`", then re-run the same copy. It should auto-approve.
- Check that `move_path` behaves the same way.

---

## Fix #2: ACP `write_text_file` ignores `Confirm` decision

### Problem

In `acp.rs` (L1348–1356), the `write_text_file` method calls `check_acp_tool_permission("edit_file", &path_str, settings)` but only handles `Deny`. Both `Allow` and `Confirm` fall through to the write. Since the global default is `Confirm`, every ACP `write_text_file` call executes without any user prompt.

### Fix

After the existing `Deny` check, add handling for `Confirm`. Since `write_text_file` is an async trait method without access to the UI permission flow, the safest approach is to **treat `Confirm` the same as `Deny`** for direct ACP writes — return an error telling the ACP server to use `request_permission` instead. This matches the security-conservative design: if the user hasn't explicitly allowed a path, ACP servers should go through the interactive permission flow.

```rust
let decision = cx.update(|cx| {
    let settings = AgentSettings::get_global(cx);
    check_acp_tool_permission("edit_file", &path_str, settings)
});
match decision {
    AcpPermissionDecision::Deny(reason) => {
        return Err(anyhow!("{}", reason).into());
    }
    AcpPermissionDecision::Confirm => {
        return Err(anyhow!(
            "File write to '{}' requires confirmation. \
             Use request_permission to prompt the user first, \
             or configure an always_allow pattern for this path.",
            path_str
        ).into());
    }
    AcpPermissionDecision::Allow => {}
}
```

### Files to change

- `crates/agent_servers/src/acp.rs` — L1348–1356: replace `if let Deny` with `match` that handles all three variants

### Verification

- With default settings (`"confirm"`), an ACP `write_text_file` call should fail with the informative error.
- With `"default": "allow"`, it should succeed.
- With an `always_allow` pattern matching the path, it should succeed.

---

## Fix #5: Hardcoded `rm` security regexes lack word boundary

### Problem

In `tool_permissions.rs` (L26–44), all five hardcoded security regexes use `rm\s+` without a word boundary anchor. The pattern `rm\s+` matches `rm` appearing inside other words like `storm`, `inform`, `gorm`. For example, `storm -rf /tmp` would be falsely blocked because the regex finds `rm ` at position 3 inside `storm`.

### Fix

Add `\b` before `rm` in all five regex patterns. Change `r"rm\s+{FLAGS}..."` to `r"\brm\s+{FLAGS}..."` for each pattern. The `\b` asserts a word boundary, so `rm` only matches as a standalone word.

### Files to change

- `crates/agent/src/tool_permissions.rs` — L26–44: add `\b` before `rm` in all 5 `CompiledRegex::new` calls

### Verification

- Existing tests should still pass (they test `rm -rf /` etc. which still match with `\b`).
- Add a test case that `storm -rf /` is **not** blocked by the hardcoded rules.
- Add a test case that `inform -rf /` is **not** blocked.

---

## Fix #6: `from_input` doc comment falsely claims copy/move match combined strings

### Problem

The doc comment on `from_input` in `tool_permissions.rs` (L131–134) says:

> For `copy_path` and `move_path`, patterns are matched against the formatted string `source_path -> destination_path`.

This is wrong. `from_input` takes a single `input: &str` and is called separately for each path. The combined format only exists in `ToolPermissionContext.input_value` for UI display.

### Fix

Replace lines 131–134 with:

```
/// - For `copy_path` and `move_path`, the calling tool evaluates permissions against the
///   source and destination paths independently (two separate calls). A deny on either
///   path blocks the operation; a confirm on either path triggers a prompt.
```

### Files to change

- `crates/agent/src/tool_permissions.rs` — L131–134: fix doc comment

### Verification

- Read the updated doc and confirm it matches the actual behavior in `copy_path_tool.rs` L86–98 and `move_path_tool.rs` L100–112.

---

## Fix #7: `agent-settings.md` uses deprecated `default_mode` field name

### Problem

`docs/src/ai/agent-settings.md` uses `default_mode` in three places:
- L183: prose says "Each tool entry supports a `default_mode`"
- L191: JSON example uses `"default_mode": "allow"`
- L222: precedence list says "Tool-specific `default_mode`"

The canonical field name is `default` (the old name `default_mode` is only a serde alias for backward compat).

### Fix

1. L183: Change `default_mode` to `default`
2. L191: Change `"default_mode": "allow"` to `"default": "allow"`
3. L222: Change `default_mode` to `default`

### Files to change

- `docs/src/ai/agent-settings.md` — L183, L191, L222

### Verification

- Search the entire file for `default_mode` and confirm zero occurrences remain.

---

## Fix #8: `always_allow` description doesn't note terminal all-subcommands requirement

### Problem

`docs/src/ai/agent-settings.md` L221 says:

> If any allow pattern matches (and no deny/confirm matched), the tool call proceeds without prompting.

For the terminal tool with chained commands (`&&`, `||`, `;`), the code (tool_permissions.rs L284) requires **all** sub-commands to match an allow pattern. The docs describe a weaker guarantee.

### Fix

Amend L221 to:

> **`always_allow`** — If any allow pattern matches (and no deny/confirm matched), the tool call proceeds without prompting. For `terminal` commands with chaining (`&&`, `||`, `;`), **all** sub-commands must match an allow pattern.

### Files to change

- `docs/src/ai/agent-settings.md` — L221

### Verification

- Read the updated text and confirm it matches the logic in `check_commands` (tool_permissions.rs L271–286).

---

## Fix #9: MCP tool permission keys undocumented

### Problem

The settings content code (`settings_content/src/agent.rs` L558–560) and tests clearly support MCP tool names like `mcp:server_name:tool_name`, but `agent-settings.md` never mentions this capability.

### Fix

Add a paragraph and example after the `copy_path`/`move_path` patterns section (after ~L265), before the "Single-file Review" heading:

```markdown
#### MCP and External Tool Permissions

MCP tools can also have per-tool defaults using the key format `mcp:server_name:tool_name`:

\`\`\`json [settings]
{
  "agent": {
    "tool_permissions": {
      "tools": {
        "mcp:github:create_issue": {
          "default": "allow"
        },
        "mcp:filesystem:write_file": {
          "default": "deny"
        }
      }
    }
  }
}
\`\`\`

For MCP tools, only the `default` key is meaningful. Pattern-based rules (`always_allow`, `always_deny`, `always_confirm`) are evaluated against the tool call's title (which is set by the MCP server), not the raw tool input.
```

### Files to change

- `docs/src/ai/agent-settings.md` — insert new section before "Single-file Review"

### Verification

- Read the new section and confirm the key format matches what `check_acp_tool_permission` and the settings schema actually accept.

---

## Fix #10: Non-boolean `always_allow_tool_actions` silently removed during migration

### Problem

In `m_2026_02_04/settings.rs` (L66–72), `.as_bool()` returns `None` for non-boolean values like `"true"`, `1`, or `null`. The unconditional `.remove()` on L72 then deletes the key regardless. User intent is silently lost if they had a non-boolean value.

### Fix

Only remove the key when it's a boolean. If it's some other type, leave it in place — the settings schema will surface it as a validation error, which is better than silently losing data.

```rust
let should_migrate_always_allow = match agent_object.get("always_allow_tool_actions") {
    Some(Value::Bool(true)) => {
        agent_object.remove("always_allow_tool_actions");
        true
    }
    Some(Value::Bool(false)) | None => {
        agent_object.remove("always_allow_tool_actions");
        false
    }
    Some(_) => {
        // Non-boolean value — leave it in place so the schema validator
        // can report it, rather than silently dropping user data.
        false
    }
};
```

### Files to change

- `crates/migrator/src/migrations/m_2026_02_04/settings.rs` — L63–72

### Verification

- Add a test case: `"always_allow_tool_actions": "true"` (string) should be left in place, not removed.
- Add a test case: `"always_allow_tool_actions": null` should be removed (treated as false).
- Existing tests for `true` and `false` booleans should still pass.

---

## Fix #11: `bail!` in `m_2026_02_02` aborts entire migration chain

### Problem

In `m_2026_02_02/settings.rs` (L31–33), if `edit_predictions` exists but is not an object (e.g. `true`, `"string"`, `null`), the migration calls `bail!()`. The `?` on `migrator.rs` L91 propagates this error out of `run_migrations` entirely, preventing `m_2026_02_03` and `m_2026_02_04` from ever running. The user's tool permission settings remain unmigrated.

### Fix

Replace the `bail!` with a graceful early return, matching the defensive pattern used by `m_2026_02_04`:

```rust
let Some(edit_predictions_obj) = edit_predictions.as_object_mut() else {
    return Ok(());
};
```

This is safe because: if `edit_predictions` isn't an object, we can't insert a `provider` key into it anyway. Returning `Ok(())` lets subsequent migrations run normally.

### Files to change

- `crates/migrator/src/migrations/m_2026_02_02/settings.rs` — L31–33

### Verification

- Add a test case: settings with `"edit_predictions": true` and `"agent": { "always_allow_tool_actions": true }` — the `m_2026_02_02` migration should gracefully skip, and `m_2026_02_04` should still migrate the tool permissions.
- Existing migration tests should still pass.

---

## Fix #12: `m_2026_02_02` and `m_2026_02_03` don't handle platform/channel blocks

### Problem

`m_2026_02_04` correctly iterates `PLATFORM_AND_CHANNEL_KEYS` (`macos`, `linux`, `windows`, `dev`, `nightly`, `preview`, `stable`) and also handles `profiles`. But `m_2026_02_02` and `m_2026_02_03` only operate at root level. A user with settings like `{ "macos": { "features": { "edit_prediction_provider": "copilot" } } }` would not have that key migrated.

### Fix

**For `m_2026_02_02/settings.rs`:**

Extract the current body into a helper function `migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()>`. Then call it for:
1. The root object
2. Each platform/channel sub-object
3. Each profile sub-object

Use the same `PLATFORM_AND_CHANNEL_KEYS` constant (define it locally or import from a shared location — locally is fine since this is migration code that won't change).

**For `m_2026_02_03/settings.rs`:**

Same pattern: extract the core logic into a helper and apply it across root, platform/channel blocks, and profiles.

### Files to change

- `crates/migrator/src/migrations/m_2026_02_02/settings.rs`
- `crates/migrator/src/migrations/m_2026_02_03/settings.rs`

### Verification

- Add test cases for both migrations with settings nested inside platform keys (e.g. `"macos": { "features": { "edit_prediction_provider": "copilot" } }`).
- Add test cases with settings inside profiles.
- Existing tests should still pass.

---

## Fix #13: ACP `write_text_file` skips `.zed/` and config directory protections

### Problem

The built-in `authorize_file_edit` (`edit_file_tool.rs` L192–226) has special protections that force confirmation for:
- Paths containing `.zed/` (local settings folder) — L193–205
- Paths inside the global config directory — L210–222

The ACP `write_text_file` path (`acp.rs` L1341–1367) only calls `check_acp_tool_permission`, which has no awareness of these protected paths. An ACP agent can write to `.zed/settings.json` without any special prompting.

### Fix

After the `check_acp_tool_permission` check in `write_text_file` (which, after Fix #2, now handles `Deny` and `Confirm`), add checks for sensitive paths. Since the ACP code doesn't have access to the UI permission flow, treat writes to sensitive paths the same as `Confirm` — return an error directing the ACP server to use `request_permission`.

Add the `paths` crate as a dependency of `agent_servers` in `Cargo.toml`.

Then in `write_text_file`, after the existing permission check:

```rust
// Protect sensitive Zed-specific directories
let local_settings_folder = paths::local_settings_folder_name();
if arguments.path.components().any(|component| {
    component.as_os_str() == <_ as AsRef<std::ffi::OsStr>>::as_ref(&local_settings_folder)
}) {
    return Err(anyhow!(
        "File write to '{}' targets a local settings directory (.zed/). \
         Use request_permission to prompt the user first.",
        path_str
    ).into());
}

if let Ok(canonical_path) = std::fs::canonicalize(&arguments.path)
    && canonical_path.starts_with(paths::config_dir())
{
    return Err(anyhow!(
        "File write to '{}' targets the global config directory. \
         Use request_permission to prompt the user first.",
        path_str
    ).into());
}
```

### Files to change

- `crates/agent_servers/Cargo.toml` — add `paths.workspace = true` to `[dependencies]`
- `crates/agent_servers/src/acp.rs` — add `use paths;` import and add path checks in `write_text_file` after the permission decision handling

### Verification

- Confirm that an ACP `write_text_file` targeting `.zed/settings.json` returns an error.
- Confirm that an ACP `write_text_file` targeting a normal project file with `default: "allow"` still succeeds.

---

## Fix #14: ACP `check_acp_tool_permission` doesn't call hardcoded security rules

### Problem

`check_acp_tool_permission` (`acp.rs` L1155–1218) never calls `check_hardcoded_security_rules`. An ACP server providing a tool named `terminal` (or any tool matching the terminal tool name) would bypass the hardcoded `rm -rf /` protections.

The hardcoded rules are defined in the `agent` crate (`tool_permissions.rs`), which `agent_servers` doesn't depend on. The rules rely on `ShellKind` and `extract_commands` from the shell parser.

### Fix

Since the hardcoded security rules are tightly coupled to the agent crate's shell parser and tool names, and ACP tool names are unlikely to collide with `terminal` exactly, the pragmatic fix is to add a simple path-to-the-rules approach:

1. Move `HARDCODED_SECURITY_RULES`, `HardcodedSecurityRules`, `HARDCODED_SECURITY_DENIAL_MESSAGE`, and `check_hardcoded_security_rules` from `crates/agent/src/tool_permissions.rs` into `crates/agent_settings/src/agent_settings.rs` (which both `agent` and `agent_servers` already depend on). The function only needs `CompiledRegex` (already in `agent_settings`) and `ShellKind` (in `util`), plus the terminal tool name constant (pass as a parameter instead of importing).

2. In `check_acp_tool_permission`, call the moved function before the pattern-based checks:

```rust
if let Some(denial) = check_hardcoded_security_rules(tool_name, input, ShellKind::system()) {
    return AcpPermissionDecision::Deny(denial);
}
```

Note: `check_hardcoded_security_rules` currently returns `Option<ToolPermissionDecision>`, but `ToolPermissionDecision` is in the `agent` crate. The moved version should return `Option<String>` (the denial reason) instead, and callers wrap it in their respective decision types.

3. Update the callsite in `agent/src/tool_permissions.rs` to use the moved function.

### Files to change

- `crates/agent_settings/src/agent_settings.rs` — add `HardcodedSecurityRules`, `HARDCODED_SECURITY_RULES`, and `check_hardcoded_security_rules` (returning `Option<String>`)
- `crates/agent/src/tool_permissions.rs` — remove the moved items, import from `agent_settings`, update `check_hardcoded_security_rules` callsite to wrap result in `ToolPermissionDecision::Deny`
- `crates/agent_servers/src/acp.rs` — call `check_hardcoded_security_rules` in `check_acp_tool_permission`

### Verification

- Existing hardcoded security rule tests should still pass.
- Confirm that an ACP tool named `terminal` with input `rm -rf /` is blocked.

---

## Fix #16: `settings_content` doc comments about copy/move matching are wrong

### Problem

Three doc comments in `crates/settings_content/src/agent.rs` (L576, L584, L592) all say:

> For `copy_path` and `move_path`: matches against `source_path -> destination_path`.

This is wrong — patterns are matched against each path independently.

### Fix

Change all three lines to:

> For `copy_path` and `move_path`: matched independently against the source and destination paths.

### Files to change

- `crates/settings_content/src/agent.rs` — L576, L584, L592

### Verification

- Read the updated doc comments and confirm they match the actual behavior.

---

## Fix #17: ACP `request_permission` limitation (title-based matching) undocumented

### Problem

`acp.rs` L1275–1278 uses `arguments.tool_call.fields.title` for pattern matching since ACP doesn't expose raw tool input. Users have no way to know this limitation exists.

### Fix

Add a note to the MCP/external tools section added in Fix #9 (in `docs/src/ai/agent-settings.md`):

> **Note:** For external (MCP/ACP) tools, permission patterns are matched against the tool call's **title** as provided by the server, not the raw tool input. This means pattern matching for external tools is best-effort and depends on the server providing descriptive titles.

### Files to change

- `docs/src/ai/agent-settings.md` — append to the MCP section added in Fix #9

### Verification

- Read the updated docs and confirm the note is clear and accurate.

---

## Fix #19: Settings UI `regex_explanation` for copy/move needs update

### Problem

The settings UI `tool_permissions_setup.rs` (L38, L44) says:

> Patterns are matched against both the source and destination paths.

This is correct but could be clearer about how the single test input maps to dual-path matching. The placeholder text (L409) says "Enter a rule to see how it applies…" which is confusing — the user should enter a **path**, not a rule.

### Fix

1. Update `regex_explanation` for `copy_path` (L38) and `move_path` (L44) to:

```
"Patterns are matched independently against the source path and the destination path. Enter either path below to test."
```

2. Update the placeholder text (L409) to be more accurate. Change it to a tool-sensitive placeholder. For copy/move, it should say something like "Enter a source or destination path to test…". For terminal, "Enter a command to test…". For the general case, "Enter a tool input to test your rules…".

If making the placeholder tool-sensitive is too invasive, at minimum change the generic placeholder from `"Enter a rule to see how it applies…"` to `"Enter a tool input to test your rules…"`.

### Files to change

- `crates/settings_ui/src/pages/tool_permissions_setup.rs` — L38, L44 (regex_explanation), L409 (placeholder)

### Verification

- Open the settings UI for copy_path and confirm the explanation text makes sense.
- Confirm the placeholder text is not misleading.

---

## Fix #20: Remove commented-out dead code in `set_model`

### Problem

`crates/settings_content/src/agent.rs` L134–139 contains six lines of commented-out dead code. Per project guidelines: "Do not write organizational comments that summarize the code."

### Fix

Delete lines 134–139 (the commented-out code). Keep only the active line:

```rust
pub fn set_model(&mut self, language_model: LanguageModelSelection) {
    self.default_model = Some(language_model)
}
```

### Files to change

- `crates/settings_content/src/agent.rs` — L134–139: delete commented-out code

### Verification

- Confirm the function still compiles and works.