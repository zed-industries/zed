# Migrating `always_allow_tool_actions` to `tool_permissions.default_mode`

This document describes how to implement the settings migration from `always_allow_tool_actions` to a new `default_mode` field within `tool_permissions`.

## Background

### Current State

The agent settings currently have two related but separate mechanisms for tool permission control:

1. **`always_allow_tool_actions`** (boolean) - A global override that, when `true`, bypasses all tool permission checks (except hardcoded security rules like blocking `rm -rf /`).

2. **`tool_permissions`** - Granular per-tool permission rules with:
   - `tools.<tool_name>.default_mode` - Per-tool default (`allow`, `deny`, `confirm`)
   - `tools.<tool_name>.always_allow` - Regex patterns to auto-approve
   - `tools.<tool_name>.always_deny` - Regex patterns to auto-reject
   - `tools.<tool_name>.always_confirm` - Regex patterns requiring confirmation

```json
{
  "agent": {
    "always_allow_tool_actions": true,
    "tool_permissions": {
      "tools": {
        "terminal": {
          "default_mode": "confirm",
          "always_deny": [{ "pattern": "rm\\s+-rf" }]
        }
      }
    }
  }
}
```

### Problem

The current design has issues:

1. **Footgun**: Users can carefully configure granular permissions, not realizing `always_allow_tool_actions: true` overrides everything (except hardcoded security rules).
2. **All-or-nothing YOLO mode**: You can't say "allow everything by default, but still respect my `always_confirm` patterns for `sudo`."
3. **Redundant concepts**: Two separate ways to express "allow by default" is confusing.

### Proposed State

Move the global default into `tool_permissions.default_mode`:

```json
{
  "agent": {
    "tool_permissions": {
      "default_mode": "allow",
      "tools": {
        "terminal": {
          "default_mode": "confirm",
          "always_confirm": [{ "pattern": "sudo" }]
        }
      }
    }
  }
}
```

Benefits:
- Simpler mental model
- No security footgun
- Can layer guardrails on top of a permissive baseline
- Cleaner codebase and Settings UI

## Implementation Guide

### Step 1: Create the Migration Module

Create a new migration module at `crates/migrator/src/migrations/m_YYYY_MM_DD/settings.rs` (use the date you're implementing this).

The migration should:
1. Check if `agent.always_allow_tool_actions` exists and is `true`
2. If so, set `agent.tool_permissions.default_mode` to `"allow"`
3. Remove `agent.always_allow_tool_actions`

```rust
// crates/migrator/src/migrations/m_YYYY_MM_DD/settings.rs

use anyhow::Result;
use serde_json::Value;

pub fn migrate_always_allow_tool_actions_to_default_mode(value: &mut Value) -> Result<()> {
    let Some(obj) = value.as_object_mut() else {
        return Ok(());
    };

    let Some(agent) = obj.get_mut("agent") else {
        return Ok(());
    };

    let Some(agent_obj) = agent.as_object_mut() else {
        return Ok(());
    };

    // Check if always_allow_tool_actions exists and is true
    let should_migrate = agent_obj
        .get("always_allow_tool_actions")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !should_migrate {
        // If it's false or not present, just remove it if present (cleanup)
        agent_obj.remove("always_allow_tool_actions");
        return Ok(());
    }

    // Remove the old setting
    agent_obj.remove("always_allow_tool_actions");

    // Get or create tool_permissions
    let tool_permissions = agent_obj
        .entry("tool_permissions")
        .or_insert_with(|| Value::Object(Default::default()));

    let Some(tool_permissions_obj) = tool_permissions.as_object_mut() else {
        anyhow::bail!("Expected tool_permissions to be an object");
    };

    // Only set default_mode if it's not already set
    if !tool_permissions_obj.contains_key("default_mode") {
        tool_permissions_obj.insert(
            "default_mode".to_string(),
            Value::String("allow".to_string()),
        );
    }

    Ok(())
}
```

### Step 2: Register the Migration

Update `crates/migrator/src/migrations.rs` to expose your new migration:

```rust
// Add at the end of the file:

pub(crate) mod m_YYYY_MM_DD {
    mod settings;

    pub(crate) use settings::migrate_always_allow_tool_actions_to_default_mode;
}
```

### Step 3: Add Migration to the Runner

Update `crates/migrator/src/migrator.rs` in the `migrate_settings` function. Add your migration at the end of the `migrations` array:

```rust
pub fn migrate_settings(text: &str) -> Result<Option<String>> {
    let migrations: &[MigrationType] = &[
        // ... existing migrations ...
        MigrationType::Json(migrations::m_2026_02_03::migrate_experimental_sweep_mercury),
        // Add your new migration here:
        MigrationType::Json(
            migrations::m_YYYY_MM_DD::migrate_always_allow_tool_actions_to_default_mode,
        ),
    ];
    run_migrations(text, migrations)
}
```

### Step 4: Add Tests

Add tests in `crates/migrator/src/migrator.rs` in the `mod tests` section:

```rust
#[test]
fn test_migrate_always_allow_tool_actions_to_default_mode() {
    // Case 1: No agent settings - no change
    assert_migrate_settings_with_migrations(
        &[MigrationType::Json(
            migrations::m_YYYY_MM_DD::migrate_always_allow_tool_actions_to_default_mode,
        )],
        &r#"{ }"#.unindent(),
        None,
    );

    // Case 2: always_allow_tool_actions: true -> tool_permissions.default_mode: "allow"
    assert_migrate_settings_with_migrations(
        &[MigrationType::Json(
            migrations::m_YYYY_MM_DD::migrate_always_allow_tool_actions_to_default_mode,
        )],
        &r#"
        {
            "agent": {
                "always_allow_tool_actions": true
            }
        }
        "#
        .unindent(),
        Some(
            &r#"
            {
                "agent": {
                    "tool_permissions": {
                        "default_mode": "allow"
                    }
                }
            }
            "#
            .unindent(),
        ),
    );

    // Case 3: always_allow_tool_actions: false -> just remove it
    assert_migrate_settings_with_migrations(
        &[MigrationType::Json(
            migrations::m_YYYY_MM_DD::migrate_always_allow_tool_actions_to_default_mode,
        )],
        &r#"
        {
            "agent": {
                "always_allow_tool_actions": false
            }
        }
        "#
        .unindent(),
        Some(
            &r#"
            {
                "agent": {}
            }
            "#
            .unindent(),
        ),
    );

    // Case 4: Preserve existing tool_permissions.tools when migrating
    assert_migrate_settings_with_migrations(
        &[MigrationType::Json(
            migrations::m_YYYY_MM_DD::migrate_always_allow_tool_actions_to_default_mode,
        )],
        &r#"
        {
            "agent": {
                "always_allow_tool_actions": true,
                "tool_permissions": {
                    "tools": {
                        "terminal": {
                            "always_deny": [{ "pattern": "rm\\s+-rf" }]
                        }
                    }
                }
            }
        }
        "#
        .unindent(),
        Some(
            &r#"
            {
                "agent": {
                    "tool_permissions": {
                        "default_mode": "allow",
                        "tools": {
                            "terminal": {
                                "always_deny": [{ "pattern": "rm\\s+-rf" }]
                            }
                        }
                    }
                }
            }
            "#
            .unindent(),
        ),
    );

    // Case 5: Don't override existing default_mode
    assert_migrate_settings_with_migrations(
        &[MigrationType::Json(
            migrations::m_YYYY_MM_DD::migrate_always_allow_tool_actions_to_default_mode,
        )],
        &r#"
        {
            "agent": {
                "always_allow_tool_actions": true,
                "tool_permissions": {
                    "default_mode": "confirm"
                }
            }
        }
        "#
        .unindent(),
        Some(
            &r#"
            {
                "agent": {
                    "tool_permissions": {
                        "default_mode": "confirm"
                    }
                }
            }
            "#
            .unindent(),
        ),
    );

    // Case 6: No migration needed if already using new format
    assert_migrate_settings_with_migrations(
        &[MigrationType::Json(
            migrations::m_YYYY_MM_DD::migrate_always_allow_tool_actions_to_default_mode,
        )],
        &r#"
        {
            "agent": {
                "tool_permissions": {
                    "default_mode": "allow"
                }
            }
        }
        "#
        .unindent(),
        None,
    );
}
```

### Step 5: Update Settings Content Structs

In `crates/settings_content/src/agent.rs`:

1. **Remove** the `always_allow_tool_actions` field from `AgentSettingsContent`
2. **Add** `default_mode` to `ToolPermissionsContent`

```rust
// In ToolPermissionsContent, add:
#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ToolPermissionsContent {
    /// Global default mode when no tool-specific rules match.
    /// Individual tools can override this with their own default_mode.
    /// Default: confirm
    pub default_mode: Option<ToolPermissionMode>,

    /// Per-tool permission rules.
    /// Keys: terminal, edit_file, delete_path, move_path, create_directory,
    ///       save_file, fetch, web_search
    #[serde(default)]
    pub tools: HashMap<Arc<str>, ToolRulesContent>,
}
```

### Step 6: Update Agent Settings Structs

In `crates/agent_settings/src/agent_settings.rs`:

1. **Remove** `always_allow_tool_actions` from `AgentSettings`
2. **Add** `default_mode` to `ToolPermissions`

```rust
#[derive(Clone, Debug, Default)]
pub struct ToolPermissions {
    /// Global default mode when no tool-specific rules or patterns match.
    pub default_mode: ToolPermissionMode,
    pub tools: collections::HashMap<Arc<str>, ToolRules>,
}
```

### Step 7: Update Permission Decision Logic

In `crates/agent/src/tool_permissions.rs`, update `ToolPermissionDecision::from_input`:

```rust
pub fn from_input(
    tool_name: &str,
    input: &str,
    permissions: &ToolPermissions,
    shell_kind: ShellKind,
) -> ToolPermissionDecision {
    // First, check hardcoded security rules
    if let Some(denial) = check_hardcoded_security_rules(tool_name, input, shell_kind) {
        return denial;
    }

    let rules = permissions.tools.get(tool_name);
    
    // Get the effective default mode:
    // 1. Tool-specific default_mode if configured
    // 2. Otherwise, global default_mode from tool_permissions
    let effective_default = rules
        .map(|r| r.default_mode)
        .unwrap_or(permissions.default_mode);

    // ... rest of pattern matching logic, using effective_default as fallback
}
```

Also update `decide_permission_from_settings` to no longer pass `always_allow_tool_actions`:

```rust
pub fn decide_permission_from_settings(
    tool_name: &str,
    input: &str,
    cx: &App,
) -> ToolPermissionDecision {
    let settings = AgentSettings::get_global(cx);
    ToolPermissionDecision::from_input(
        tool_name,
        input,
        &settings.tool_permissions,
        ShellKind::system(),
    )
}
```

### Step 7b: Update ACP Thread Authorization

In `crates/acp_thread/src/acp_thread.rs`, update `request_tool_call_authorization`:

The current code checks `always_allow_tool_actions` directly:

```rust
// BEFORE
pub fn request_tool_call_authorization(
    &mut self,
    tool_call: acp::ToolCallUpdate,
    options: PermissionOptions,
    respect_always_allow_setting: bool,
    cx: &mut Context<Self>,
) -> Result<BoxFuture<'static, acp::RequestPermissionOutcome>> {
    let (tx, rx) = oneshot::channel();

    if respect_always_allow_setting && AgentSettings::get_global(cx).always_allow_tool_actions {
        // Don't use AllowAlways, because then if you were to turn off always_allow_tool_actions,
        // some tools would (incorrectly) continue to auto-accept.
        if let Some(allow_once_option) = options.allow_once_option_id() {
            self.upsert_tool_call_inner(tool_call, ToolCallStatus::Pending, cx)?;
            return Ok(async {
                acp::RequestPermissionOutcome::Selected(acp::SelectedPermissionOutcome::new(
                    allow_once_option,
                ))
            }
            .boxed());
        }
    }
    // ...
}
```

Update to use `tool_permissions.default_mode`:

```rust
// AFTER
pub fn request_tool_call_authorization(
    &mut self,
    tool_call: acp::ToolCallUpdate,
    options: PermissionOptions,
    respect_default_mode_setting: bool,
    cx: &mut Context<Self>,
) -> Result<BoxFuture<'static, acp::RequestPermissionOutcome>> {
    let (tx, rx) = oneshot::channel();

    let settings = AgentSettings::get_global(cx);
    let global_default = settings.tool_permissions.default_mode;
    
    if respect_default_mode_setting && global_default == ToolPermissionMode::Allow {
        // Don't use AllowAlways, because then if you were to change default_mode,
        // some tools would (incorrectly) continue to auto-accept.
        if let Some(allow_once_option) = options.allow_once_option_id() {
            self.upsert_tool_call_inner(tool_call, ToolCallStatus::Pending, cx)?;
            return Ok(async {
                acp::RequestPermissionOutcome::Selected(acp::SelectedPermissionOutcome::new(
                    allow_once_option,
                ))
            }
            .boxed());
        }
    }
    // ...
}
```

Note: You'll also need to update all call sites of `request_tool_call_authorization` if you rename the parameter. Search for usages:

```bash
grep -r "request_tool_call_authorization" crates/
```

### Step 8: Update Default Settings

In `assets/settings/default.json`, update the agent section:

```json
{
  "agent": {
    "tool_permissions": {
      // Global default for all tools when no patterns match.
      // Individual tools can override this.
      // "allow" - Auto-approve without prompting
      // "deny" - Auto-reject
      // "confirm" - Always prompt (default)
      "default_mode": "confirm",
      "tools": {
        // Tool-specific rules here...
      }
    }
  }
}
```

### Step 9: Update Tests

Search for all uses of `always_allow_tool_actions` in tests and update them:

```bash
grep -r "always_allow_tool_actions" crates/
```

Key files to update:
- `crates/agent/src/tests/mod.rs`
- `crates/agent/src/tool_permissions.rs` (tests)
- `crates/agent/src/tools/edit_file_tool.rs` (tests)
- `crates/agent/src/tools/save_file_tool.rs` (tests)
- `crates/acp_thread/src/acp_thread.rs`
- `crates/agent_ui/src/agent_ui.rs` (tests)
- `crates/eval/runner_settings.json`

For test helper functions like `always_allow_tools()`, update to use the new structure:

```rust
fn always_allow_tools(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.default_mode = ToolPermissionMode::Allow;
        agent_settings::AgentSettings::override_global(settings, cx);
    });
}
```

### Step 10: Update UI Code

Update any Settings UI code that references `always_allow_tool_actions`. Search for:

```bash
grep -r "always_allow_tool_actions" crates/settings_ui/
grep -r "always_allow_tool_actions" crates/agent_ui/
```

The UI should now show:
- A dropdown/selector for `tool_permissions.default_mode` (Allow/Deny/Confirm)
- Remove any warning about `always_allow_tool_actions` overriding granular permissions

Key file: `crates/settings_ui/src/page_data.rs` - update the `agent_configuration_section` function.

### Step 11: Update Documentation

Update the documentation files that reference `always_allow_tool_actions`:

- `docs/src/ai/agent-panel.md` - Update the Tool Approval section
- `docs/src/ai/agent-settings.md` - Update the Auto-run Commands section  
- `docs/src/ai/mcp.md` - Update the Tool Approval section

Example documentation update:

```markdown
<!-- Before -->
Zed's Agent Panel surfaces the `agent.always_allow_tool_actions` setting that, 
if turned to `false`, will require you to give permission...

<!-- After -->
Zed's Agent Panel provides the `agent.tool_permissions.default_mode` setting to control
tool approval behavior. Set it to `"allow"` to auto-approve all tool actions, `"confirm"` 
(the default) to require approval, or `"deny"` to block all tool actions. You can also
configure per-tool rules under `agent.tool_permissions.tools`.
```

### Step 12: Update Eval Runner Settings

Update `crates/eval/runner_settings.json`:

```json
// Before
{
  "agent": {
    "always_allow_tool_actions": true
  }
}

// After
{
  "agent": {
    "tool_permissions": {
      "default_mode": "allow"
    }
  }
}
```

## Migration Behavior Summary

| Before | After |
|--------|-------|
| `"always_allow_tool_actions": true` | `"tool_permissions": { "default_mode": "allow" }` |
| `"always_allow_tool_actions": false` | (field removed, default is "confirm") |
| No setting | No change needed |

## Precedence Order After Migration

The new precedence order (highest to lowest):

1. **Hardcoded security rules** - Built-in rules like blocking `rm -rf /` (cannot be bypassed)
2. **`always_deny` patterns** - Per-tool deny patterns
3. **`always_confirm` patterns** - Per-tool confirm patterns  
4. **`always_allow` patterns** - Per-tool allow patterns
5. **Tool-specific `default_mode`** - Per-tool default
6. **Global `default_mode`** - `tool_permissions.default_mode`

This is cleaner than before because there's no separate global override that can silently bypass everything.

## Checklist

### Migration Infrastructure
- [ ] Create migration module `m_YYYY_MM_DD/settings.rs`
- [ ] Register migration in `migrations.rs`
- [ ] Add migration to `migrate_settings()` in `migrator.rs`
- [ ] Add migration tests in `migrator.rs`

### Settings Structs
- [ ] Add `default_mode` to `ToolPermissionsContent` in `settings_content/src/agent.rs`
- [ ] Add `default_mode` to `ToolPermissions` in `agent_settings/src/agent_settings.rs`
- [ ] Remove `always_allow_tool_actions` from `AgentSettingsContent`
- [ ] Remove `always_allow_tool_actions` from `AgentSettings`
- [ ] Remove `set_always_allow_tool_actions` method from `AgentSettingsContent`
- [ ] Update `Settings::from_settings` impl to handle new structure

### Permission Logic
- [ ] Update `ToolPermissionDecision::from_input()` signature (remove `always_allow_tool_actions` param)
- [ ] Update `ToolPermissionDecision::from_input()` logic to use `permissions.default_mode`
- [ ] Update `decide_permission_from_settings()` call signature
- [ ] Update `crates/acp_thread/src/acp_thread.rs` - `request_tool_call_authorization`

### Default Settings
- [ ] Update `assets/settings/default.json` - add `default_mode` to `tool_permissions`
- [ ] Update `crates/eval/runner_settings.json`

### Tests
- [ ] Update `crates/agent/src/tests/mod.rs`
- [ ] Update `crates/agent/src/tool_permissions.rs` tests
- [ ] Update `crates/agent/src/tools/edit_file_tool.rs` tests
- [ ] Update `crates/agent/src/tools/save_file_tool.rs` tests
- [ ] Update `crates/agent_ui/src/agent_ui.rs` tests
- [ ] Update `crates/agent_settings/src/agent_settings.rs` tests

### UI
- [ ] Update `crates/settings_ui/src/page_data.rs` - `agent_configuration_section`

### Documentation
- [ ] Update `docs/src/ai/agent-panel.md`
- [ ] Update `docs/src/ai/agent-settings.md`
- [ ] Update `docs/src/ai/mcp.md`

### Validation
- [ ] Run `./script/clippy` and fix any issues
- [ ] Run the full test suite
- [ ] Manually test settings migration with a real `settings.json`

## Edge Cases and Common Patterns

### Edge Case 1: Both Settings Present

If a user has both `always_allow_tool_actions: true` AND `tool_permissions.default_mode` set:

```json
{
  "agent": {
    "always_allow_tool_actions": true,
    "tool_permissions": {
      "default_mode": "confirm"
    }
  }
}
```

The migration should **preserve** the existing `default_mode` (don't override it). The user explicitly set `default_mode`, so respect that. Just remove `always_allow_tool_actions`.

### Edge Case 2: Empty tool_permissions Object

```json
{
  "agent": {
    "always_allow_tool_actions": true,
    "tool_permissions": {}
  }
}
```

Should become:

```json
{
  "agent": {
    "tool_permissions": {
      "default_mode": "allow"
    }
  }
}
```

### Edge Case 3: tool_permissions with Only tools

```json
{
  "agent": {
    "always_allow_tool_actions": true,
    "tool_permissions": {
      "tools": {
        "terminal": { "always_deny": [{ "pattern": "rm" }] }
      }
    }
  }
}
```

Should become:

```json
{
  "agent": {
    "tool_permissions": {
      "default_mode": "allow",
      "tools": {
        "terminal": { "always_deny": [{ "pattern": "rm" }] }
      }
    }
  }
}
```

### Edge Case 4: always_allow_tool_actions: false (Explicit)

When explicitly set to `false`, just remove the field (the default behavior is "confirm" anyway):

```json
// Before
{ "agent": { "always_allow_tool_actions": false } }

// After  
{ "agent": {} }
```

### Common Test Pattern Updates

When updating tests, look for patterns like:

```rust
// OLD PATTERN
settings.always_allow_tool_actions = true;

// NEW PATTERN
settings.tool_permissions.default_mode = ToolPermissionMode::Allow;
```

And in tool permission decision tests:

```rust
// OLD PATTERN
ToolPermissionDecision::from_input(tool_name, input, &permissions, true, shell_kind)

// NEW PATTERN (no boolean parameter)
ToolPermissionDecision::from_input(tool_name, input, &permissions, shell_kind)
// where permissions.default_mode is set appropriately
```

## Behavioral Differences

### Before Migration

With `always_allow_tool_actions: true`:
- **ALL** tool actions auto-approved (except hardcoded security rules)
- `always_deny` patterns in `tool_permissions` are **ignored**
- `always_confirm` patterns are **ignored**
- Per-tool `default_mode: deny` is **ignored**

### After Migration

With `tool_permissions.default_mode: "allow"`:
- Tool actions follow the new precedence order
- `always_deny` patterns **still block** actions
- `always_confirm` patterns **still require confirmation**
- Per-tool `default_mode` **can override** the global default

This is intentionally different! Users who want the old "ignore everything" behavior can achieve it by:
1. Setting `default_mode: "allow"`
2. Not configuring any `always_deny` or `always_confirm` patterns

Users who want "allow by default but block dangerous commands" can now do:
```json
{
  "agent": {
    "tool_permissions": {
      "default_mode": "allow",
      "tools": {
        "terminal": {
          "always_confirm": [{ "pattern": "sudo" }],
          "always_deny": [{ "pattern": "rm\\s+-rf\\s+/" }]
        }
      }
    }
  }
}
```

This was impossible before because `always_allow_tool_actions` bypassed everything.

## Technical Notes: How JSON Migrations Work

The migrator crate supports two types of migrations:

### TreeSitter Migrations

Used for pattern-based text transformations. Good for renaming keys, changing values, etc. These work by:
1. Parsing the JSON with TreeSitter
2. Running queries to find matching patterns
3. Applying text replacements at specific byte ranges

### JSON Migrations (Recommended for This Migration)

Used for structural changes. This is what we use here because we need to:
- Remove a key from one location
- Add/modify a key in a nested object

JSON migrations work by:
1. Parsing the settings file into a `serde_json::Value`
2. Calling your callback function with a mutable reference to the value
3. Comparing old vs new value
4. Using `update_value_in_json_text` to generate minimal edits that preserve formatting

```rust
// The callback signature for JSON migrations:
fn my_migration(value: &mut serde_json::Value) -> anyhow::Result<()>
```

Key points:
- Return `Ok(())` even if no changes were made - the framework detects changes by comparing values
- The migration runs on the entire settings file, so navigate to `value["agent"]` first
- Use `as_object_mut()` to safely get mutable access to JSON objects
- Use `.entry().or_insert_with()` to create nested objects if they don't exist
- The framework handles preserving comments and formatting

### Migration Order

Migrations are run in the order they appear in the `migrations` array in `migrate_settings()`. Each migration sees the result of all previous migrations. Always append new migrations at the end of the array.

### Testing Migrations

Use `assert_migrate_settings_with_migrations` to test individual migrations in isolation:

```rust
assert_migrate_settings_with_migrations(
    &[MigrationType::Json(my_migration_fn)],
    &input_json.unindent(),
    Some(&expected_output.unindent()),  // or None if no change expected
);
```

The test helper:
- Compares the actual output against expected
- Handles JSON formatting differences  
- Verifies that running the migration twice produces the same result (idempotency)